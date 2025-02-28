const models = {
    "local/deepseek-r1": {
        name: "DeepSeek-R1(本地)",
        model: "local/DeepSeek-R1",
        local: true,
        url: "/service/deepseek-local",
        api_key: ""
    },
    "ollama-qwen": {
        name: "Ollama qwen2.5",
        model: "qwen2.5",
        local: false,
        url: "/service/ollama-api",
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

const default_model = "local/deepseek-r1"

const default_system_message = "You are DeepSeek, a large language model. Follow the user\'s instructions carefully."

const chat_db_name = "chatdb"