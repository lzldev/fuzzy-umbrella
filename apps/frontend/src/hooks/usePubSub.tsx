import { ServerMessage, ClientMessage } from "artspace-shared";
import { useCallback, useEffect, useRef } from "react";
import { useToast } from "~/shadcn/ui/use-toast";

export type PubSubOptions = {
  onEvent: (message: ServerMessage) => any;
};

export function usePubSub(options: PubSubOptions) {
  const { toast } = useToast();

  const ws = useRef<WebSocket>();

  const send = useCallback(
    (message: ClientMessage) => {
      if (!ws.current) {
        console.error("trying to send event without a socket");
        return;
      }
      ws.current?.send(JSON.stringify(message));
    },
    [ws.current]
  );

  const updateListeners = () => {
    if (!ws.current) {
      console.info("Effect : Socket not found");
      return;
    }

    console.info("Adding pubsub Socket listeners");
    const onOpenListener = (event: Event) => {
      console.info("Pubsub Open");
    };
    const onCloseListener = (event: Event) => {
      console.info("Pubsub Close");
    };
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

      options.onEvent(message);
    };

    ws.current.addEventListener("open", onOpenListener);
    ws.current.addEventListener("close", onCloseListener);
    ws.current.addEventListener("error", onErrorListener);
    ws.current.addEventListener("message", onMessageListener);
    // return () => {
    //   if (!ws.current) {
    //     return;
    //   }
    //   console.info("Removing pubsub Socket listeners");
    //   ws.current.removeEventListener("open", onOpenListener);
    //   ws.current.removeEventListener("close", onCloseListener);
    //   ws.current.removeEventListener("error", onErrorListener);
    //   ws.current.removeEventListener("message", onMessageListener);
    // };
  };

  const connect = () => {
    console.log("PubSub connecting...");
    if (ws.current) {
      return;
    }

    ws.current = new WebSocket("ws://localhost:8000/ws/sub");
    updateListeners();
  };

  const subscribe = (channel: string) => {
    if (!ws.current) {
      console.info("Connect and subscribe");
      connect();
      const l = () => {
        _subscribe(channel);
        ws.current!.removeEventListener("open", l);
      };
      ws.current!.addEventListener("open", l);
      return;
    }

    console.info("just sub");
    _subscribe(channel);
  };

  const _subscribe = (channel: string) => {
    console.info("Subscribing to ", channel);
    send({
      event: "subscribe",
      message: {
        eventName: channel,
      },
    });
  };
  const unsubscribe = (channel: string) => {
    if (!ws.current) {
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
