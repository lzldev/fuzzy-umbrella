import { ofetch } from "ofetch";

export const api = ofetch.create({
  baseURL: `http://${
    window.location.hostname === "localhost"
      ? "localhost:3000"
      : window.location.hostname
  }/api`,
  mode: "cors",
  credentials: "include",
});
