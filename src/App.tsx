import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  let fileInput: HTMLInputElement | undefined;


  async function uploadImages() {
    if (!fileInput) return;

    const promises = []

    for (const file of fileInput.files ?? []) {
      promises.push(file.arrayBuffer())
    }

    const imageBuffers = await Promise.all(promises);

    const imagePayload = imageBuffers.map(buf => Array.from(new Uint8Array(buf)));

    await invoke("process_images", {
      images: imagePayload
    });
  }





  return (
    <main class="container">
      <input type="file" ref={fileInput} multiple></input>
      <button onClick={uploadImages}>Send</button>
    </main>
  );
}

export default App;
