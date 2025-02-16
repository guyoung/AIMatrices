AIMatrices
==============

AIMatrices 是一款轻量级、高性能、可扩展、开源的AI应用快速构建平台，旨在为开发者提供高效、便捷的 AI 应用开发体验。它通过集成多种先进的技术和工具，帮助用户快速搭建、部署和维护 AI 应用，无需从零开始编写复杂的代码。

> [!WARNING]
> AIMatrices 是一个实验性的软件包。它可能会发生变化，仅用于评估目的。

## 功能特性

**开箱即用**

使用 Rust 编程语言开发，直接编译成二进制可执行文件，生成的可执行文件大小约为 40MB左右，无需安装任何外部依赖库，极大地简化了部署过程，无需使用容器即可在多种操作系统（如 Windows、macOS、Linux）上运行。

**广泛硬件平台支持**

通过 Rust 的 FFI 技术调用高性能的 llama.cpp 库，并以 Vulkan 作为后端进行模型推理，支持 NVIDIA、AMD 、Intel 等多种主流显卡，能够为模型推理提供显著的GPU加速效果，这种架构设计使得项目能够充分利用现代 GPU 的强大计算能力，同时保持良好的跨平台兼容性。

**安全高效**

使用 Rust 编程语言开发，确保应用在不同设备和操作系统上都能安全高效运行。Rust 的所有权系统和生命周期检查机制从根本上杜绝了空指针、数据竞争、内存泄漏等常见错误，使得程序在编译阶段就能发现大部分潜在的内存问题，从而减少运行时错误和崩溃。Rust 的性能接近于 C/C++，同时避免了垃圾回收机制的开销。它通过零成本抽象和高效的编译器优化，确保代码执行效率极高。

**灵活配置与组件化**

采用 WebAssembly 组件技术，通过应用配置文件定义应用的组件、路由规则、环境变量等配置信息，实现高度灵活的组件化开发。例如，可通过配置 allowed_outbound_hosts 控制组件的外部请求权限，或通过文件映射实现静态资源托管。

**可扩展性与自定义能力**

遵循 OpenAI API 规范进行模型 API 调用，通过标准化的接口参数，如输入文本、模型选择、温度参数等，实现对模型的调用和结果获取。同时支持使用 JavaScript 代码进行自定义扩展，允许用户根据业务需求实现个性化的逻辑处理。



**完全开源**

AIMatrices 是一个开源项目，开发者可以自由访问其代码，进行定制和扩展。



## 文档

## 实际案例

* [AIMatrices DeepSeek](https://github.com/guyoung/AIMatrices/tree/main/packages/ai-matrices-deepseek)

![AIMatrices DeepSeek](docs/ai-matrices-deepseek/assets/20250215-ai-matrices-deepseek3.png)

* [AIMatrices Flux](https://github.com/guyoung/AIMatrices/tree/main/packages/ai-matrices-flux)

![AIMatrices Flux](docs/ai-matrices-deepseek/assets/20250214-ai-matrices-flux.png)

## 常见问题

## 获取帮助

GitHub [Issues](https://github.com/guyoung/AIMatrices/issues) 提交 bug 和功能请求

## 许可证

本项目采用 [Apache License 2.0](LICENSE)


