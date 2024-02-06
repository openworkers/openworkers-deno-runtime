use deno_core::Extension;
use deno_core::ExtensionFileSource;
use deno_core::ExtensionFileSourceCode;

deno_core::extension!(
  runtime,
  deps = [ deno_console, deno_crypto ],
  customizer = |ext: &mut Extension| {
    ext.esm_files.to_mut().push(ExtensionFileSource {
      specifier: "ext:runtime.js",
      code: ExtensionFileSourceCode::IncludedInBinary(
        include_str!("runtime.js")
      ),
    });
    ext.esm_entry_point = Some("ext:runtime.js");
  }
);
