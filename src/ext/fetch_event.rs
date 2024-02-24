use std::rc::Rc;

use bytes::Bytes;
use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::serde::Deserialize;
use deno_core::serde::Serialize;
use deno_core::OpState;
use deno_core::ResourceId;
use log::debug;

type HttpRequest = http_v02::Request<Bytes>;
type HttpResponse = http_v02::Response<Bytes>;
type ResponseSender = tokio::sync::oneshot::Sender<HttpResponse>;

#[derive(Debug)]
struct HttpResponseTx {
    tx: ResponseSender,
}

/// FetchResponse is a struct that represents the response
/// from a fetch request that comes from js realm.
#[derive(Debug, Deserialize)]
pub struct FetchResponse {
    status: u16,

    #[serde(rename = "headerList")]
    headers: Vec<(String, String)>,

    body: Option<Bytes>,
}

impl Into<HttpResponse> for FetchResponse {
    fn into(self) -> HttpResponse {
        let mut builder = http_v02::Response::builder().status(self.status);


        for (k, v) in self.headers {
            builder = builder.header(k, v);
        }

        match self.body {
            Some(body) => builder.body(body).unwrap(),
            None => builder.body(Default::default()).unwrap(),
        }
    }
}

impl From<ResponseSender> for HttpResponseTx {
    fn from(tx: ResponseSender) -> Self {
        HttpResponseTx { tx }
    }
}

impl HttpResponseTx {
    pub fn send(self, res: FetchResponse) -> Result<(), HttpResponse> {
        self.tx.send(res.into())
    }
}

#[derive(Debug)]
pub struct FetchInit {
    pub req: HttpRequest,
    pub res_tx: Option<ResponseSender>
}

#[derive(Debug, Serialize)]
struct InnerRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Debug, Serialize)]
struct FetchEvent {
    req: InnerRequest,
    rid: u32,
}

impl From<HttpRequest> for InnerRequest {
    fn from(req: HttpRequest) -> Self {
        InnerRequest {
            method: req.method().to_string(),
            url: req.uri().to_string(),
            headers: req
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                .collect(),
            body: None,
        }
    }
}

deno_core::extension!(
    fetch_init,
    deps = [deno_console, deno_fetch],
    ops = [op_fetch_init, op_fetch_respond],
);

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

    Ok(FetchEvent { req, rid })
}

#[op2]
#[serde]
fn op_fetch_respond(
    state: &mut OpState,
    #[smi] rid: ResourceId,
    #[serde] res: FetchResponse,
) -> Result<(), AnyError> {
    debug!("op_fetch_respond with status {}", res.status);

    let tx = match state.resource_table.take::<HttpResponseTx>(rid) {
        Ok(tx) => tx,
        Err(err) => return Err(err),
    };

    let tx = Rc::try_unwrap(tx).unwrap();

    let tx = tx.send(res);
    debug!("op_fetch_respond tx {:?}", tx);

    Ok(())
}
