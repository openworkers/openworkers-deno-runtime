console.log("Hello from script! ");

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

handleFetchRequest(async (event) => {
  console.log("fetch called with event: ", event);

  await wait(50);

  event.respondWith(new Response("Hello from fetch!"));
});

console.log("setTimeout called");

setTimeout(() => console.log("setTimeout 300 called!!!!"), 300);
setTimeout(() => console.log("setTimeout 3000 called!!!!"), 3000);

fetch("https://example.workers.rocks/data.json").then((response) => {
  console.log("fetch response: ", response);
});