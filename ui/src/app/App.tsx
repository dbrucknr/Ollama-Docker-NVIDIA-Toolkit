import { createSignal, createEffect, onMount } from "solid-js";
import { generateResponse } from "@/app/services/ollama.services";

import DefaultLayout from "@/app/layouts/Default";

function App() {
  const [replyChunks, setReplyChunks] = createSignal<string>("");
  const [input, setInput] = createSignal<string>("");

  // const [placeholder, setPlaceholder] = createSignal<string>("");

  const submit = async () => {
    setReplyChunks("");
    if (input().length > 1) {
      for await (const chunk of generateResponse(input(), setInput)) {
        console.log("---->", chunk);
        setReplyChunks((prev) => prev + chunk);
      }
    }
    // Else, throw a helpful message + add form validation
  };

  let replyRef: HTMLPreElement | undefined;

  createEffect(() => {
    replyChunks(); // Track Chunks
    // placeholder();
    replyRef?.scrollIntoView({ behavior: "smooth", block: "end" });
  });

  // onMount(async () => {
  //   const response = await fetch("https://jsonplaceholder.typicode.com/posts");
  //   if (response.ok) {
  //     const text = await response.text();
  //     setPlaceholder(text);
  //   }
  // });

  return (
    <DefaultLayout>
      <div class="flex flex-col flex-1 overflow-hidden">
        {/* Reply area */}
        <div class="flex-1 overflow-y-auto p-4">
          <pre ref={replyRef} class="whitespace-pre-wrap overflow-y-auto">
            {replyChunks()}
            {/*{placeholder()}*/}
          </pre>
        </div>
        {/* Input area */}
        <div class="border-t p-4 flex gap-2">
          <input
            type="text"
            min="1"
            placeholder="Enter a prompt"
            value={input()}
            onInput={(e) => setInput(e.currentTarget.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                submit();
              }
            }}
            class="flex-1 border rounded px-2 py-1"
          />
          <button
            onClick={submit}
            class="px-4 py-1 rounded bg-blue-600 text-white cursor-pointer"
          >
            Submit
          </button>
        </div>
      </div>
    </DefaultLayout>
  );
}

export default App;
