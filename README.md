<div align="center">

# AIMatrices

<p align="center">
AIMatrices is a lightweight, high-performance, scalable and open source AI application rapid construction platform, which aims to provide developers with efficient and convenient AI application development experience. It integrates a variety of advanced technologies and tools to help users quickly build, deploy, and maintain AI applications without writing complex code from scratch.

[English](README.md) / [简体中文](README_cn.md)
</p>

> [!WARNING]
> AIMatrices is an experimental package. It is subject to change and is only used for evaluation purposes.

</div>
<br />
<br />


## Features

**Out of the box**

Developed in the Rust programming language, it directly compiles into a binary executable file, and the resulting executable is about 40MB in size. There is no need to install any external dependencies, which greatly simplifies the deployment process, and it can run on multiple operating systems (such as Windows, macOS, Linux) without using containers.

**Broad hardware platform support**

It uses Rust FFI technology to call the high-performance lama.cpp library, and uses Vulkan as the back-end for model inference. It supports NVIDIA, AMD, Intel and other mainstream graphics cards, which can provide significant GPU acceleration effect for model inference. This architectural design allows the project to take full advantage of the powerful computing power of modern Gpus while maintaining good cross-platform compatibility.

**Secure and efficient**

Written in the Rust programming language, it ensures that the application runs safely and efficiently on different devices and operating systems. Rust's ownership system and lifecycle checking fundamentally eliminate common errors such as null Pointers, data races, and memory leaks, allowing the majority of potential memory issues to be found at compile time, resulting in fewer runtime errors and crashes. Rust's performance is close to C/C++ while avoiding the overhead of garbage collection mechanisms. It ensures extremely efficient code execution through zero-cost abstractions and efficient compiler optimizations.

**Flexible configuration and componentization**

The WebAssembly component technology is used to realize highly flexible component-based development by defining application configuration information such as components, routing rules, and environment variables through application configuration files. For example, you can configure allowed_outbound_hosts to control a component's access to external requests, or file mappings to enable static resource hosting.

**Extensibility and customization**

The OpenAI API specification is followed for model API calls, and the model calls and results are obtained through standardized interface parameters, such as input text, model selection, and temperature parameters. At the same time, it supports the use of JavaScript code for custom extension, allowing users to achieve personalized logic processing according to business needs.

**Open source**

AIMatrices is an open source project with free access to its code for developers to customize and extend.

## Use Cases

### [AIMatrices DeepSeek](https://github.com/guyoung/AIMatrices/tree/main/packages/ai-matrices-deepseek)

AIMatrices DeepSeek is an open source DeepSeek localization deployment tool based on AIMatrices, a lightweight AI application rapid construction platform, which supports flexible switching between local models and remote apis. This paper aims to provide users with an efficient, flexible and low-cost DeepSeek large model deployment solution.

![AIMatrices DeepSeek](docs/ai-matrices-deepseek/assets/20250215-ai-matrices-deepseek3.png)

### [AIMatrices Flux](https://github.com/guyoung/AIMatrices/tree/main/packages/ai-matrices-flux)

AIMatrices Flux is a lightweight AI application rapid construction platform AIMatrices using Flux model to develop open Vincenson diagram application, support prompt word automatic translation function.

![AIMatrices Flux](docs/ai-matrices-flux/assets/20250214-ai-matrices-flux.png)

## Documentation

## FAQ


## Getting Help


GitHub [Issues](https://github.com/guyoung/AIMatrices/issues) Submit bugs and feature requests

## License

 [Apache License 2.0](LICENSE)


