import { useContext, createContext, PropsWithChildren, useState } from "react";

export type WebSocketConfig = {
  url: string;
};
export function useWebsocketContext() {
  return useContext(WebSocketContext);
}

const WebSocketContext = createContext<WebSocket>(null as any);

export function WebSocketProvider({
  children,
  config,
}: { config: WebSocketConfig } & PropsWithChildren) {
  const [ws] = useState(() => new WebSocket(config.url));
  return (
    <WebSocketContext.Provider value={ws}>{children}</WebSocketContext.Provider>
  );
}
