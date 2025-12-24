import { type Setter } from "solid-js";
import { type OllamaStreamChunk } from "@/types/responses";

export async function* generateResponse(input: string, reset: Setter<string>) {
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
