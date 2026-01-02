I uninstalled my docker-desktop context (as it was buggy and probably misinstalled from a few years ago)

# Uninstalling Docker

Check to see if it's installed:
- dpkg -l | grep -i docker

sudo apt-get purge -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin docker-ce-rootless-extras
sudo apt-get autoremove -y --purge
sudo apt-get autoclean

Clean up:
- sudo rm -rf /var/lib/docker
- sudo rm /etc/apparmor.d/docker


Check stage:
- docker --version
- dpkg -l | grep -i docker
    - I still had files left on my machine
    
    - sudo apt purge docker-desktop
    - sudo apt autoremove --purge

- dpkg -l | grep -i docker
    - Appears to be fully uninstalled
    
# Reinstall Phase:

Pre-requisite packages:
sudo apt update && sudo apt install -y apt-transport-https ca-certificates curl gnupg-agent software-properties-common

Docker Software Repo:
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/docker.gpg > /dev/null
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt install -y \
  docker-ce \
  docker-ce-cli \
  containerd.io \
  docker-buildx-plugin \
  docker-compose-plugin

Installing NVIDIA Container Toolkit:
- [Source](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
- This installation happens on the host machine
- I believe it may be necessary only on Linux systems or Non Docker-Desktop MacOS/Windows

```bash
sudo apt-get update && sudo apt-get install -y --no-install-recommends \
   curl \
   gnupg2
 ```

```bash
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
  && curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
    sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
    sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
```

```bash
sudo apt-get update
```

```bash
export NVIDIA_CONTAINER_TOOLKIT_VERSION=1.18.1-1
  sudo apt-get install -y \
      nvidia-container-toolkit=${NVIDIA_CONTAINER_TOOLKIT_VERSION} \
      nvidia-container-toolkit-base=${NVIDIA_CONTAINER_TOOLKIT_VERSION} \
      libnvidia-container-tools=${NVIDIA_CONTAINER_TOOLKIT_VERSION} \
      libnvidia-container1=${NVIDIA_CONTAINER_TOOLKIT_VERSION}
```
sudo apt install -y nvidia-container-toolkit
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
docker run --rm --gpus all nvidia/cuda:12.3.2-base-ubuntu22.04 nvidia-smi

This enables me to use the deploy context in the docker-compose for NVIDIA acceleration.

- Lets's download a model: docker exec -it ollama ollama run mistral
- Endpoints: https://docs.ollama.com/api/introduction

Check Docker Status:
- sudo systemctl status docker
- Start manually: sudo systemctl start docker
- Configure start on boot: sudo systemctl enable docker
- Add user to docker group: sudo usermod -aG docker $USER



# Setting up the frontend:
pnpm create vite
pnpm i tailwindcss @tailwindcss/vite
https://tailwindcss.com/docs/installation/using-vite

pnpm i solid-icons
https://solid-icons.vercel.app/


# Setting up Axum Backend api
- cargo new --bin core
- cargo add rig-core (while in local /core directory)


Serve built frontend: 
- cargo add tower-http -F tower-http/fs
- Configure the /ui folder's vite.config.ts file to emit build data into the /core Rust context.
- In the /ui pkg run: pnpm build
