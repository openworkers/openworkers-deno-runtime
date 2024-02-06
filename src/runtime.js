// runtime.js

import * as console from "ext:deno_console/01_console.js";
import { core, primordials } from "ext:core/mod.js";

Deno.core.print(`Will setup runtime.js\n`);

{
  const core = Deno.core;

  const {
    ArrayBufferIsView,
    ArrayPrototypeForEach,
    ArrayPrototypePush,
    ArrayPrototypeSort,
    ArrayIteratorPrototype,
    BigInt,
    BigIntAsIntN,
    BigIntAsUintN,
    DataViewPrototypeGetBuffer,
    Float32Array,
    Float64Array,
    FunctionPrototypeBind,
    Int16Array,
    Int32Array,
    Int8Array,
    MathFloor,
    MathFround,
    MathMax,
    MathMin,
    MathPow,
    MathRound,
    MathTrunc,
    Number,
    NumberIsFinite,
    NumberIsNaN,
    NumberMAX_SAFE_INTEGER,
    NumberMIN_SAFE_INTEGER,
    ObjectAssign,
    ObjectCreate,
    ObjectDefineProperties,
    ObjectDefineProperty,
    ObjectGetOwnPropertyDescriptor,
    ObjectGetOwnPropertyDescriptors,
    ObjectGetPrototypeOf,
    ObjectHasOwn,
    ObjectPrototypeIsPrototypeOf,
    ObjectIs,
    PromisePrototypeThen,
    PromiseReject,
    PromiseResolve,
    ReflectApply,
    ReflectDefineProperty,
    ReflectGetOwnPropertyDescriptor,
    ReflectHas,
    ReflectOwnKeys,
    RegExpPrototypeTest,
    SafeRegExp,
    SafeSet,
    SetPrototypeEntries,
    SetPrototypeForEach,
    SetPrototypeKeys,
    SetPrototypeValues,
    SetPrototypeHas,
    SetPrototypeClear,
    SetPrototypeDelete,
    SetPrototypeAdd,
    // TODO(lucacasonato): add SharedArrayBuffer to primordials
    // SharedArrayBufferPrototype,
    String,
    StringPrototypeCharCodeAt,
    StringPrototypeToWellFormed,
    Symbol,
    SymbolIterator,
    SymbolToStringTag,
    TypedArrayPrototypeGetBuffer,
    TypedArrayPrototypeGetSymbolToStringTag,
    TypeError,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
  } = primordials;

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

  // https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope
  const windowOrWorkerGlobalScope = {
    console: nonEnumerable(
      // https://choubey.gitbook.io/internals-of-deno/bridge/4.2-print
      new console.Console((msg, level) => core.print(msg, level > 1))
    ),
  };

  let hasBootstrapped = false;

  globalThis.bootstrap = () => {
    if (hasBootstrapped) {
      throw new Error("Worker runtime already bootstrapped");
    }

    hasBootstrapped = true;

    // Delete globalThis.bootstrap (this function)
    delete globalThis.bootstrap;

    // Delete globalThis.console (from v8)
    delete globalThis.console;

    // delete globalThis.Deno;/
    delete globalThis.__bootstrap;

    Deno.core.print(`Hello bootstrap! ${typeof console} \n\n`);

    ObjectDefineProperties(globalThis, windowOrWorkerGlobalScope);
  };
}
