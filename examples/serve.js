addEventListener("fetch", (event) => {
  event.respondWith(
    handleRequest(event.request).catch(
      (err) => new Response(err.stack, { status: 500 })
    )
  );
});

async function handleRequest(request) {
  if (request.method !== "GET") {
    return new Response("Method Not Allowed", { status: 405 });
  }

  if (request.url.startsWith("/favicon.ico")) {
    return new Response(null, { status: 404 });
  }

  return new Response("Hello world");
}

setTimeout(() => console.log("Hello from timeout"), 500);