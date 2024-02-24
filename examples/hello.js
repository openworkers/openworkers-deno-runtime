handleFetchRequest(async (event) => {
  console.log(
    "fetch called with event: ",
    event.request.method,
    event.request.url
  );

  const request = event.request;

  if (request.url.startsWith("/favicon.ico")) {
    return event.respondWith(new Response(null, { status: 404 }));
  }

  if (request.url.startsWith("/error")) {
    throw new Error("Error from fetch");
  }

  event.respondWith(new Response("Hello World!"));
});
