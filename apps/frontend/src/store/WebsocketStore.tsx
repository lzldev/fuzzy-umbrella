import { create } from "zustand";

export type WebsocketStore = {
  ws: WebSocket | undefined;
  openSocket: (ws: WebSocket | undefined) => void;
  closeSocket: () => void;
  tickets: number;
  addTicket: () => void;
  removeTicket: () => void;
};

export const useWebSocketStore = create<WebsocketStore>()((set, get) => ({
  ws: undefined,
  tickets: 0,
  openSocket(ws) {
    set((store) => {
      if (store.ws && store.tickets > 0) {
        console.log("Trying to open twice");
        return {};
      }

      return { ws: new WebSocket("ws://localhost:8000/ws/sub") };
    });
  },
  closeSocket() {
    set((store) => {
      if (store.tickets > 0) {
        return {};
      }

      store.ws?.close();
      return { ws: undefined };
    });
  },
  addTicket() {
    set((s) => ({
      tickets: s.tickets + 1,
    }));
  },
  removeTicket() {
    set((s) => ({
      tickets: s.tickets - 1,
    }));
  },
}));
