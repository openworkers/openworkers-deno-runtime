use deno_core::Extension;
use deno_core::ExtensionFileSource;

deno_core::extension!(
    runtime,
    deps = [deno_console, deno_web, deno_crypto, deno_fetch, fetch_event, scheduled_event],
    customizer = |ext: &mut Extension| {
        ext.esm_files.to_mut().push(ExtensionFileSource::new("ext:runtime.js", include_str!("runtime.js")));
        ext.esm_entry_point = Some("ext:runtime.js");
    }
);
