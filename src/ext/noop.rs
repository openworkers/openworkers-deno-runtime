// Noop implementation of various files
deno_core::extension!(
    noop_ext,
    esm = [
        "ext:deno_telemetry/telemetry.ts" = {
            source = r#"
                export const TRACING_ENABLED = false;
                export const METRICS_ENABLED = false;

                export const builtinTracer = () => {};
                export const enterSpan = () => {};
                export const restoreContext = () => {};
            "#
        },
        "ext:deno_telemetry/util.ts" = {
            source = r#"
                export const updateSpanFromResponse = () => {};
                export const updateSpanFromRequest = () => {};
            "#
        },
        "ext:deno_fetch/22_http_client.js" = {
            source = r#"
                export const HttpClientPrototype = (class HttpClient {}).prototype;
                export const createHttpClient = () => {};
            "#
        },
    ]
);
