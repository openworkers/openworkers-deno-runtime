console.log("Hello from script! ");

handleFetchRequest(async function fetch(request) {
  console.log("fetch called with request: ", request);

  return "Response from fetch!";
});