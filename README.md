# Ollama-Docker-NVIDIA-Toolkit

## Getting Started
- The default `compose.yml` file is set to use the NVIDIA Docker Toolkit. What this means is that the Ollama container will be GPU accelerated if you have a compatible NVIDIA GPU and the NVIDIA Toolkit installed.
- You must install the NVIDIA Docker Toolkit to use the GPU acceleration feature and have a compatible NVIDIA GPU on the system you deploy to. If you do not have a compatible NVIDIA GPU, you can use the CPU-only version of Ollama by commenting out the `deploy` section in the `compose.yml` file under the `ollama` service.
  - Installing NVIDIA Container Toolkit:
    - [Source](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
    - I have a history of what I opted to do in a `setup.md` file in this repository.
    - I'm running Linux with a NVIDIA 4080 Super on a desktop PC.

## Development Instructions (First time launch)
To begin the development context, you can run:
- `docker compose --profile dev up --build` (add `-d` flag for detached mode)
- Pull the mistral LLM into the container: `docker exec -it ollama ollama run mistral`
- You can run the provided ui by changing into the `ui` directory and running `pnpm run dev`.
  - The `/ui` is set to proxy the backend in the `vite.config.ts` file.
  - If you want Rust to serve the frontend `/ui` package, you must run:
  
```bash
pnpm run build
```
  
<!-- Hot-Reload when /ui is built and hosted from axum -->
pnpm build --watch
docker compose up

To inspect container: docker compose --profile dev run --rm dev bash

# Shimmy Context
docker compose exec shimmy bash
Install Hugging Face CLI: 
- `curl -LsSf https://hf.co/cli/install.sh | bash`
In root Directory: 
- `hf download openai/gpt-oss-20b 
--include "original/*" \ 
--local-dir ./models`

# List available models
curl http://localhost:11435/v1/models

# Chat completion
curl http://localhost:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "model",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'


Agent routing: https://github.com/0xPlaygrounds/rig/blob/main/rig/rig-core/examples/agent_routing.rs
Ollama Stream Pause / Resume: https://github.com/0xPlaygrounds/rig/blob/main/rig/rig-core/examples/ollama_streaming_pause_control.rs
Ollama Tools: https://github.com/0xPlaygrounds/rig/blob/main/rig/rig-core/examples/ollama_streaming_with_tools.rs
Ollama RAG: https://github.com/0xPlaygrounds/rig/blob/main/rig/rig-core/examples/rag_ollama.rs
