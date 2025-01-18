// @ts-check
/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

// deno_core
// @ts-ignore
import { core } from "ext:core/mod.js";

import { Headers } from "ext:ow_fetch/20_headers.js";

export class Response {
  constructor(body = null, options = {}) {
    const { status = 200, statusText = "", headers = {} } = options;

    if (status < 200 || status > 599) {
      throw new RangeError("Status code must be between 200 and 599.");
    }

    this.body = body;
    this.status = status;
    this.statusText = statusText;
    this.headers = new Headers(headers);
  }

  bytes() {
    return core.encode(this.body);
  }

  arrayBuffer() {
    return this.bytes();
  }

  clone() {
    return new Response(this.body, {
      status: this.status,
      statusText: this.statusText,
      headers: Object.fromEntries(this.headers),
    });
  }

  async text() {
    return this.body?.toString() || "";
  }

  async json() {
    const text = await this.text();
    return JSON.parse(text);
  }
}

export function toInnerResponse(response) {
  if (!(response instanceof Response)) {
    throw new TypeError("Expected an instance of Response.");
  }

  return {
    body: null,
    status: response.status,
    headerList: Object.entries(response.headers),
  };
}
