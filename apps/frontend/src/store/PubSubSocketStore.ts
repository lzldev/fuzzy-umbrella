import { create } from "zustand";

export type WebsocketStore = {
  ws: WebSocket | undefined;
  openSocket: () => void;
  closeSocket: () => void;
  tickets: number;
  addTicket: () => void;
  removeTicket: () => void;
};

export const useWebSocketStore = create<WebsocketStore>()((set, get) => ({
  ws: undefined,
  tickets: 0,
  openSocket() {
    set((store) => {
      if (store.ws && store.tickets > 0) {
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
