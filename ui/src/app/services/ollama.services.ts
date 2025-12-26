import { type Setter } from "solid-js";
import { type StreamChunk } from "@/types/responses";

export async function* generateResponse(input: string, reset: Setter<string>) {
  // const body = {
  //   model: "mistral",
  //   prompt: input,
  //   stream: true,
  // };

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
    // console.log("---->", parts);
    buffer = parts.pop()!; // incomplete chunk stays in buffer

    // const data = JSON.parse(value) as OllamaStreamChunk;
    // yield data;
    for (const msg of parts) {
      // console.log("---->", msg);
      const lines = msg.split("\n");
      const eventLine = lines.find((l) => l.startsWith("event:"));
      if (eventLine !== undefined && !eventLine.endsWith("assistant")) continue; // skip other events
      const dataLines = lines.filter((l) => l.startsWith("data:"));

      for (const line of dataLines) {
        const data = JSON.parse(line.slice(6));
        yield data as StreamChunk;
      }
      // console.log(dataLine);
      // const eventLine = lines.find((l) => l.startsWith("event:"));
      // console.log("===================>", eventLine);
      // if (!dataLine) continue;
      // if (eventLine !== undefined && !eventLine.endsWith("assistant")) continue; // skip other events

      // const data = JSON.parse(dataLine.slice(5));
      // console.log("---->", dataLine.slice(5));
      // yield dataLine.slice(6);
    }
  }
}
