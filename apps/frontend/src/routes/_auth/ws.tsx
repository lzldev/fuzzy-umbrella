import { createFileRoute } from "@tanstack/react-router";
import clsx from "clsx";
import { useCallback, useEffect, useRef, useState } from "react";
import { Container } from "~/components/Container";

export const Route = createFileRoute("/_auth/ws")({
  component: WS,
});

function WS() {
  const wsref = useRef<WebSocket>();
  const [isConnected, setIsConnected] = useState(false);

  const wsConnect = useCallback(() => {
    if (isConnected) {
      console.error("Tried to close with a open connection");
      return;
    }

    const ws = new WebSocket("ws://localhost:8000/ws/");

    ws.addEventListener("open", (event) => {
      console.info("open", event);
    });
    ws.addEventListener("message", (event) => {
      console.info("message", event);
    });
    ws.addEventListener("error", (event) => {
      console.error("error", event);
    });
    ws.addEventListener("close", (event) => {
      console.info("CLOSE", event);
      setIsConnected(false);
    });

    return ws;
  }, []);

  useEffect(() => {
    wsref.current = wsConnect();
  }, []);

  return (
    <Container>
      <div className="flex flex-col justify-center items-center p-4 text-xl gap-2 bg-slate-200">
        <div className="gap-2">
          Connected:
          <span className="font-bold">{isConnected ? "YES" : "NO"}</span>
        </div>
        <button
          className={clsx(
            "bg-fuchsia-500 p-2 rounded-md text-white",
            isConnected && "invisible"
          )}
          onClick={async () => {
            console.info("closing", wsref.current?.close());
            wsConnect();
          }}
        >
          Reconnect
        </button>
      </div>
      <div className="bg-slate-100 w-full flex flex-1 overflow-y-auto"></div>
    </Container>
  );
}
