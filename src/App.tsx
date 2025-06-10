import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [images, setImages] = createSignal<string[]>([]);

  let fileInput: HTMLInputElement | undefined;

  async function uploadImages() {
    if (!fileInput?.files) return;

    const buffers = await Promise.all(
      Array.from(fileInput.files).map(file => file.arrayBuffer())
    );

    const payload = buffers.map(buf => Array.from(new Uint8Array(buf)));

    const result = await invoke<string[]>("process_images", { images: payload });
    setImages(result);
  }

  return (
    <main>
      <input type="file" multiple ref={el => fileInput = el} />
      <button onClick={uploadImages}>Send</button>

      <div>
        {images().map((src, i) => (
          <img src={src} alt={`img-${i}`} />
        ))}
      </div>
    </main>
  );
}

export default App;
