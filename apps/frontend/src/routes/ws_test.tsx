import { createFileRoute } from "@tanstack/react-router";
import { ClientMessage } from "artspace-shared";
import { useRef } from "react";
import { WebSocketProvider } from "~/context/WebsocketContext";
import { useWebSocket } from "~/hooks/useWebsocket";

export const Route = createFileRoute("/ws_test")({
  component: WSTEST,
});

function WSTEST() {
  return (
    <WebSocketProvider config={{ url: "ws://localhost:8000/ws/chat" }}>
      <WSTESTInner />
    </WebSocketProvider>
  );
}

function WSTESTInner() {
  const { status, send, isOpen, lastMessage } = useWebSocket<
    any,
    ClientMessage
  >("ws://localhost:8000/ws/chat", {
    requireAuth: true,
    onMessage(data) {
      console.log("message", data.data);
    },
  });
  const txt = useRef<HTMLInputElement>(null);

  return (
    <>
      <div className="flex flex-[2] flex-col justify-center items-center p-4 text-xl gap-2">
        <div className="gap-2">
          Connected:
          <span className="font-bold">{status}</span>
          <div className="py-2">{JSON.stringify(lastMessage)}</div>
          <div className="flex flex-col">
            <input ref={txt} type="text" placeholder="YO" />
            <div className="flex justify-between gap-2 pt-2 *:text-center *:justify-center *:flex *:flex-1">
              <button
                className="text-white bg-pink-600 hover:bg-neutral-600"
                onClick={() => {
                  const v = txt.current?.value;

                  send({
                    event: "subscribe",
                    message: { eventName: v! },
                  });
                }}
              >
                SUB
              </button>
              <button
                className="text-white bg-pink-600 hover:bg-neutral-600"
                onClick={() => {
                  const v = txt.current?.value;

                  send({
                    event: "unsubscribe",
                    message: { eventName: v! },
                  });
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
