import { ServerMessage, ClientMessage } from "artspace-shared";
import { useCallback, useEffect, useRef } from "react";
import { useToast } from "~/shadcn/ui/use-toast";
import { useWebSocketStore } from "~/store/PubSubSocketStore";

export type PubSubOptions = {
  onEvent: (message: ServerMessage) => any;
};

const eventMap = new Map();

export function usePubSub(options: PubSubOptions) {
  const { toast } = useToast();

  const { ws, addTicket, removeTicket, openSocket, closeSocket } =
    useWebSocketStore();

  const onEvent = useCallback(options.onEvent, [options.onEvent]);
  const isOpen = useRef<boolean>(!!ws);
  const messageQueue = useRef<ClientMessage[]>([]);

  useEffect(() => {
    addTicket();
    return () => {
      removeTicket();
      closeSocket();
    };
  }, []);

  useEffect(() => {
    if (!ws) {
      return;
    }

    const onCloseListener = (event: Event) => {};
    const onErrorListener = (event: Event) => {
      toast({
        title: "Error",
        description: "PubSub error",
      });
    };

    const onOpenListener = (event: Event) => {
      isOpen.current = true;

      if (messageQueue.current.length === 0) {
        return;
      }

      for (const message of messageQueue.current) {
        send(message);
      }

      messageQueue.current.length = 0;
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
      ws.removeEventListener("open", onOpenListener);
      ws.removeEventListener("close", onCloseListener);
      ws.removeEventListener("error", onErrorListener);
      ws.removeEventListener("message", onMessageListener);
    };
  }, [ws]);

  const send = (message: ClientMessage) => {
    const { ws } = useWebSocketStore.getState();

    if (!ws) {
      return;
    } else if (!isOpen.current) {
      return;
    }

    ws.send(JSON.stringify(message));
  };

  const subscribe = (channel: string) => {
    const { ws } = useWebSocketStore.getState();
    const subscribeMessage = {
      event: "subscribe",
      message: {
        eventName: channel,
      },
    } satisfies ClientMessage;

    if (!isOpen.current) {
      messageQueue.current.push(subscribeMessage);
    }

    const n = eventMap.get(channel);
    if (n) {
      eventMap.set(channel, n + 1);
      return;
    } else {
      eventMap.set(channel, 1);
    }

    if (!ws) {
      openSocket();
      return;
    }

    send(subscribeMessage);
  };

  const unsubscribe = (channel: string) => {
    const { ws } = useWebSocketStore.getState();
    const n = eventMap.get(channel);

    if (n && n > 1) {
      eventMap.set(channel, n - 1);
      return;
    } else if (n === 1) {
      eventMap.delete(channel);
    }

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
