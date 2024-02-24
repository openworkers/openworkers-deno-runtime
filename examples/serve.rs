use bytes::Bytes;

use log::debug;
use log::error;
use openworkers_runtime::run_js;
use openworkers_runtime::AnyError;
use openworkers_runtime::FetchInit;
use openworkers_runtime::Task;

use tokio::sync::oneshot;

use actix_web::{App, HttpServer};

use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

struct AppState {
    path: String,
}

async fn handle_request(data: Data<AppState>, req: HttpRequest) -> HttpResponse {
    debug!("handle_request {} {}", req.method(), req.uri());

    let start = tokio::time::Instant::now();

    let file_path = data.path.clone();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<Option<AnyError>>();
    let (response_tx, response_rx) = oneshot::channel::<http_v02::Response<Bytes>>();

    let _res = {
        let file_path = file_path.clone();

        let task = Task::Fetch(FetchInit {
            req: http_v02::Request::builder()
                .uri(req.uri())
                .body(Default::default())
                .unwrap(),
            res_tx: Some(response_tx),
        });

        std::thread::spawn(move || run_js(file_path.as_str(), task, shutdown_tx))
    };

    debug!("js worker for {:?} started", file_path);

    // wait for shutdown signal
    match shutdown_rx.await {
        Ok(None) => debug!("js worker for {:?} stopped", file_path),
        Ok(Some(err)) => {
            error!("js worker for {:?} error: {}", file_path, err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
        Err(err) => {
            error!("js worker for {:?} error: {}", file_path, err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }

    match response_rx.await {
        Ok(res) => {
            debug!(
                "worker fetch replied {} {:?}",
                res.status(),
                start.elapsed()
            );

            let mut rb = HttpResponse::build(res.status());

            for (k, v) in res.headers() {
                rb.append_header((k, v));
            }

            rb.body(res.body().clone())
        }
        Err(err) => {
            error!("worker fetch error: {}, ensure the worker registered a listener for the 'fetch' event", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

fn get_path() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("examples/serve.js"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    debug!("start main");

    // Check that the path is correct
    {
        let path = get_path();
        if !std::path::Path::new(&path).is_file() {
            eprintln!("file not found: {}", path);
            std::process::exit(1);
        }
    }

    println!("Listening on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(AppState { path: get_path() }))
            .default_service(web::to(handle_request))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
