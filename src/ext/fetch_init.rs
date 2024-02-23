deno_core::extension!(
    fetch_init,
    deps = [deno_console, deno_fetch],
    ops = [op_fetch_init],
);

use std::rc::Rc;

use deno_core::error::bad_resource_id;
use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::serde::Serialize;
use deno_core::serde::Deserialize;
use deno_core::OpState;
use deno_core::ResourceId;
// use deno_fetch::FetchRequestResource;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FetchResource {
    pub req: String
}

impl deno_core::Resource for FetchResource {
    fn close(self: Rc<Self>) {
        println!("TcpStreamResource.close()");
    }
}

impl std::fmt::Display for FetchResource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FetchResource req: {}", self.req)
    }
}

#[op2]
#[serde]
fn op_fetch_init(state: &mut OpState) -> Result<FetchResource, AnyError> {
    println!("op_fetch_init empty");

    let req: FetchResource = state.take::<FetchResource>();

    Ok(req)
}
