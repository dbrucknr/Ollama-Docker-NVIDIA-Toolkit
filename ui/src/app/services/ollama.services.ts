import { type Setter } from "solid-js";
import { type StreamChunk } from "@/types/responses";

export async function* streamResponse(input: string, reset: Setter<string>) {
  let body = {
    query: input,
  };
  reset("");

  const response = await fetch("/api/ollama/stream", {
    method: "POST",
    body: JSON.stringify(body),
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (!response.body) return;

  const stream = new TextDecoderStream();
  const reader = response.body.pipeThrough(stream).getReader();

  let buffer = "";

  while (true) {
    const { value, done } = await reader.read();
    if (done) {
      console.log("Stream ended");
      break;
    }
    buffer += value;
    let parts = buffer.split("\n\n");
    buffer = parts.pop()!; // incomplete chunk stays in buffer

    for (const msg of parts) {
      const lines = msg.split("\n");
      const eventLine = lines.find((l) => l.startsWith("event:"));
      if (eventLine !== undefined && !eventLine.endsWith("assistant")) continue; // skip other events
      const dataLines = lines.filter((l) => l.startsWith("data:"));

      for (const line of dataLines) {
        const data = JSON.parse(line.slice(6));
        yield data as StreamChunk;
      }
    }
  }
}
