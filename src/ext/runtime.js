// runtime.js

// deno_core
import { core, primordials } from "ext:core/mod.js";
import { op_fetch_init, op_fetch_respond } from "ext:core/ops";

// deno_webidl
import * as webidl from "ext:deno_webidl/00_webidl.js";

// deno_console
import * as console from "ext:deno_console/01_console.js";

// deno_web
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import * as event from "ext:deno_web/02_event.js";
import * as timers from "ext:deno_web/02_timers.js";
import * as abortSignal from "ext:deno_web/03_abort_signal.js";
import {} from "ext:deno_web/04_global_interfaces.js";
import * as base64 from "ext:deno_web/05_base64.js";
import * as streams from "ext:deno_web/06_streams.js";
import * as encoding from "ext:deno_web/08_text_encoding.js";
import * as file from "ext:deno_web/09_file.js";
import * as fileReader from "ext:deno_web/10_filereader.js";
import * as location from "ext:deno_web/12_location.js";
import * as messagePort from "ext:deno_web/13_message_port.js";
import * as compression from "ext:deno_web/14_compression.js";
import * as performance from "ext:deno_web/15_performance.js";

// deno_url
import * as url from "ext:deno_url/00_url.js";
import * as urlPattern from "ext:deno_url/01_urlpattern.js";

// deno_crypto
import * as crypto from "ext:deno_crypto/00_crypto.js";

// deno_fetch
import * as headers from "ext:deno_fetch/20_headers.js";
import * as formData from "ext:deno_fetch/21_formdata.js";
import * as request from "ext:deno_fetch/23_request.js";
import * as response from "ext:deno_fetch/23_response.js";
import * as fetch from "ext:deno_fetch/26_fetch.js";
import * as eventSource from "ext:deno_fetch/27_eventsource.js";

{
  core.print(`Will setup runtime.js\n`);

  const { ObjectDefineProperties, ObjectDefineProperty, SymbolFor } =
    primordials;

  class WorkerNavigator {
    constructor() {
      webidl.illegalConstructor();
    }

    [SymbolFor("Deno.privateCustomInspect")](inspect) {
      return `${this.constructor.name} ${inspect({})}`;
    }
  }

  const workerNavigator = webidl.createBranded(WorkerNavigator);

  let numCpus, userAgent, language;

  // https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator
  ObjectDefineProperties(WorkerNavigator.prototype, {
    hardwareConcurrency: {
      configurable: true,
      enumerable: true,
      get() {
        webidl.assertBranded(this, WorkerNavigatorPrototype);
        return numCpus;
      },
    },
    userAgent: {
      configurable: true,
      enumerable: true,
      get() {
        webidl.assertBranded(this, WorkerNavigatorPrototype);
        return userAgent;
      },
    },
    language: {
      configurable: true,
      enumerable: true,
      get() {
        webidl.assertBranded(this, WorkerNavigatorPrototype);
        return language;
      },
    },
    languages: {
      configurable: true,
      enumerable: true,
      get() {
        webidl.assertBranded(this, WorkerNavigatorPrototype);
        return [language];
      },
    },
  });

  const WorkerNavigatorPrototype = WorkerNavigator.prototype;

  function nonEnumerable(value) {
    return {
      value,
      writable: true,
      enumerable: false,
      configurable: true,
    };
  }

  function writable(value) {
    return {
      value,
      writable: true,
      enumerable: true,
      configurable: true,
    };
  }

  function readOnly(value) {
    return {
      value,
      enumerable: true,
      writable: false,
      configurable: true,
    };
  }

  function getterOnly(getter) {
    return {
      get: getter,
      set() {},
      enumerable: true,
      configurable: true,
    };
  }

  class AssertionError extends Error {
    /** @param msg {string} */
    constructor(msg) {
      super(msg);
      this.name = "AssertionError";
    }
  }

  function newFetchEvent(request, respondWith) {
    return {
      request,
      respondWith,
    };
  }

  function handleFetchRequest(respondWith) {
    core.print("handleFetchRequest called\n");

    const evt = op_fetch_init();

    const rid = evt.rid;

    const signal = abortSignal.newSignal();

    const inner = request.newInnerRequest(
      evt.req.method,
      evt.req.url,
      () => evt.req.headers,
      evt.req.body
    );

    const guard = headers.guardFromHeaders(
      headers.headersFromHeaderList(inner.headerList)
    );

    const req = request.fromInnerRequest(inner, signal, guard);

    return respondWith(
      newFetchEvent(req, async (resOrPromise) => {
        core.print("respondWith called\n");

        let res = core.isPromise(resOrPromise)
          ? await resOrPromise
          : resOrPromise;

        const inner = response.toInnerResponse(res);

        const body = await res.arrayBuffer();

        core.print("respondWith body.len " + body?.length + "\n");
        core.print("respondWith consumed " + JSON.stringify(body) + "\n");

        op_fetch_respond(rid, { ...inner, body });
      })
    );
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope
  const windowOrWorkerGlobalScope = {
    console: nonEnumerable(
      // https://choubey.gitbook.io/internals-of-deno/bridge/4.2-print
      new console.Console((msg, level) => core.print(msg, level > 1))
    ),

    // DOM Exception
    // deno_web - 01 - dom_exception
    DOMException: nonEnumerable(DOMException),
    AssertionError: nonEnumerable(AssertionError),

    // Timers
    // deno_web - 02 - timers
    clearInterval: writable(timers.clearInterval),
    clearTimeout: writable(timers.clearTimeout),
    setInterval: writable(timers.setInterval),
    setTimeout: writable(timers.setTimeout),

    // Abort signal
    // deno_web - 03 - abort_signal
    AbortController: nonEnumerable(abortSignal.AbortController),
    AbortSignal: nonEnumerable(abortSignal.AbortSignal),

    // Base64
    // deno_web - 05 - base64
    atob: writable(base64.atob),
    btoa: writable(base64.btoa),

    // Streams
    // deno_web - 06 - streams

    // streams
    ByteLengthQueuingStrategy: nonEnumerable(streams.ByteLengthQueuingStrategy),
    CountQueuingStrategy: nonEnumerable(streams.CountQueuingStrategy),
    ReadableStream: nonEnumerable(streams.ReadableStream),
    ReadableStreamDefaultReader: nonEnumerable(
      streams.ReadableStreamDefaultReader
    ),
    ReadableByteStreamController: nonEnumerable(
      streams.ReadableByteStreamController
    ),
    ReadableStreamBYOBReader: nonEnumerable(streams.ReadableStreamBYOBReader),
    ReadableStreamBYOBRequest: nonEnumerable(streams.ReadableStreamBYOBRequest),
    ReadableStreamDefaultController: nonEnumerable(
      streams.ReadableStreamDefaultController
    ),
    TransformStream: nonEnumerable(streams.TransformStream),
    TransformStreamDefaultController: nonEnumerable(
      streams.TransformStreamDefaultController
    ),
    WritableStream: nonEnumerable(streams.WritableStream),
    WritableStreamDefaultWriter: nonEnumerable(
      streams.WritableStreamDefaultWriter
    ),
    WritableStreamDefaultController: nonEnumerable(
      streams.WritableStreamDefaultController
    ),

    // Text Encoding
    // deno_web - 08 - text_encoding
    TextDecoder: nonEnumerable(encoding.TextDecoder),
    TextEncoder: nonEnumerable(encoding.TextEncoder),
    TextDecoderStream: nonEnumerable(encoding.TextDecoderStream),
    TextEncoderStream: nonEnumerable(encoding.TextEncoderStream),

    // File
    // deno_web - 09 - file
    File: nonEnumerable(file.File),
    Blob: nonEnumerable(file.Blob),

    // FileReader
    // deno_web - 10 - filereader
    FileReader: nonEnumerable(fileReader),

    // Compression
    // deno_web - 14 - compression
    CompressionStream: nonEnumerable(compression.CompressionStream),
    DecompressionStream: nonEnumerable(compression.DecompressionStream),

    // Performance
    // deno_web - 15 - performance
    Performance: nonEnumerable(performance.Performance),
    PerformanceEntry: nonEnumerable(performance.PerformanceEntry),
    PerformanceMark: nonEnumerable(performance.PerformanceMark),
    PerformanceMeasure: nonEnumerable(performance.PerformanceMeasure),
    performance: writable(performance.performance),

    // MessagePort
    structuredClone: writable(messagePort.structuredClone),

    // URL
    // deno_url
    URL: nonEnumerable(url.URL),
    URLPattern: nonEnumerable(urlPattern.URLPattern),
    URLSearchParams: nonEnumerable(url.URLSearchParams),

    // Crypto
    CryptoKey: nonEnumerable(crypto.CryptoKey),
    crypto: readOnly(crypto.crypto),
    Crypto: nonEnumerable(crypto.Crypto),
    SubtleCrypto: nonEnumerable(crypto.SubtleCrypto),

    // Fetch
    // deno_fetch - 20 - headers
    Headers: nonEnumerable(headers.Headers),

    // deno_fetch - 21 - formdata
    FormData: nonEnumerable(formData.FormData),

    // deno_fetch - 23 - request
    Request: nonEnumerable(request.Request),

    // deno_fetch - 23 - response
    Response: nonEnumerable(response.Response),

    // deno_fetch - 26 - fetch
    fetch: nonEnumerable(fetch.fetch),

    // deno_fetch - 27 - eventsource
    EventSource: nonEnumerable(eventSource.EventSource),

    // fetch event
    handleFetchRequest: readOnly(handleFetchRequest),
  };

  const globalProperties = {
    WorkerLocation: location.workerLocationConstructorDescriptor,
    location: location.workerLocationDescriptor,
    WorkerNavigator: nonEnumerable(WorkerNavigator),
    navigator: getterOnly(() => workerNavigator),
    self: getterOnly(() => globalThis),
  };

  let hasBootstrapped = false;

  globalThis.bootstrap = (agent) => {
    core.print(`Bootstrapping runtime\n`);

    if (hasBootstrapped) {
      throw new Error("Worker runtime already bootstrapped");
    }

    hasBootstrapped = true;

    // TODO
    numCpus = 1;
    language = "en-US";
    userAgent = agent ?? "OpenWorkers/0.0.0";

    // Delete globalThis.bootstrap (this function)
    delete globalThis.bootstrap;

    // Delete globalThis.console (from v8)
    delete globalThis.console;

    // delete globalThis.Deno;/
    delete globalThis.__bootstrap;

    // Assign global scope properties
    ObjectDefineProperties(globalThis, windowOrWorkerGlobalScope);

    // Assign global properties
    ObjectDefineProperties(globalThis, globalProperties);

    // Remove Deno from globalThis
    ObjectDefineProperty(globalThis, "Deno", {
      value: undefined,
      writable: true,
      enumerable: false,
      configurable: true,
    });

    core.setMacrotaskCallback(timers.handleTimerMacrotask);
    core.setReportExceptionCallback(event.reportException);

    // core.setWasmStreamingCallback(fetch.handleWasmStreaming);
  };
}
