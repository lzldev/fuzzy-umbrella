import { createFileRoute } from "@tanstack/react-router";
import clsx from "clsx";
import { useCallback, useEffect, useRef, useState } from "react";
import { ClientMessage } from "artspace-shared";

export const Route = createFileRoute("/ws")({
  component: WSComponent,
});

function WSComponent() {
  const wsref = useRef<WebSocket>();
  const txtref = useRef<HTMLInputElement>(null);

  const [isConnected, setIsConnected] = useState(false);
  const [isClosing, setIsClosing] = useState(false);

  const wsConnect = useCallback(() => {
    console.log("WS_CONNECT");

    const ws = new WebSocket(
      `ws://${window.location.hostname === "localhost" ? "localhost:8000" : window.location.hostname}/ws/sub`
    );

    ws.addEventListener("open", (event) => {
      console.info("open", event);
      setIsConnected(true);
    });
    ws.addEventListener("message", (event) => {
      console.info("message", event);
    });
    ws.addEventListener("error", (...args) => {
      console.error("error", ...args);
    });
    ws.addEventListener("close", (event) => {
      console.info("CLOSE", event);
      setIsConnected(false);
      setIsClosing(false);
    });

    return ws;
  }, [isConnected]);

  useEffect(() => {
    wsref.current = wsConnect();
    return () => {
      console.log("Closing effect");
      wsref.current?.close();
    };
  }, []);

  return (
    <>
      <div className="flex flex-col w-full h-screen-minus-navbar">
        <div className="flex bg-slate-200">
          <div className="flex flex-[2] flex-col justify-center items-center p-4 text-xl gap-2">
            <div className="gap-2">
              Connected:
              <span className="font-bold">{isConnected ? "YES" : "NO"}</span>
            </div>
            <button
              className={clsx(
                "bg-fuchsia-500 p-2 rounded-md text-white hover:ring-1 ring-white"
              )}
              onClick={async () => {
                console.info("CONNECTION CLICK", wsref.current?.close());
                console.info("isClosing", isClosing);
                console.info("isConnected", isConnected);

                if (isConnected) {
                  setIsClosing(true);
                  wsref.current?.close();

                  return;
                }

                wsref.current = wsConnect();
              }}
              disabled={isClosing}
            >
              {isClosing && "Closing.."}
              {!isClosing && isConnected && "Disconnect"}
              {!isClosing && !isConnected && "Reconnect"}
            </button>
          </div>

          <div className="flex flex-col flex-1 gap-1 p-1">
            <input
              type="text"
              ref={txtref}
              className="flex-1 outline-none"
            ></input>
            <div className="flex gap-2 *:flex-1">
              <button
                className={clsx(
                  "bg-fuchsia-500 disabled:bg-fuchsia-300 p-1 rounded-md text-white hover:ring-1 ring-white"
                )}
                onClick={async () => {
                  const msg = txtref.current?.value;

                  wsref.current?.send(msg!);
                }}
                disabled={!isConnected}
              >
                Send
              </button>
              <button
                className={clsx(
                  "bg-fuchsia-500 disabled:bg-fuchsia-300 p-1 rounded-md text-white hover:ring-1 ring-white"
                )}
                onClick={async () => {
                  const msg = txtref.current?.value;

                  wsref.current?.send(
                    JSON.stringify({
                      event: "subscribe",
                      message: {
                        eventName: "wawawawaa",
                      },
                    } satisfies ClientMessage)
                  );
                }}
                disabled={!isConnected}
              >
                Send Sub
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
