import { ServerMessage, ClientMessage } from "artspace-shared";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useToast } from "~/shadcn/ui/use-toast";

export type PubSubOptions = {
  onEvent: (message: ServerMessage) => any;
};

export function usePubSub(options: PubSubOptions) {
  const { toast } = useToast();

  const [ws, setWs] = useState<WebSocket>();

  const onEvent = useCallback(options.onEvent, [options.onEvent]);

  useEffect(() => {
    if (!ws) {
      return;
    }
    console.log("[PUBSUB] useEffect");

    const onOpenListener = (event: Event) => {
      if (firstMessage.current) {
        send(firstMessage.current);
        firstMessage.current = undefined;
      }
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

  const firstMessage = useRef<ClientMessage>();

  const connectWithMessage = useCallback((startingMessage: ClientMessage) => {
    firstMessage.current = startingMessage;
    setWs(new WebSocket("ws://localhost:8000/ws/sub"));
  }, []);

  const subscribe = useCallback(
    (channel: string) => {
      const message = {
        event: "subscribe",
        message: {
          eventName: channel,
        },
      } satisfies ClientMessage;

      if (!ws) {
        console.log("[SUB] Connecting before subscribing...");
        connectWithMessage(message);
        return;
      }
      send(message);
    },
    [ws, send]
  );

  const unsubscribe = (channel: string) => {
    if (!ws) {
      console.error("trying to unsub before creating a socket");
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
