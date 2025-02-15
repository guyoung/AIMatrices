const http = require("http")
const openai = require('openai')

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

const default_model = "local/DeepSeek-R1"

const default_system_message = "You are DeepSeek, a large language model. Follow the user\'s instructions carefully."

function main(req) {
  let method = req.method
  let headers = req.headers
  let path = headers.get("req-path-info")



  if (method && method.toLowerCase() == "post" && path == "/chat") {
    return handle_chat(req)
  }

  if (method && method.toLowerCase() == "get" && path == "/list-models") {
    return handle_list_models(req)
  }

  return http.Response.not_found()
}

function handle_list_models(req) {
  let arr_models = []

  for (let k of Object.keys(models)) {
    let obj = models[k]
    arr_models.push({
      label: obj.name,
      key: k,
      value: k
    })
  }

  return http.Response.json({
    models: arr_models
  })

}

function handle_chat(req) {
  try {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let req_json = JSON.parse(req_body)
 

    let req_model = req_json.model??default_model

    if (!models[req_model]) {
      return http.Response.bad_request()
    }

    let cur_model = models[req_model]



    let system_message = req_json.systemMessage ?? default_system_message

    let prompt = req_json.prompt
    let max_tokens = req_json.max_tokens ?? 1000
    let temperature = req_json.temperature ?? 0.9
    let top_p = req_json.top_p ?? 1
    let presence_penalty = req_json.presence_penalty ?? 0.0
    let stream = req_json.stream

    const client = new openai.Client(cur_model.url, cur_model.api_key)

    let res = client.chat_completion({
      max_tokens: max_tokens,     
      model: cur_model.model,
      temperature: temperature,
      top_p: top_p,
      presence_penalty: presence_penalty,
      messages: [
        {
          role: 'system',
          content: system_message
        },
        { role: 'user', content: prompt, name: undefined }
      ],
      stream: stream
    })

    let text = Buffer.from(res).toString("utf-8")
    let json = JSON.parse(text)

    return http.Response.json(json)

  } catch (e) {
    console.log(e)
    return http.Response.internal_server_error(e)
  }
}



