const http = require("http")
const openai = require('openai')
const dbs = require('dbs');
const url = require('url');

load('/codes/config.js')
load('/codes/history.js')

function main(req) {
  let method = req.method
  let headers = req.headers
  let path = headers.get("req-path-info")

  if (method && method.toLowerCase() == "post" && path == "/chat/process") {
    return handle_chat_process(req)
  }

  if (method && method.toLowerCase() == "get" && path == "/chat/list-models") {
    return handle_chat_list_models(req)
  }

  if (path.startsWith("/chat/history")) {
    return handle_chat_history(req)
  }


  return http.Response.not_found()
}

function handle_chat_list_models(req) {
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

function handle_chat_process(req) {
  try {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let req_json = JSON.parse(req_body) 

    let req_model = req_json.model??default_model

    if (!models[req_model]) {
      return http.Response.bad_request()
    }

    let cur_model = models[req_model]

    let system_message = req_json.systemMessage ?? default_system_message

    let messages = req_json.messages
    let max_tokens = req_json.max_tokens ?? 1000
    let temperature = req_json.temperature ?? 0.9
    let top_p = req_json.top_p ?? 1
    let presence_penalty = req_json.presence_penalty ?? 0.0
    let stream = req_json.stream

    const client = new openai.Client(cur_model.url, cur_model.api_key)


    let chat_req = {
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
       
      ].concat(messages),
      stream: stream
    };

    let res = client.chat_completion(chat_req)
    
    let text = Buffer.from(res).toString("utf-8")

    let json = JSON.parse(text)

    return http.Response.json(json)

  } catch (e) {
    console.log(e)
    return http.Response.internal_server_error(e.message)
  }
}

