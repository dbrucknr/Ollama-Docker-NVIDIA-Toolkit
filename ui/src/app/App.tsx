import { createSignal, type Setter } from "solid-js";

type OllamaStreamChunk = {
  model: string;
  created_at: string;
  response: string;
  done: boolean;
  done_reason?: string;
};

async function* askOllama(input: string, reset: Setter<string>) {
  console.log("Text:", input);

  const body = {
    model: "mistral",
    prompt: input,
    stream: true,
  };
  reset("");

  const response = await fetch(`http://localhost:11434/api/generate`, {
    method: "POST",
    body: JSON.stringify(body),
  });
  if (!response.body) return;
  const stream = new TextDecoderStream();
  const reader = response.body.pipeThrough(stream).getReader();

  while (true) {
    const { value, done } = await reader.read();
    if (done) break;
    const data = JSON.parse(value) as OllamaStreamChunk;
    yield data;
  }
}

function App() {
  const [reply, setReply] = createSignal<string>("");
  const [input, setInput] = createSignal<string>("");

  const submit = async () => {
    setReply("");
    if (input().length > 1) {
      for await (const chunk of askOllama(input(), setInput)) {
        console.log(chunk.response);
        setReply((prev) => prev + chunk.response);
      }
    }
    // Else, throw a helpful message + add form validation
  };

  return (
    <div>
      <input
        type="text"
        min="1"
        placeholder="Enter a prompt"
        value={input()}
        onInput={(e) => setInput(e.currentTarget.value)}
      />
      <button onClick={submit}>Submit</button>
      {reply()}
    </div>
  );
}

export default App;
