// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { SubscribeMessage } from "./SubscribeMessage";
import type { UnsubscribeMessage } from "./UnsubscribeMessage";

export type ClientMessage = { "event": "subscribe", "message": SubscribeMessage } | { "event": "unsubscribe", "message": UnsubscribeMessage };