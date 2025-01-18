deno_core::extension!(
    ow_fetch,
    deps = [deno_console],
    esm = [
        "ext:ow_fetch/20_headers.js" = "src/ext/fetch/20_headers.js",
        "ext:ow_fetch/22_body.js" = "src/ext/fetch/22_body.js",
        "ext:ow_fetch/23_request.js" = "src/ext/fetch/23_request.js",
        "ext:ow_fetch/23_response.js" = "src/ext/fetch/23_response.js",
    ]
);
