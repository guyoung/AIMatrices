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
    prompt: string
    options?: { conversationId?: string; parentMessageId?: string }
    signal?: GenericAbortSignal
  }
) {
  const settingStore = useSettingStore()
  
  let data: Record<string, any> = {
    model: params.model,
    prompt: params.prompt,
    options: params.options,
  }


  data = {
    ...data,
    systemMessage: settingStore.systemMessage,
    max_tokens: settingStore.max_tokens,
    temperature: settingStore.temperature,
    top_p: settingStore.top_p,
  }


  return post<T>({
    url: '/chat',
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
    url: '/list-models',
  })
}