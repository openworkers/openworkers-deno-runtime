import { Headers } from "ext:ow_fetch/20_headers.js";
import { hydrateBody } from "ext:ow_fetch/22_body.js";

import { TextDecoder } from "ext:deno_web/08_text_encoding.js";

export class Request {
  constructor(method, url, headers = {}, body = null) {
    this.method = method.toUpperCase();
    this.url = url; // new URL(url).toString();
    this.headers = new Headers(headers);
    this.body = body;

    if (this.body && (this.method === "GET" || this.method === "HEAD")) {
      throw new TypeError("GET or HEAD requests cannot have a body.");
    }
  }

  async arrayBuffer() {
    return hydrateBody(this.body);
  }

  async text() {
    return new TextDecoder().decode(await this.arrayBuffer());
  }

  async json() {
    return JSON.parse(await this.text());
  }

  clone() {
    return new Request(
      this.method,
      this.url,
      Object.fromEntries(this.headers),
      this.body
    );
  }
}

export function fromInnerRequest(
  { method = "GET", url = "http://unknown/", headers = {}, body },
  _signal // TODO
) {
  return new Request(method, url, headers, body);
}
