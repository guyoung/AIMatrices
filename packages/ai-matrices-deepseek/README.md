<div align="center">

# AIMatrices DeepSeek

<p align="center">
AIMatrices DeepSeek is an open source DeepSeek localization deployment tool based on AIMatrices, a lightweight AI application rapid construction platform, which supports flexible switching between local models and remote apis. This paper aims to provide users with an efficient, flexible and low-cost DeepSeek large model deployment solution.

[English](README.md) / [简体中文](README_cn.md)
</p>

![AIMatrices DeepSeek](../../docs/ai-matrices-deepseek/assets/20250215-ai-matrices-deepseek3.png)

</div>
<br />
<br />


## What's new

* 2025-02-22
  * Added multi-round dialogue feature; Added the feature of saving dialogue records to the local database; Added Mermaid chart preview and export feature;Added Markdwon editing and preview feature
* 2025-02-28
  * Added Ollama Api interface call feature; Added Python code directly run and edit feature; Added Html code directly browse and edit feature

## Features

**Out of the box**

Developed in the Rust programming language, it directly compiles into a binary executable file, and the resulting executable is about 40MB in size. There is no need to install any external dependencies, which greatly simplifies the deployment process, and it can run on multiple operating systems (such as Windows, macOS, Linux) without using containers.

**Broad hardware platform support**

It uses Rust FFI technology to call the high-performance lama.cpp library, and uses Vulkan as the back-end for model inference. It supports NVIDIA, AMD, Intel and other mainstream graphics cards, which can provide significant GPU acceleration effect for model inference. This architectural design allows the project to take full advantage of the powerful computing power of modern Gpus while maintaining good cross-platform compatibility.

**Multi-model support and flexible switching**

Multiple DeepSeek-R1-Distill-Qwen model versions are supported, including 1.5B, 7B, 14B, etc. Users can choose the appropriate model for local deployment according to their hardware configuration to meet the dual requirements of data privacy and performance. At the same time, it supports calling DeepSeek model service through remote API without local hardware support. Users can flexibly switch between local model and remote API according to actual needs to achieve efficient deployment and resource optimization.

**Secure and efficient**

The Rust programming language is developed based on AIMatrices platform to ensure that the application can run safely and efficiently on different devices and operating systems. Rust's ownership system and lifecycle checking fundamentally eliminate common errors such as null Pointers, data races, and memory leaks, allowing the majority of potential memory issues to be found at compile time, resulting in fewer runtime errors and crashes. Rust's performance is close to C/C++ while avoiding the overhead of garbage collection mechanisms. It ensures extremely efficient code execution through zero-cost abstractions and efficient compiler optimizations.

**Flexible configuration and componentization**

The WebAssembly component technology is used to realize highly flexible component-based development by defining application configuration information such as components, routing rules, and environment variables through application configuration files. For example, you can configure allowed_outbound_hosts to control a component's access to external requests, or file mappings to enable static resource hosting.

**Extensibility and customization**

Model API calls are made according to the OpenAI API specification, and the model calls and results are obtained through standardized interface parameters, such as input text, model selection, temperature parameters, etc. In addition to the DeepSeek model, other OpenAI API models are also supported. At the same time, it supports the use of JavaScript code for custom extension, allowing users to achieve personalized logic processing according to business needs.

**AI Chat is feature-rich**

The model API call was carried out according to the OpenAI API specification, and the model call and result acquisition were realized through standardized interface parameters, such as input text, model selection, temperature parameters, etc. In addition to the DeepSeek model, other OpenAI API models were also supported. Ollama Api interface calls are also supported to interact with the local model. At the same time, it supports the use of JavaScript code for custom extension, allowing users to achieve personalized logic processing according to business needs.

**User-friendly interface**

Multi-language switching is supported, and users can choose the interface language according to their language preferences to ensure the convenience and comfort of the use experience. The front-end page adopts a responsive design, which can automatically adjust the layout and element size according to the device screen size, ensuring that it can also provide clear and smooth visual effects on mobile phones, tablets and other mobile devices, providing a better mobile terminal experience.

**Open source**

AIMatrices DeepSeek is an open source project that gives developers free access to its code for customization and extension.


## Getting Started

### Install

#### 1.Download executable files directly

From AIMatrices [GitHub Release](https://github.com/guyoung/AIMatrices/releases) to download the ai-matrices-deepseek corresponding system version, after decompression can be used

#### 2.Source code build

A prerequisite for build is the installation of the Rust environment

```shell


git clone https://github.com/guyoung/AIMatrices.git

## Build AIMatrices DeepSeek main program
cd AIMatrices/packages/ai-matrices-deepseek
cagro build -- release

## Build AIMatrices WebAssembly components
cd AIMatrices/components/llm-handler-component
cargo component build --release
cd AIMatrices/components/llm-local-handler-component
cargo component build --release
cd AIMatrices/components/static-file-handler-component
cargo component build --release
```

### Download model file

* DeepSeek-R1-Distill-Qwen-7B-GGUF 
    * https://huggingface.co/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf
    * mirror: https://hf-mirror.com/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf

Other models can be selected according to the actual situation of the running computer

### Configuration

To run a local model, you need to change the local model information in the application config file by opening the app-config.json file in the app_data directory and changing the model path.

```json
{
    "id": "deepseek-local",
    "metadata": {
      "build": {},
      "local_llm_models": {
        "local/DeepSeek-R1": "./app_data/ai-models/huggingface/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf"
      }
    },
    "source": {
      "content_type": "application/wasm",
      "source": "./app_data/components/http_llm_local_handler_component.wasm"
    }
}
```
local/DeepSeek-R1 is the model_id, and the value is the path of the model.

Multiple models can be configured at the same time.
```json
{
  "local_llm_models": {
    "local/DeepSeek-R1-7B": "./app_data/ai-models/huggingface/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf",
    "local/DeepSeek-R1-0.5B": "./app_data/ai-models/huggingface/lmstudio-community/DeepSeek-R1-Distill-Qwen-0.5B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf"
  }
}
```

If you want to build from source code, you will also need to change the location of the WebAssembly component file
```json
{
  "source": {
    "content_type": "application/wasm",
    "source": "./app_data/components/http_llm_local_handler_component.wasm"
  }
}
```

Open the index.js file in app_data/codes/deepseek-api directory and change the local model and remote API configuration information.
```javascript
const models = {
  "local/deepseek-r1": {
    name: "DeepSeek-R1(本地)", 
    model: "local/DeepSeek-R1", 
    local: true,
    url: "/service/deepseek-local",
    api_key: ""
  },
  "deepseek-chat": {
    name: "DeepSeek Chat", 
    model: "deepseek-chat", 
    local: false,
    url: "https://api.deepseek.com",
    api_key: "sk-********************************"
  },

  "deepseek-reasoner": {
    name: "DeepSeek Reasoner", 
    model: "deepseek-reasoner", 
    local: false,
    url: "https://api.deepseek.com",
    api_key: "sk-********************************"
  },
  "siliconflow/deepseek-v3": {
    name: "DeepSeek-V3(硅基流动)", 
    model: "deepseek-ai/DeepSeek-V3", 
    local: false,
    url: "https://api.siliconflow.cn/v1",
    api_key: "sk-********************************"
  },
  "siliconflow/deepseek-r1": {
    name: "DeepSeek-R1(硅基流动)", 
    model: "deepseek-ai/DeepSeek-R1", 
    local: false,
    url: "https://api.siliconflow.cn/v1",
    api_key: "sk-********************************"
  },
}

```

key is model_id, local value true is the local model, the local model model_id is consistent with the name, remote API model and url view API documentation, remote API needs to configure api_key.

If you are using a different URL API, change the deepseek-api component allowed_outbound_hosts value and open the app-config.json file in the app_data directory.

```json
{
  "id": "deepseek-api",
  "metadata": {
    "build": { },
    "allowed_outbound_hosts": [
      "http://self",
      "https://api.deepseek.com",
      "https://api.siliconflow.cn"
    ],

    "key_value_stores": [
      "default"
    ]
  },
  "source": {
    "content_type": "application/wasm",
    "source": "../../target/wasm32-wasip1/release/http_llm_local_handler_component.wasm"
  }
}
```


### Run

Windows Command-line run
```shell
ai-matrices-deepseek-windows-amd64.exe
```

Linux 命令行运行

```shell
./ai-matrices-deepseek-linux-amd64
```

![AIMatrices DeepSeek](../../docs/ai-matrices-deepseek/assets/20250215-ai-matrices-deepseek.png)

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
ai-matrices-deepseek-windows-amd64.exe -i 0.0.0.0 -p 8080 --user admin -- pass admin
```

## Other

AIMatrices DeepSeek Front end program use [chatgpt-web](https://github.com/Chanzhaoyu/chatgpt-web) project code.