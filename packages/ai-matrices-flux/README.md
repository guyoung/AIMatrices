<div align="center">

# AIMatrices Flux

<p align="center">
AIMatrices Flux is an image generation application based on the lightweight AI application rapid construction platform AIMatrices, developed using the Flux model, and supports the automatic translation function of prompt words.

[English](README.md) / [简体中文](README_cn.md)
</p>

![AIMatrices Flux](../../docs/ai-matrices-flux/assets/20250214-ai-matrices-flux.png)

</div>
<br />
<br />

## Features

**Out of the box**

AIMatrices Flux Developed in the Rust programming language, it directly compiles into a binary executable file, and the resulting executable is about 40MB in size. There is no need to install any external dependencies, which greatly simplifies the deployment process, and it can run on multiple operating systems (such as Windows, macOS, Linux) without using containers.

**Broad hardware platform support**

AIMatrices Flux uses Rust FFI technology to call the high-performance lama.cpp library, and uses Vulkan as the back-end for model inference. It supports NVIDIA, AMD, Intel and other mainstream graphics cards, which can provide significant GPU acceleration effect for model inference. This architectural design allows the project to take full advantage of the powerful computing power of modern Gpus while maintaining good cross-platform compatibility.

**Fully open source**

AIMatrices DeepSeek is an open source project that gives developers free access to its code for customization and extension.


## Getting Started

### Install

#### 1.Download executable files directly

From AIMatrices [GitHub Release](https://github.com/guyoung/AIMatrices/releases) to download the ai-matrices-flux corresponding system version, after decompression can be used

#### 2.Source code build

A prerequisite for build is the installation of the Rust environment

```shell

git clone https://github.com/guyoung/AIMatrices.git

## Build AIMatrices Flux main program
cd AIMatrices/packages/ai-matrices-flux
cagro build -- release

## Build AIMatrices WebAssembly components
cd AIMatrices/components/flux-handler-component
cargo component build --release
```

### Download model file

* llama-translate model(Prompt word automatic translation)
    * https://huggingface.co/dahara1/llama-translate-gguf/llama-translate.f16.Q4_K_M.gguf
    * mirror: https://hf-mirror.com/dahara1/llama-translate-gguf/llama-translate.f16.Q4_K_M.gguf

* Flux.1 Dev model
    * https://huggingface.co/city96/FLUX.1-dev-gguf/resolve/main/flux1-dev-Q4_0.gguf
    * mirror: https://hf-mirror.com/city96/FLUX.1-dev-gguf/resolve/main/flux1-dev-Q4_0.gguf

* vae model
    * https://huggingface.co/black-forest-labs/FLUX.1-dev/resolve/main/ae.safetensors
    * mirror: https://hf-mirror.com/black-forest-labs/FLUX.1-dev/resolve/main/ae.safetensors

* clip_l model 
    * https://huggingface.co/comfyanonymous/flux_text_encoders/resolve/main/clip_l.safetensors
    * mirror: https://hf-mirror.com/comfyanonymous/flux_text_encoders/resolve/main/clip_l.safetensors

* t5xxl model
    * https://huggingface.co/comfyanonymous/flux_text_encoders/resolve/main/t5xxl_fp8_e4m3fn.safetensors
    * mirror: https://hf-mirror.com/comfyanonymous/flux_text_encoders/resolve/main/t5xxl_fp8_e4m3fn.safetensors

### Configuration

Open the app-config.json file in the app_data directory and change the model path.
```json
[{
      "id": "flux",
      "metadata": {
        "build": { },
        "local_llm_models": {
          "local/llama-translate": "./app_data/ai-models/huggingface/dahara1/llama-translate-gguf/llama-translate.f16.Q4_K_M.gguf"
        },
        "local_sd_models": {
          "stable-diffusion-v1-5": "./app_data/ai-models/huggingface/runwayml/stable-diffusion-v1-5/v1-5-pruned-emaonly.safetensors",
          "flux1-dev": "./app_data/ai-models/huggingface/city96/FLUX.1-dev-gguf/flux1-dev-Q4_0.gguf",
          "t5xxl": "./app_data/ai-models/huggingface/comfyanonymous/flux_text_encoders/t5xxl_fp8_e4m3fn.safetensors",
          "vae": "./app_data/ai-models/huggingface/black-forest-labs/FLUX.1-dev/ae.safetensors",
          "clip_l": "./app_data/ai-models/huggingface/comfyanonymous/flux_text_encoders/clip_l.safetensors"
        }
      },
      "source": {
        "content_type": "application/wasm",
        "source": "./app_data/components/http_flux_handler_component.wasm"
      }
    },
    {
      "id": "static-file",
      "metadata": {
        "build": {}
      },
      "source": {
        "content_type": "application/wasm",
        "source": "./app_data/components/http_static_file_handler_component.wasm"
    }
}]
```


### Run

Windows Command-line run
```shell
ai-matrices-flux-windows-amd64.exe
```

Linux Command-line run
```shell
./ai-matrices-flux-linux-amd64
```

Command-line arguments:
* -i,--ip <port_number>: Specifies the IP address that the server should listen on; default is 127.0.0.0.
  * example: --ip 0.0.0.0
* -p,--port <port_number>: Specifies the port on which the server should listen; default is 21500.
  * example: --port 3000
* --appdir <appdir>: Specifies the working directory, default is ./app_data
  * example:--app-dir ./dir1
* --appconfig <appconfig>:  Specifies the app configuration file, default is ./app_data/app-config.json
  * example:--appconfig ./dir1/config.json
* --user <username>: Specifies the login username; default is empty
  * example:--user admin
* --pass <password>:Specifies the password of the logged-in user; the default is empty
  * example:--pass admin
* --version: Prints the system version number and exits.
* --help: See the instructions and arguments for the command.

```shell
ai-matrices-deepseek-flux-amd64.exe -i 0.0.0.0 -p 8080 --user admin -- pass admin
```

## Other

AIMatrices Flux front-end is generated using DeepSeek.


