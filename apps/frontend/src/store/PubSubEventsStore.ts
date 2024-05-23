import { create } from "zustand";

export type EventStore = {
  eventMap: Map<string, number>;
};

export const useEventStore = create<EventStore>()((set, get) => ({
  eventMap: new Map(),
}));
