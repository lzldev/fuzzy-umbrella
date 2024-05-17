import { ServerMessage, ClientMessage } from "artspace-shared";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { useToast } from "~/shadcn/ui/use-toast";
import { useWebSocketStore } from "~/store/WebsocketStore";

export type PubSubOptions = {
  onEvent: (message: ServerMessage) => any;
};

export function usePubSub(options: PubSubOptions) {
  const { toast } = useToast();

  const { ws, addTicket, removeTicket, openSocket, closeSocket } =
    useWebSocketStore();

  const onEvent = useCallback(options.onEvent, [options.onEvent]);

  useEffect(() => {
    if (!ws) {
      return;
    }
    console.log("[PUBSUB] useEffect");

    const onOpenListener = (event: Event) => {
      isOpen.current = true;

      if (startingMessages.current.length === 0) {
        return;
      }

      for (const message of startingMessages.current) {
        send(message);
      }

      startingMessages.current.length = 0;
    };
    const onCloseListener = (event: Event) => {};
    const onErrorListener = (event: Event) => {
      toast({
        title: "Error",
        description: "PubSub error",
      });
    };

    const onMessageListener = (event: MessageEvent<any>) => {
      let message: ServerMessage;
      try {
        message = JSON.parse(event.data);
      } catch (e) {
        toast({
          title: "Error",
          description: "Invalid PubSub Message",
        });
        console.error("PUBSUB JSON PARSE ERROR", e);
        return;
      }

      onEvent(message);
    };

    ws.addEventListener("open", onOpenListener);
    ws.addEventListener("close", onCloseListener);
    ws.addEventListener("error", onErrorListener);
    ws.addEventListener("message", onMessageListener);
    return () => {
      if (!ws) {
        return;
      }
      console.info("Removing pubsub Socket listeners");
      ws.removeEventListener("open", onOpenListener);
      ws.removeEventListener("close", onCloseListener);
      ws.removeEventListener("error", onErrorListener);
      ws.removeEventListener("message", onMessageListener);
    };
  }, [ws]);

  useEffect(() => {
    addTicket();
    return () => {
      removeTicket();
      closeSocket();
    };
  }, []);

  const send = useMemo(() => {
    console.log("[SEND] Update");
    return (message: ClientMessage) => {
      if (!ws) {
        console.error("trying to send event without a socket");
        return;
      }

      ws.send(JSON.stringify(message));
    };
  }, [ws]);

  const isOpen = useRef<boolean>(false);
  const startingMessages = useRef<ClientMessage[]>([]);

  const connectWithMessage = useCallback((startingMessage: ClientMessage) => {
    startingMessages.current.push(startingMessage);
    console.log("setting ws", ws);
    openSocket();
  }, []);

  const subscribe = useCallback(
    (channel: string) => {
      const subscribeMessage = {
        event: "subscribe",
        message: {
          eventName: channel,
        },
      } satisfies ClientMessage;

      if (!ws) {
        console.log("[SUB] Connecting before subscribing...");
        connectWithMessage(subscribeMessage);
        return;
      } else if (!isOpen.current) {
        console.log("[SUB] Just appending message");
        startingMessages.current.push(subscribeMessage);
      }
      send(subscribeMessage);
    },
    [ws, send]
  );

  const unsubscribe = (channel: string) => {
    if (!ws) {
      return;
    }

    send({
      event: "unsubscribe",
      message: {
        eventName: channel,
      },
    });
  };

  return { subscribe, unsubscribe };
}
