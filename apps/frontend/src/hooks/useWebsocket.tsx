import { useAuth } from "@clerk/clerk-react";
import { useCallback, useEffect, useMemo, useState } from "react";

export type useWebsocketConfig<T> = {
  requireAuth?: boolean;
  onMessage?: (data: MessageEvent<T>) => any;
  onOpen?: (data: Event) => any;
  onClose?: (data: Event) => any;
  onError?: (data: Event) => any;
};

export type WebsocketStatus =
  | "OPENING"
  | "CLOSED"
  | "OK"
  | "ERROR"
  | "UNAUTHORIZED"
  | "LOADING";

export function useWebSocket<
  MessageType extends unknown,
  SendMessageType extends unknown =
    | string
    | ArrayBufferLike
    | Blob
    | ArrayBufferView,
>(url: string, config?: useWebsocketConfig<MessageType>) {
  const {
    onMessage = () => {},
    onOpen = () => {},
    onClose = () => {},
    onError = () => {},
    requireAuth = false,
  } = config || {};

  const { isLoaded, isSignedIn } = useAuth();

  const ws = useMemo(() => {
    if (requireAuth && !isLoaded) {
      return;
    }

    if (requireAuth && !isSignedIn) {
      return;
    }

    return new WebSocket(url);
  }, [url, isLoaded, isSignedIn]);

  const [lastMessage, setLastMessage] = useState<MessageType | null>(null);

  const updateState = useCallback(
    (socket: WebSocket) =>
      requireAuth && !isLoaded
        ? "LOADING"
        : requireAuth && !isSignedIn
          ? "UNAUTHORIZED"
          : socket
            ? matchWebsocketState(socket.readyState)
            : "CLOSED",
    [requireAuth, isSignedIn]
  );

  const [status, setStatus] = useState<WebsocketStatus>(updateState(ws!));

  useEffect(() => {
    setStatus(updateState(ws!));
  }, [ws, isSignedIn, isLoaded, requireAuth]);

  const send = useCallback(
    (data: SendMessageType) => ws?.send(JSON.stringify(data)),
    [ws]
  ) as (data: SendMessageType) => void;

  const isOpen = useMemo(() => status === "OK", [status]);

  useEffect(() => {
    function lastMessageListener(message: MessageEvent<any>) {
      setLastMessage(message.data);
    }

    const onChangeListener = () => {
      setStatus(updateState(ws!));
    };

    ws?.addEventListener("open", onChangeListener);
    ws?.addEventListener("close", onChangeListener);
    ws?.addEventListener("message", lastMessageListener);

    if (!requireAuth || (requireAuth && isSignedIn)) {
      ws?.addEventListener("message", onMessage);
      ws?.addEventListener("open", onOpen);
      ws?.addEventListener("close", onClose);
      ws?.addEventListener("error", onError);
    }

    return () => {
      ws?.removeEventListener("open", onChangeListener);
      ws?.removeEventListener("close", onChangeListener);
      ws?.removeEventListener("message", lastMessageListener);

      ws?.removeEventListener("message", onMessage);
      ws?.removeEventListener("open", onOpen);
      ws?.removeEventListener("close", onClose);
      ws?.removeEventListener("error", onError);
    };
  }, [ws, status, onError, onMessage]);

  return { isOpen, lastMessage, status, send };
}

function matchWebsocketState(state: number): WebsocketStatus {
  switch (state) {
    case 0:
      return "OPENING";
    case 1:
      return "OK";
    case 2:
    case 3:
      return "CLOSED";
    default:
      return "ERROR";
  }
}
