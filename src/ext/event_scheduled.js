import { core } from "ext:core/mod.js";
import { op_scheduled_init, op_scheduled_respond } from "ext:core/ops";

let scheduledEventListener;

function registerScheduledEventListener(listener) {
  if (typeof listener !== "function") {
    throw new TypeError("Listener must be a function");
  }

  scheduledEventListener = listener;
}

function triggerScheduledEvent(rid) {
  if (!scheduledEventListener) {
    throw new Error("No scheduled event listener registered");
  }

  const evt = op_scheduled_init(rid);

  // Convert seconds to milliseconds
  const scheduledTime = evt.time * 1000;

  scheduledEventListener({
    scheduledTime,
    waitUntil: async (promise) => {
      if (core.isPromise(promise)) {
        await promise;
      }

      op_scheduled_respond(evt.rid);
    },
  });
}

export { triggerScheduledEvent, registerScheduledEventListener };
