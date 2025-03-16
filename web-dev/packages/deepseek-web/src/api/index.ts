import type { AxiosProgressEvent, GenericAbortSignal } from 'axios'
import { get, post } from '@/utils/request'
import { useAuthStore, useSettingStore } from '@/store'

/***
export function fetchChatAPI<T = any>(
  prompt: string,
  options?: { conversationId?: string; parentMessageId?: string },
  signal?: GenericAbortSignal,
) {
  return post<T>({
    url: '/chat',
    data: { prompt, options },
    signal,
  })
}
***/

/***
export function fetchChatConfig<T = any>() {
  return post<T>({
    url: '/config',
  })
}
***/

/***
export function fetchChatAPIProcess<T = any>(
  params: {
    prompt: string
    options?: { conversationId?: string; parentMessageId?: string }
    signal?: GenericAbortSignal
    onDownloadProgress?: (progressEvent: AxiosProgressEvent) => void },
) {
  const settingStore = useSettingStore()
  const authStore = useAuthStore()

  let data: Record<string, any> = {
    prompt: params.prompt,
    options: params.options,
  }

  if (authStore.isChatGPTAPI) {
    data = {
      ...data,
      systemMessage: settingStore.systemMessage,
      temperature: settingStore.temperature,
      top_p: settingStore.top_p,
    }
  }

  return post<T>({
    url: '/chat-process',
    data,
    signal: params.signal,
    onDownloadProgress: params.onDownloadProgress,
  })
}
***/

export function fetchChatAPIProcess<T = any>(
  params: {
    model: String,
    messages: any
    options?: { conversationId?: string; parentMessageId?: string }
    signal?: GenericAbortSignal
  }
) {
  const settingStore = useSettingStore()

  let data: Record<string, any> = {
    model: params.model,
    messages: params.messages,
    options: params.options,
  }


  data = {
    ...data,
    systemMessage: settingStore.systemMessage,
    max_tokens: settingStore.maxTokens,
    temperature: settingStore.temperature,
    top_p: settingStore.top_p,
  }


  return post<T>({
    url: '/service/deepseek-api/chat/process',
    data,
    signal: params.signal,
  })
}

/***
export function fetchSession<T>() {
  return post<T>({
    url: '/session',
  })
}
***/

/***
export function fetchVerify<T>(token: string) {
  return post<T>({
    url: '/verify',
    data: { token },
  })
}
***/

export function fetchModels<T = any>() {
  return get<T>({
    url: '/service/deepseek-api/chat/list-models',
  })
}


export async function fetchCodeHandler(
  data: string,
  language: string
) {

  let url;

  if (process.env.NODE_ENV === "development") {
    url = import.meta.env.VITE_CONFIG_API_PROXY_URL + 'service/python-code-handler'
  } else {
    url = import.meta.env.VITE_CONFIG_API_URL + 'service/python-code-handler'
  }

  const res = await fetch(url, {
    method: "POST",
    body: data
  })

  let text = res.text()

  return text
}

export async function fetchGraphvizHandler(
  data: string,
) {

  let url;

  if (process.env.NODE_ENV === "development") {
    url = import.meta.env.VITE_CONFIG_API_PROXY_URL + 'service/graphviz-handler'
  } else {
    url = import.meta.env.VITE_CONFIG_API_URL + 'service/graphviz-handler'
  }

  const res = await fetch(url, {
    method: "POST",
    body: data
  })

  let text = res.text()

  return text
}

export async function fetchPandocHandler(
  filename: string,
  data: string, 
) {

  let url;

  if (process.env.NODE_ENV === "development") {
    url = import.meta.env.VITE_CONFIG_API_PROXY_URL + 'service/pandoc-handler?-o='+filename
  } else {
    url = import.meta.env.VITE_CONFIG_API_URL + 'service/pandoc-handler?-o='+filename
  }

  const res = await fetch(url, {
    method: "POST",
    body: data
  })

  let text = res.blob()

  return text
}