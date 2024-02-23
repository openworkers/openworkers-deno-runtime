console.log("Hello from script! ");

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

handleFetchRequest(async (event) => {
  console.log("fetch called with event: ", event);

  event.respondWith(wait(50).then(() => new Response("Hello from fetch!")));
  // event.respondWith(fetch("https://example.workers.rocks/data.json"));
});

console.log("setTimeout called");

setTimeout(() => console.log("setTimeout 300 called!!!!"), 300);
setTimeout(() => console.log("setTimeout 900 called!!!!"), 900);
