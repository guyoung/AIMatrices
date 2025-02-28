

function handle_chat_history(req) {
    let method = req.method
    let headers = req.headers
    let path = headers.get("req-path-info")

    if (method && method.toLowerCase() == "post" && path == "/chat/history/add-conversation") {
        let req_body = Buffer.from(req.body).toString("utf-8")
        let body = JSON.parse(req_body)

        return handle_history_create("conversations", body)
    }

    if (method && method.toLowerCase() == "post" && path == "/chat/history/update-conversation") {
        let req_body = Buffer.from(req.body).toString("utf-8")
        let body = JSON.parse(req_body)

        return handle_history_update(body)
    }

    if (method && method.toLowerCase() == "post" && path == "/chat/history/delete-conversation") {
        let req_body = Buffer.from(req.body).toString("utf-8")
        let body = JSON.parse(req_body)

        return handle_history_delete(body)
    }

    if (method && method.toLowerCase() == "post" && path == "/chat/history/delete-conversations") {
        let sql = `DELETE FROM conversations`

        return handle_history_query(sql)
    }

    if (method && method.toLowerCase() == "get" && path == "/chat/history/conversations") {
        const _url = url.URL.parse(req.url)

        let sql = `SELECT * FROM conversations ORDER BY seq_num DESC`

        return handle_history_query(sql)
    }


    if (method && method.toLowerCase() == "post" && path == "/chat/history/add-message") {
        let req_body = Buffer.from(req.body).toString("utf-8")
        let body = JSON.parse(req_body)

        return handle_history_create("messages", body)
    }

    if (method && method.toLowerCase() == "post" && path == "/chat/history/update-message") {
        let req_body = Buffer.from(req.body).toString("utf-8")
        let body = JSON.parse(req_body)

        return handle_history_update(body)
    }

    if (method && method.toLowerCase() == "post" && path.startsWith("/chat/history/batch-update-messages")) {

        const _url = url.URL.parse(req.url)
        let conversation_id = _url.searchParams.get("conversation")

        let loading = false
        let str_loading = _url.searchParams.get("loading")
        if (str_loading && str_loading.toLowerCase() == "true") {
            loading = true
        }

        let sql = `UPDATE messages SET loading=$loading  WHERE conversation_id=$conversation_id`

        return handle_history_query(sql, {
            conversation_id,
            loading
        })
    }

    if (method && method.toLowerCase() == "post" && path == "/chat/history/delete-message") {
        let req_body = Buffer.from(req.body).toString("utf-8")
        let body = JSON.parse(req_body)

        return handle_history_delete(body)
    }

    if (method && method.toLowerCase() == "post" && path == "/chat/history/delete-messages") {
        const _url = url.URL.parse(req.url)
        let conversation_id = _url. searchParams.get("conversation")

        let sql = `DELETE FROM messages WHERE conversation_id=$conversation_id`

        return handle_history_query(sql, {
            conversation_id,
        })
    }


    if (method && method.toLowerCase() == "get" && path.startsWith("/chat/history/messages")) {

        const _url = url.URL.parse(req.url)
        let conversation_id = _url. searchParams.get("conversation")

        let sql = `SELECT * FROM messages WHERE conversation_id=$conversation_id ORDER BY seq_num`

        return handle_history_query(sql, {
            conversation_id,
        })
    }

    return http.Response.not_found()

}


function handle_history_create(table, body) {
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

function handle_history_update(body) {
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

function handle_history_delete(body) {
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

function handle_history_get_all(table) {
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

function handle_history_query(sql, parmas) {
    try {
        const conn = new dbs.Connection(chat_db_name)

        let res = conn.query_sync(sql, parmas)
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

