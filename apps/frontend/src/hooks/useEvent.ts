import { useEffect, useState } from "react";
import { usePubSub } from "./usePubSub";
import { ServerMessage } from "artspace-shared";

type ReceivedEvent = Extract<ServerMessage, { event: "received" }>;

export type useEventOptions = {
  events: string[];
  onEvent: (message: ReceivedEvent) => void;
};
export function useEvent(options: useEventOptions) {
  const [lastEvent, setLastEvent] = useState<ReceivedEvent>();

  const { subscribe, unsubscribe } = usePubSub({
    onEvent(message) {
      console.log("use event event");
      if (message.event !== "received") {
        return;
      }

      if (!options.events.includes(message.message.eventName)) {
        return;
      }

      options.onEvent(message);
      setLastEvent(message);
    },
  });

  useEffect(() => {
    for (const event of options.events) {
      subscribe(event);
    }
    return () => {
      for (const event of options.events) {
        unsubscribe(event);
      }
    };
  }, []);

  return { lastEvent };
}
