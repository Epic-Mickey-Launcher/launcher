import "./style.css";
import App from "./App.svelte";
import { mount } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import { SetServerURL } from "./routes/library/networking";

invoke("get_server_url").then((url: string) => {
  SetServerURL(url);
  console.log("server url set to: " + url);
});

const app = mount(App, { target: document.getElementById("app") });

export default app;
