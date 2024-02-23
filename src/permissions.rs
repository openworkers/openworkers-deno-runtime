use deno_core::error::AnyError;
use deno_core::url::Url;
use std::path::Path;

#[derive(Clone)]
pub struct Permissions {}

impl Permissions {
    pub fn new() -> Self {
        Self {}
    }
}

impl deno_web::TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

impl deno_fetch::FetchPermissions for Permissions {
    fn check_net_url(&mut self, _url: &Url, _api_name: &str) -> Result<(), AnyError> {
        Ok(()) // TODO
    }

    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), AnyError> {
        Ok(()) // TODO
    }
}

deno_core::extension!(
    permissions,
    state = |state| state.put::<Permissions>(Permissions::new())
);
