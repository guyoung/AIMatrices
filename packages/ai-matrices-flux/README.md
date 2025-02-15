AIMatrices Flux
=========================
AIMatrices Flux 是一款基于 AI 应用快速构建平台 AIMatrices 开发的开源 Flux 模型应用程序。

![AIMatrices Flux](../../docs/ai-matrices-deepseek/assets/20250214-ai-matrices-flux.png)

## 功能特性



## 快速开始

### 安装

#### 方式一：直接下载可执行文件

从 AIMatrices [GitHub Release](https://github.com/guyoung/AIMatrices/releases) 下载 ai-matrices-flux对应系统版本，解压后即可使用

#### 方式二：源码编译

编译前提条件是需要安装 Rust 环境

```shell

git clone https://github.com/guyoung/AIMatrices.git

## 编译 AIMatrices Flux 主程序
cd AIMatrices/packages/ai-matrices-flux
cagro build -- release

## 编译 AIMatrices WebAssembly 组件
cd AIMatrices/components/flux-handler-component
cargo component build --release
```



### 下载模型文件

* llama-translate
    * https://huggingface.co/dahara1/llama-translate-gguf/llama-translate.f16.Q4_K_M.gguf
    * mirror: https://hf-mirror.com/dahara1/llama-translate-gguf/llama-translate.f16.Q4_K_M.gguf


* Flux.1 Dev 
    * https://huggingface.co/city96/FLUX.1-dev-gguf/resolve/main/flux1-dev-Q4_0.gguf
    * mirror: https://hf-mirror.com/city96/FLUX.1-dev-gguf/resolve/main/flux1-dev-Q4_0.gguf

* vae 模型
    * https://huggingface.co/black-forest-labs/FLUX.1-dev/resolve/main/ae.safetensors
    * mirror: https://hf-mirror.com/black-forest-labs/FLUX.1-dev/resolve/main/ae.safetensors

* clip_l 模型  
    * https://huggingface.co/comfyanonymous/flux_text_encoders/resolve/main/clip_l.safetensors
    * mirror: https://hf-mirror.com/comfyanonymous/flux_text_encoders/resolve/main/clip_l.safetensors

* t5xxl 模型
    * https://huggingface.co/comfyanonymous/flux_text_encoders/resolve/main/t5xxl_fp8_e4m3fn.safetensors
    * mirror: https://hf-mirror.com/comfyanonymous/flux_text_encoders/resolve/main/t5xxl_fp8_e4m3fn.safetensors

### 修改配置文件


### 运行应用

Windows 命令行运行
```shell
ai-matrices-flux-windows-amd64.exe
```

Linux 命令行运行

```shell
./ai-matrices-flux-linux-amd64
```

命令行参数：
* -i,--ip <port_number>: 指定服务器监听的 IP 地址，默认为 127.0.0.0。
  * example: --ip 0.0.0.0
* -p,--port <port_number>: 指定服务器监听的端口号，默认为 21500。
  * example: --port 3000
* --appdir <appdir>: 指定工作目录，默认值为 ./app_data
  * example：--app-dir ./dir1
* --appconfig <appconfig>: 指定应用配置文件，默认值为 ./app_data/app-config.json
  * example：--appconfig ./dir1/config.json
* --user <username>: 指定登录用户名，默认值为空
  * example：--user admin
* --pass <password>: 指定登录用户密码，默认值为空
  * example：--pass admin
* --version: 打印系统版本号并退出。
* --help: 查看命令的使用帮助和参数说明。

```shell
ai-matrices-deepseek-flux-amd64.exe -i 0.0.0.0 -p 8080 --user admin -- pass admin
```

## 其它

AIMatrices Flux 前端程序使用 DeepSeek 生成。


