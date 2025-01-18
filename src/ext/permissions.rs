#[derive(Clone)]
pub struct Permissions {}

impl deno_web::TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

deno_core::extension!(
    permissions,
    state = |state| state.put::<Permissions>(Permissions {})
);
