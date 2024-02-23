deno_core::extension!(
    fetch_init,
    deps = [deno_console, deno_fetch],
    ops = [op_fetch_init, op_fetch_respond],
);

use std::rc::Rc;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::serde::Deserialize;
use deno_core::serde::Serialize;
use deno_core::OpState;
use deno_core::ResourceId;
use deno_fetch::reqwest::Request as HttpRequest;
use deno_fetch::reqwest::Response as HttpResponse;
use log::debug;
use log::info;

type ResponseSender = tokio::sync::oneshot::Sender<()>;

#[derive(Debug)]
pub struct HttpResponseTx {
    tx: ResponseSender,
}

impl From<ResponseSender> for HttpResponseTx {
    fn from(tx: ResponseSender) -> Self {
        HttpResponseTx { tx }
    }
}

impl HttpResponseTx {
    pub fn send(self, res: ()) -> Result<(), ()> {
        self.tx.send(res)
    }
}

#[derive(Debug)]
pub struct FetchInit {
    pub req: HttpRequest,
    pub res_tx: Option<ResponseSender>,
}


#[derive(Debug, Serialize, Deserialize)]
struct InnerRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FetchEvent {
    req: InnerRequest,
    rid: u32,
}

impl From<HttpRequest> for InnerRequest {
    fn from(req: HttpRequest) -> Self {
        InnerRequest {
            method: req.method().to_string(),
            url: req.url().to_string(),
            headers: req
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                .collect(),
            body: None,
        }
    }
}

impl deno_core::Resource for FetchInit {
    fn close(self: Rc<Self>) {
        println!("TcpStreamResource.close()");
    }
}

impl deno_core::Resource for HttpResponseTx {
    fn close(self: Rc<Self>) {
        println!("TcpStreamResource.close()"); // TODO
    }
}

#[op2]
#[serde]
fn op_fetch_init(state: &mut OpState) -> Result<FetchEvent, AnyError> {
    debug!("op_fetch_init");

    let mut evt: FetchInit = state.take::<FetchInit>();

    let req = InnerRequest::from(evt.req);

    let res = HttpResponseTx::from(evt.res_tx.take().unwrap()); 

    let rid = state.resource_table.add::<HttpResponseTx>(res);

    Ok(FetchEvent { req , rid })
}

#[op2]
#[serde]
fn op_fetch_respond(state: &mut OpState, #[smi] response_id: ResourceId) -> Result<(), AnyError> {
    debug!("op_fetch_respond response_id: {}", response_id);

    let tx = match state.resource_table.take::<HttpResponseTx>(response_id) {
        Ok(tx) => tx,
        Err(err) => return Err(err),
    };

    let tx = Rc::try_unwrap(tx).unwrap();

    let tx = tx.send(());
    
    Ok(())
}
