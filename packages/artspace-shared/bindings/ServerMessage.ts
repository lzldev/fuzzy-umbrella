// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ErrorMessage } from "./ErrorMessage";
import type { ReceivedMessage } from "./ReceivedMessage";

export type ServerMessage = { "event": "received", "message": ReceivedMessage } | { "event": "error", "message": ErrorMessage } | { "event": "close" };