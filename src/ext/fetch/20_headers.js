export class Headers {
  constructor(init = {}) {
    Object.assign(this, init);
  }

  // Iterable
  [Symbol.iterator]() {
    return Object.entries(this)[Symbol.iterator]();
  }
}
