/* @refresh reload */
import { render } from "solid-js/web";
import "@/assets/index.css";
import App from "@/app/App.tsx";

const root = document.getElementById("root");
if (!root) {
  throw new Error("Could not locate root element.");
}

render(() => <App />, root);
