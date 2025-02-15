import axios, { type AxiosResponse } from 'axios'
import { useAuthStore } from '@/store'

let service;

if (process.env.NODE_ENV === "development") {
  console.log("development: ", import.meta.env.VITE_CONFIG_API_PROXY_URL)
  service= axios.create({  
    baseURL: import.meta.env.VITE_CONFIG_API_PROXY_URL,
  })
} else {
  service= axios.create({  
    baseURL: import.meta.env.VITE_CONFIG_API_URL,
  })
}



service.interceptors.request.use(
  (config) => {
    const token = useAuthStore().token
    if (token)
      config.headers.Authorization = `Bearer ${token}`
    return config
  },
  (error) => {
    return Promise.reject(error.response)
  },
)

service.interceptors.response.use(
  (response: AxiosResponse): AxiosResponse => {
    if (response.status === 200)
      return response

    throw new Error(response.status.toString())
  },
  (error) => {
    return Promise.reject(error)
  },
)

export default service
