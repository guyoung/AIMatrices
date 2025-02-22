const http = require("http")
const openai = require('openai')
const dbs = require('dbs');
const url = require('url');



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

const default_model = "local/deepseek-r1"

const default_system_message = "You are DeepSeek, a large language model. Follow the user\'s instructions carefully."

const chat_db_name = "chatdb"

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

  if (method && method.toLowerCase() == "post" && path == "/chat/add-conversation") {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let body = JSON.parse(req_body)

    return handle_table_create("conversations", body)
  }

  if (method && method.toLowerCase() == "post" && path == "/chat/update-conversation") {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let body = JSON.parse(req_body)

    return handle_table_update(body)
  }

  if (method && method.toLowerCase() == "post" && path == "/chat/delete-conversation") {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let body = JSON.parse(req_body)

    return handle_table_delete(body)
  }

  if (method && method.toLowerCase() == "post" && path == "/chat/delete-conversations") {
    let sql = `DELETE FROM conversations`

    return handle_query(sql)
  }

  if (method && method.toLowerCase() == "get" && path == "/chat/conversations") {
    const _url = url.URL.parse(req.url)

    let sql = `SELECT * FROM conversations ORDER BY seq_num DESC`

    return handle_query(sql)
  }


  if (method && method.toLowerCase() == "post" && path == "/chat/add-message") {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let body = JSON.parse(req_body)

    return handle_table_create("messages", body)
  }

  if (method && method.toLowerCase() == "post" && path == "/chat/update-message") {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let body = JSON.parse(req_body)

    return handle_table_update(body)
  }

  if (method && method.toLowerCase() == "post" && path.startsWith("/chat/batch-update-messages")) {

    const _url = url.URL.parse(req.url)
    let conversation = _url. searchParams.get("conversation")
    let loading = _url. searchParams.get("loading")

    let sql = `UPDATE messages SET loading=${loading}  WHERE conversation_id='${conversation}'`

    return handle_query(sql)
  }

  if (method && method.toLowerCase() == "post" && path == "/chat/delete-message") {
    let req_body = Buffer.from(req.body).toString("utf-8")
    let body = JSON.parse(req_body)

    return handle_table_delete(body)
  }

  if (method && method.toLowerCase() == "post" && path == "/chat/delete-messages") {
    const _url = url.URL.parse(req.url)
    let conversation = _url. searchParams.get("conversation")

    let sql = `DELETE FROM messages WHERE conversation_id='${conversation}'`

    return handle_query(sql)
  }


  if (method && method.toLowerCase() == "get" && path.startsWith("/chat/messages")) {

    const _url = url.URL.parse(req.url)
    let conversation = _url. searchParams.get("conversation")

    let sql = `SELECT * FROM messages WHERE conversation_id='${conversation}' ORDER BY seq_num`

    return handle_query(sql)
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

function handle_table_create(table, body) {
  try {
    const conn = new dbs.Connection(chat_db_name)

    let res = conn.create_sync(table, null, body)
    let text = Buffer.from(res).toString("utf-8")

    let data = JSON.parse(text)

    if (data.length > 0) {
      return http.Response.json(data[0])
    }
    return http.Response.not_found()
  } catch(e) {
    console.log(e)

    return http.Response.internal_server_error(e.message)
  }
}

function handle_table_update(body) {
  try {
    const conn = new dbs.Connection(chat_db_name)

    let table = body.id.split(":")[0]
    let id = body.id.split(":")[1]

    let res = conn.update_sync(table, id, body)
    let text = Buffer.from(res).toString("utf-8")

    let data = JSON.parse(text)

    if (data.length > 0) {
      return http.Response.json(data[0])
    }
    return http.Response.not_found()
  } catch(e) {
    console.log(e)

    return http.Response.internal_server_error(e.message)
  }
}

function handle_table_delete(body) {
  try {
    const conn = new dbs.Connection(chat_db_name)

    let table = body.id.split(":")[0]
    let id = body.id.split(":")[1]

    let res = conn.delete_sync(table, id)
    let text = Buffer.from(res).toString("utf-8")

    let data = JSON.parse(text)

    if (data.length > 0) {
      return http.Response.json(data[0])
    }
    return http.Response.not_found()
  } catch(e) {
    console.log(e)

    return http.Response.internal_server_error(e.message)
  }
}

function handle_table_get_all(table) {
  try {
    const conn = new dbs.Connection(chat_db_name)

    let res = conn.select_all_sync(table)
    let text = Buffer.from(res).toString("utf-8")

    let data = JSON.parse(text)

    if (data.length > 0) {

      return http.Response.json(data[0])
    }

    return http.Response.not_found()
  } catch(e) {
    console.log(e)

    return http.Response.internal_server_error(e.message)
  }
}

function handle_query(sql) {
  try {
    const conn = new dbs.Connection(chat_db_name)

    let res = conn.query_sync(sql)
    let text = Buffer.from(res).toString("utf-8")

    let data = JSON.parse(text)

    if (data.length > 0) {

      return http.Response.json(data[0])
    }

    return http.Response.not_found()
  } catch(e) {
    console.log(e)

    return http.Response.internal_server_error(e.message)
  }
}

