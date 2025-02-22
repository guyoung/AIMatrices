<script setup lang='ts'>
import { computed, onMounted, ref } from 'vue'
import { NSpin } from 'naive-ui'
/***
import { fetchChatConfig } from '@/api'

import { useAuthStore } from '@/store'
***/

interface ConfigState {

}

/***
const authStore = useAuthStore()
***/

const loading = ref(false)

const config = ref<ConfigState>()



async function fetchConfig() {
  try {
    loading.value = true
    /***
    const { data } = await fetchChatConfig<ConfigState>()
    ***/
    const data = {}
    
    config.value = data
  }
  finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchConfig()
})
</script>

<template>
  <NSpin :show="loading">
    <div class="p-4 space-y-4">
      <h2 class="text-xl font-bold">
        AIMatrices(DeepSeek)
      </h2>
      <div class="p-2 space-y-2 rounded-md bg-neutral-100 dark:bg-neutral-700">
        AIMatrices is a lightweight, high-performance, scalable, and open source AI application rapid building platform designed to provide developers with an efficient and convenient AI application development experience. 
      </div>
      <div class="p-2 space-y-2 rounded-md bg-neutral-100 dark:bg-neutral-700">
        Site: <a href="https://github.com/guyoung/AIMatrices" target="_blank">https://github.com/guyoung/AIMatrices</a>
      </div>
    </div>
  </NSpin>
</template>
