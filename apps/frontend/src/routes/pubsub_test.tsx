import { createFileRoute } from "@tanstack/react-router";
import { useRef } from "react";
import { WebSocketProvider } from "~/context/WebsocketContext";
import { usePubSub } from "~/hooks/usePubSub";
import { useToast } from "~/shadcn/ui/use-toast";

export const Route = createFileRoute("/pubsub_test")({
  component: PUBSUBTEST,
});

function PUBSUBTEST() {
  return (
    <WebSocketProvider config={{ url: "ws://localhost:8000/ws/chat" }}>
      <WSTESTInner />
    </WebSocketProvider>
  );
}

function WSTESTInner() {
  const txt = useRef<HTMLInputElement>(null!);
  const { toast } = useToast();
  const { unsubscribe, subscribe } = usePubSub({
    onEvent(message) {
      console.log("[Consumer] Pubsub Message : ", message);
      if (message.event !== "received") {
        return;
      }

      toast({ title: "Event", description: message.message.eventName });
    },
  });

  return (
    <>
      <div className="flex flex-[2] flex-col justify-center items-center p-4 text-xl gap-2">
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

      <div></div>
    </>
  );
}
