declare class Console {
  log(message?: any, ...optionalParams: any[]): void;
  debug(message?: any, ...optionalParams: any[]): void;
  info(message?: any, ...optionalParams: any[]): void;
  warn(message?: any, ...optionalParams: any[]): void;
  error(message?: any, ...optionalParams: any[]): void;
}

declare const internals: any;

declare module "ext:core/ops" {
  export const op_log: any;

  // Event fetch
  interface RustRequest {
    method: string;
    url: string;
    headers: Array<[string, string]>;
    body: Uint8Array | null;
  }

  interface RustResponse {
    status: number;
    headerList: Array<[string, string]>;
    body: ArrayBuffer | Uint8Array;
  }

  export const op_fetch_init: (rid: number) => {
    req: RustRequest;
    rid: number;
  };

  export const op_fetch_respond: (rid: number, response: RustResponse) => void;

  // Event scheduled
  export const op_scheduled_init: any;
  export const op_scheduled_respond: any;
}

declare module "ext:core/mod.js" {
  type PromiseRejectCallback = (
    promise: Promise<unknown>,
    reason: any
  ) => boolean;

  type PromiseHandledCallback = (
    promise: Promise<unknown>,
    reason: any
  ) => void;

  interface DenoCore {
    encode(input: string): Uint8Array;
    decode(input: Uint8Array): string;
    ops: Record<string, (...args: unknown[]) => any>;
    asyncOps: Record<string, (...args: unknown[]) => any>;
    isPromise<T, V>(value: Promise<T> | V): value is Promise<T>;
    print(message: string, is_err?: boolean): void;
    setHandledPromiseRejectionHandler(cb: PromiseHandledCallback): void;
    setUnhandledPromiseRejectionHandler(cb: PromiseRejectCallback): void;
    setReportExceptionCallback(cb: (err: Error) => void): void;
    setWasmStreamingCallback(cb: (source: any, rid: number) => void): void;
    setMacrotaskCallback(cb: () => boolean): void;
    setNextTickCallback(cb: () => void): void;
  }

  export const core: DenoCore;
  export const primordials: any;
}

declare module "ext:deno_web/01_dom_exception.js" {
  export const DOMException: any;
}

declare module "ext:deno_web/03_abort_signal.js" {
  export class AbortSignal {}
  export const AbortController: any;
  export const newSignal: () => AbortSignal;
}

// declare module "ext:deno_web/06_streams.js" {
//   export const readableStreamForRid;
// }

declare module "ext:deno_fetch/20_headers.js" {
  export type HeaderList = Array<[string, string]>;

  export class Headers {}

  export type GuardString =
    | "request"
    | "immutable"
    | "request-no-cors"
    | "response"
    | "none";

  export const guardFromHeaders: (headers: Headers) => GuardString;
  export const headersFromHeaderList: (headerList: HeaderList) => Headers;
}

declare module "ext:deno_fetch/22_body.js" {
  export class InnerBody {
    constructor(stream: { body: Uint8Array | string; consumed: boolean });
  }
}

declare module "ext:deno_fetch/23_response.js" {
  import { HeaderList } from "ext:deno_fetch/20_headers.js";
  import { InnerBody } from "ext:deno_fetch/22_body.js";

  export interface InnerResponse {
    status: number;
    statusMessage: string;
    headerList: HeaderList;
    body: InnerBody;
  }

  export class Response {
    bytes(): Promise<Uint8Array>;
  }

  export const toInnerResponse: (response: Response) => InnerResponse;
}

declare module "ext:deno_fetch/23_request.js" {
  import { HeaderList } from "ext:deno_fetch/20_headers.js";
  import { InnerBody } from "ext:deno_fetch/22_body.js";
  import { AbortSignal } from "ext:deno_web/03_abort_signal.js";
  import { GuardString } from "ext:deno_fetch/20_headers.js";

  export class Request {}

  export class InnerRequest {
    method: string;
    url: string;
    headerList: HeaderList;
    body: Uint8Array | null;
  }

  export const fromInnerRequest: (
    inner: InnerRequest,
    signal?: AbortSignal | null,
    guard?: GuardString
  ) => Request;

  export const newInnerRequest: (
    method: string,
    url: string | (() => string),
    headerList: () => HeaderList,
    body: InnerBody | null
  ) => InnerRequest;
}

declare module "ext:core/*" {
  const value: any;
  export = value;
}

declare module "ext:deno_*" {
  const value: any;
  export = value;
}
