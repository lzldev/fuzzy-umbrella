import { createFileRoute } from "@tanstack/react-router";
import { useEffect, useRef, useState } from "react";
import { useEvent } from "~/hooks/useEvent";
import { usePubSub } from "~/hooks/usePubSub";
import { useToast } from "~/shadcn/ui/use-toast";

export const Route = createFileRoute("/pubsub_test")({
  component: PUBSUBTEST,
});

function PUBSUBTEST() {
  const [show, setShow] = useState(false);
  const [show2, setShow2] = useState(false);
  const { subscribe, unsubscribe } = usePubSub({
    onEvent(message) {
      console.info("That");
      console.info(message);
    },
  });

  useEffect(() => {
    // subscribe("CHAT:GLOBAL");
    return () => {
      unsubscribe("CHAT:GLOBAL");
    };
  }, []);

  return (
    <>
      <button
        className="text-white bg-pink-600 hover:bg-neutral-600"
        onClick={() => {
          setShow(!show);
        }}
      >
        {show ? "CLOSE" : "SHOW"}
      </button>
      {show && <PUBSUBTESTER />}
      <button
        className="text-white bg-pink-600 hover:bg-neutral-600"
        onClick={() => {
          setShow2(!show2);
        }}
      >
        {show2 ? "CLOSE" : "SHOW"}
      </button>
      {show2 && <PUBSUBTESTER />}
    </>
  );
}
function PUBSUBTESTER() {
  const txt = useRef<HTMLInputElement>(null!);
  const { toast } = useToast();
  const { lastEvent } = useEvent({
    events: ["CHAT:GLOBAL", "WAWAWA", "TRUE"],
    onEvent(message) {
      console.log("useevent");
      toast({ title: "USEEVENT", description: message.message.eventName });
    },
  });

  const { unsubscribe, subscribe } = usePubSub({
    onEvent(message) {
      console.log("[Consumer] Pubsub Message : ", message);
      if (message.event !== "received") {
        return;
      }
    },
  });

  return (
    <>
      <div className="flex flex-[2] flex-col justify-center items-center p-4 text-xl gap-2 bg-neutral-200">
        <div className="text-3xl">{lastEvent?.message.eventName}</div>
        <div className="gap-2">
          Connected:
          <span className="font-bold">{status}</span>
          <div className="flex flex-col">
            <input ref={txt} type="text" placeholder="YO" />
            <div className="flex justify-between gap-2 pt-2 *:text-center *:justify-center *:flex *:flex-1">
              <button
                className="text-white bg-pink-600 hover:bg-neutral-600"
                onClick={() => {
                  const v = txt.current.value;
                  subscribe(v);
                }}
              >
                SUB
              </button>
              <button
                className="text-white bg-pink-600 hover:bg-neutral-600"
                onClick={() => {
                  const v = txt.current?.value;
                  unsubscribe(v);
                }}
              >
                UNSUB
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
