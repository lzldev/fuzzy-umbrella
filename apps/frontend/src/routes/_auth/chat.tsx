import { createFileRoute } from "@tanstack/react-router";
import clsx from "clsx";
import { useCallback, useEffect, useRef, useState } from "react";

export const Route = createFileRoute("/_auth/chat")({
  component: WS,
});

type Message = {
  timeStamp: number;
  data: string;
};
function WS() {
  const wsref = useRef<WebSocket>();
  const txtref = useRef<HTMLTextAreaElement>(null);
  const messagesRef = useRef<HTMLDivElement>(null);

  const [isConnected, setIsConnected] = useState(false);
  const [isClosing, setIsClosing] = useState(false);
  const [showGoBack, setShowGoBack] = useState(false);

  const [messages, setMessages] = useState<Message[]>([]);

  const wsConnect = useCallback(() => {
    console.log("WS_CONNECT");
    if (isConnected) {
      // console.error("Tried to open with a open connection");
      return;
    }

    const ws = new WebSocket(
      `ws://${window.location.hostname === "localhost" ? "localhost:8000" : window.location.hostname}/ws/echo`
    );

    ws.addEventListener("open", (event) => {
      console.info("open", event);
      setIsConnected(true);
    });
    ws.addEventListener("message", (event) => {
      console.info("message", event);
      setMessages((messages) => [
        ...messages,
        { data: event.data, timeStamp: event.timeStamp },
      ]);
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

  useEffect(() => {
    const container = messagesRef.current!;
    const size = container.scrollHeight;

    const mult = 1.5;
    const leway = container.scrollHeight - container.clientHeight * mult;

    console.info({
      st: container.scrollTop,
      acc: Math.max(leway, 0),
    });

    if (container.scrollTop < Math.max(leway, 0)) {
      console.info("1 window down");
      setShowGoBack(true);
      return;
    }

    setShowGoBack(false);

    console.info(container.scrollHeight);
    console.info(container.clientHeight);
    console.info(container.scrollHeight + container.clientHeight);
    console.info(container.scrollTop);

    container.scrollTo({
      behavior: "instant",
      top: size,
    });

    console.info("break");
    console.info(container.scrollHeight);
    console.info(container.clientHeight);
    console.info(container.scrollHeight + container.clientHeight);
    console.info(container.scrollTop);
  }, [messages]);

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
            <textarea ref={txtref} className="flex-1 outline-none"></textarea>
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
          </div>
        </div>
        <div className="flex items-center justify-center gap-2 p-4 text-xl bg-slate-300">
          <div className="invisible">Messages: {messages.length}</div>
          <div className="flex justify-center flex-1 flex-grow">
            <button
              className={clsx(
                "bg-fuchsia-500 p-2 rounded-md text-white hover:ring-1 ring-white"
              )}
              onClick={() => {
                setMessages(() => []);
              }}
            >
              Clear
            </button>
          </div>
          <div className="flex text-end">Messages: {messages.length}</div>
        </div>

        <div
          ref={messagesRef}
          className="relative flex flex-col flex-1 flex-grow w-full h-full max-h-full overflow-y-scroll bg-slate-100"
        >
          <button
            className={clsx(
              "fixed bottom-0 left-0 right-0 p-1 rounded-md bg-slate-100 ring-1 ring-fuchsia-500 bg-slatel-100 w-fit",
              !showGoBack && "invisible"
            )}
            onClick={() => {
              const container = messagesRef.current!;

              container.scrollTo({
                behavior: "instant",
                top: container.scrollHeight,
              });
            }}
          >
            go back down
          </button>
          {messages.map((message, idx) => (
            <div
              className="flex p-1 border-b-2 border-b-slate-300 text-fuchsia-500 gap-x-2"
              key={idx}
            >
              <div className="w-full text-slate-400 max-w-16">
                {message.timeStamp.toFixed(0)}
              </div>
              <div className="flex flex-1 flex-grow">{message.data}</div>
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
