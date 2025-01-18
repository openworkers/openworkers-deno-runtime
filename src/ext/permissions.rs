use deno_core::url::Url;
use deno_permissions::PermissionCheckError;
use deno_permissions::PermissionDeniedError;
use std::borrow::Cow;
use std::path::Path;

#[derive(Clone)]
pub struct Permissions {}

impl deno_web::TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

impl deno_fetch::FetchPermissions for Permissions {
    fn check_net_url(&mut self, _url: &Url, _api_name: &str) -> Result<(), PermissionCheckError> {
        println!("TODO check_net_url {:?}", _url); // TODO

        Ok(()) // TODO
    }

    fn check_read<'a>(
        &mut self,
        _p: &'a Path,
        _api_name: &str,
    ) -> Result<Cow<'a, Path>, PermissionCheckError> {
        println!("TODO check_read {:?}", _p.display()); // TODO

        Err(PermissionCheckError::PermissionDenied(
            PermissionDeniedError::Fatal {
                access: "Not allowed".to_string(),
            },
        ))
    }
}

deno_core::extension!(
    permissions,
    state = |state| state.put::<Permissions>(Permissions {})
);
