function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

addEventListener("scheduled", (event) => {
  event.waitUntil(handleSchedule(event.scheduledTime));
});

async function handleSchedule(scheduledDate) {
  console.log(
    "Called scheduled event:",
    scheduledDate,
    new Date(scheduledDate).toISOString()
  );

  const res = await fetch("https://echo.workers.rocks/data.json");

  let data = await res.json();

  console.log("Done waiting!", res.status, { agent: data["user-agent"] });

  return "Called deploy hook!";
}

setTimeout(() => console.log("Hello from timeout"), 200);