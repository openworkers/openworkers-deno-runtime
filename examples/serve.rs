use bytes::Bytes;

use log::debug;
use log::error;
use openworkers_runtime::AnyError;
use openworkers_runtime::FetchInit;
use openworkers_runtime::Task;
use openworkers_runtime::Url;
use openworkers_runtime::Worker;

use tokio::sync::oneshot;

use actix_web::{App, HttpServer};

use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

struct AppState {
    url: Url,
}

async fn handle_request(data: Data<AppState>, req: HttpRequest) -> HttpResponse {
    debug!("handle_request {} {}", req.method(), req.uri());

    let start = tokio::time::Instant::now();

    let url = data.url.clone();
    let url_clone = url.clone();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<Option<AnyError>>();
    let (response_tx, response_rx) = oneshot::channel::<http_v02::Response<Bytes>>();

    let task = Task::Fetch(Some(FetchInit {
        res_tx: Some(response_tx),
        req: http_v02::Request::builder()
            .uri(req.uri())
            .body(Default::default())
            .unwrap(),
    }));

    Worker::new(url, shutdown_tx).exec(task);

    let url = url_clone.clone();

    debug!("js worker for {:?} started", url);

    // wait for shutdown signal
    match shutdown_rx.await {
        Ok(None) => debug!("js worker for {:?} stopped", url),
        Ok(Some(err)) => {
            error!("js worker for {:?} error: {}", url, err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
        Err(err) => {
            error!("js worker for {:?} error: {}", url, err);
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
            .app_data(Data::new({
                let path = get_path();
                let url = openworkers_runtime::module_url(path.as_str());
                AppState { url }
            }))
            .default_service(web::to(handle_request))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
