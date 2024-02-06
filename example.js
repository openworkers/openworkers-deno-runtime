console.log("Hello from script! ");

queueMicrotask(() => {
  console.log("Hello from microtask! ");
});

console.log("globalThis: ", globalThis);
console.log("typeof globalThis.Deno ", typeof globalThis.Deno);
console.log("typeof Deno ", typeof Deno);
console.log("globalThis.Deno === Deno ", globalThis.Deno === Deno);
console.log("typeof core ", typeof core);

console.assert(false, "This is an assertion error");

const hello = btoa("Hello World");
console.log("btoa('Hello World'): ", hello);
console.log(`atob(${hello}): `, atob(hello));

console.log("randomUUID: ", crypto.randomUUID());

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

console.log("Waiting for 100 ms...");

// TODO: never resolves
setTimeout(() => console.log("Done waiting!"), 100);

// TODO; no error but never resolves
(async () => {
  await wait(100);
  console.log("Done waiting await!");
})();

// TODO: error: Top-level await promise never resolved
await wait(100);
