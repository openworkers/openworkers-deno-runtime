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

  const res = await fetch("https://example.workers.rocks/data.json");

  console.log("Done waiting!", res.status, await res.json());

  return "Called deploy hook!";
}
