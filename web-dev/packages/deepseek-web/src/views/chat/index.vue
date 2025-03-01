<script setup lang='ts'>
import type { Ref } from 'vue'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { NAutoComplete, NButton, NInput, useDialog, useMessage } from 'naive-ui'
import html2canvas from 'html2canvas'
import { Message } from './components'
import { useScroll } from './hooks/useScroll'

import { useUsingContext } from './hooks/useUsingContext'
import HeaderComponent from './components/Header/index.vue'
import { HoverButton, SvgIcon } from '@/components/common'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { useChatStore, usePromptStore, useAppStore, useSettingStore } from '@/store'
import { fetchChatAPIProcess } from '@/api'
import { t } from '@/locales'

import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc'; // 导入 utc 插件
import timezone from 'dayjs/plugin/timezone'; // 导入 timezone 插件（可选）

dayjs.extend(utc);
dayjs.extend(timezone);




let controller = new AbortController()

const route = useRoute()
const dialog = useDialog()
const ms = useMessage()

const chatStore = useChatStore()

const { isMobile } = useBasicLayout()

const { scrollRef, scrollToBottom, scrollToBottomIfAtBottom } = useScroll()
const { usingContext, toggleUsingContext } = useUsingContext()

const { conversationID } = route.params as { conversationID: string }




const dataSources = computed(() => chatStore.messages)

const promptRef = ref<string>('')
const loadingRef = ref<boolean>(false)
const inputRef = ref<Ref | null>(null)

const conversationRef = ref<Chat.ChatConversation | null>(null)

const appStore = useAppStore()

const settingStore = useSettingStore()

// 添加PromptStore
const promptStore = usePromptStore()

// 使用storeToRefs，保证store修改后，联想部分能够重新渲染
const { promptList: promptTemplate } = storeToRefs<any>(promptStore)

// 未知原因刷新页面，loading 状态不会重置，手动重置




function handleSubmit() {
  onConversation()
}

async function onConversation() {
  let prompt = promptRef.value

  if (loadingRef.value)
    return

  if (!prompt || prompt.trim() === '')
    return

  controller = new AbortController()

  let prompt_append = covertPromptAppendHistory(prompt)

  if (chatStore.messages.length == 0 && conversationRef.value?.title == t('chat.newChat')) {

    let title = prompt;

    if (title.length > 20) {
      title = title.substring(0, 20) + "..."
    }

    conversationRef.value.title = title;

    await chatStore.updateConversation(conversationRef.value)
  }

  let request_message = await chatStore.addMessage(
    conversationID,
    {
      content: prompt,
      role: "user",
      seq_num: new Date().getTime(),

      inversion: true,
      error: false,
      loading: false,

    },
  )


  scrollToBottom()

  loadingRef.value = true
  promptRef.value = ''

  let assistant_message = await chatStore.addMessage(
    conversationID,
    {

      content: t('chat.thinking'),
      role: "assistant",
      seq_num: new Date().getTime(),
      request_id: request_message.id,

      inversion: false,
      error: false,
      loading: true,
    },
  )

  scrollToBottom()



  try {
    let options = {}

    const fetchChatAPIOnce = async () => {
      let data = await fetchChatAPIProcess<Chat.ChatResponse>({
        model: appStore.model,
        messages: prompt_append,
        options,
        signal: controller.signal,
      })

      assistant_message.content = data.choices[0].message.content
      assistant_message.inversion = false
      assistant_message.error = false
      assistant_message.loading = false

      await chatStore.updateMessage(assistant_message)

      if (data.choices[0].finish_reason === 'length') {
        //在UI中提示用户内容被截断，并提供“继续生成”选项
      }

      scrollToBottomIfAtBottom()
    }

    await fetchChatAPIOnce()
  }
  catch (error: any) {
    if (error.message === 'canceled') {
      assistant_message.loading = false

      await chatStore.updateMessage(assistant_message)

      scrollToBottomIfAtBottom()

      return
    }

    const errorMessage = error?.message ?? t('common.wrong')


    if (assistant_message.content !== '') {

      assistant_message.content = `${assistant_message.content}\n[${errorMessage}]`
      assistant_message.error = false
      assistant_message.loading = false

      await chatStore.updateMessage(assistant_message)

      return
    }

    assistant_message.content = errorMessage
    assistant_message.error = false
    assistant_message.error = true
    assistant_message.loading = false

    await chatStore.updateMessage(assistant_message)

    scrollToBottomIfAtBottom()

  }
  finally {
    loadingRef.value = false
  }


}

async function onRegenerate(assistant_message: Chat.ChatMessage) {
  if (loadingRef.value)
    return

  controller = new AbortController()

  let prompt = getAssistantPrompt(assistant_message)

  if (!prompt || prompt.trim() === '')
    return


  let prompt_append = covertPromptAppendHistory(prompt)

  loadingRef.value = true

  try {
    assistant_message.content = t('chat.thinking')
    assistant_message.inversion = false
    assistant_message.error = false
    assistant_message.loading = true

    await chatStore.updateMessage(assistant_message)

    let options = {}

    const fetchChatAPIOnce = async () => {

      let data = await fetchChatAPIProcess<Chat.ChatResponse>({
        model: appStore.model,
        messages: prompt_append,
        options,
        signal: controller.signal,
      })

      assistant_message.content = data.choices[0].message.content
      assistant_message.inversion = false
      assistant_message.error = false
      assistant_message.loading = false

      await chatStore.updateMessage(assistant_message)

      if (data.choices[0].finish_reason === 'length') {
        //在UI中提示用户内容被截断，并提供“继续生成”选项
      }


    }

    await fetchChatAPIOnce()
  }
  catch (error: any) {
    if (error.message === 'canceled') {
      assistant_message.loading = false

      await chatStore.updateMessage(assistant_message)

      return
    }

    const errorMessage = error?.message ?? t('common.wrong')

    if (assistant_message.content !== '') {

      assistant_message.content = `${assistant_message.content}\n[${errorMessage}]`
      assistant_message.error = false
      assistant_message.loading = false

      await chatStore.updateMessage(assistant_message)

      return
    }

    assistant_message.content = errorMessage
    assistant_message.error = false
    assistant_message.error = true
    assistant_message.loading = false

    await chatStore.updateMessage(assistant_message)
  }
  finally {
    loadingRef.value = false
  }

}

function handleExport() {
  if (loadingRef.value)
    return

  const d = dialog.warning({
    title: t('chat.exportImage'),
    content: t('chat.exportImageConfirm'),
    positiveText: t('common.yes'),
    negativeText: t('common.no'),
    onPositiveClick: async () => {
      try {
        d.loading = true
        const ele = document.getElementById('image-wrapper')
        const canvas = await html2canvas(ele as HTMLDivElement, {
          useCORS: true,
        })
        const imgUrl = canvas.toDataURL('image/png')
        const tempLink = document.createElement('a')
        tempLink.style.display = 'none'
        tempLink.href = imgUrl
        tempLink.setAttribute('download', 'chat-shot.png')
        if (typeof tempLink.download === 'undefined')
          tempLink.setAttribute('target', '_blank')

        document.body.appendChild(tempLink)
        tempLink.click()
        document.body.removeChild(tempLink)
        window.URL.revokeObjectURL(imgUrl)
        d.loading = false
        ms.success(t('chat.exportSuccess'))
        Promise.resolve()
      }
      catch (error: any) {
        ms.error(t('chat.exportFailed'))
      }
      finally {
        d.loading = false
      }
    },
  })
}

function handleDelete(item: Chat.ChatMessage) {
  if (loadingRef.value)
    return

  dialog.warning({
    title: t('chat.deleteMessage'),
    content: t('chat.deleteMessageConfirm'),
    positiveText: t('common.yes'),
    negativeText: t('common.no'),
    onPositiveClick: () => {
      chatStore.deleteMessage(item)
    },
  })
}

function handleClear() {
  if (loadingRef.value)
    return

  dialog.warning({
    title: t('chat.clearChat'),
    content: t('chat.clearChatConfirm'),
    positiveText: t('common.yes'),
    negativeText: t('common.no'),
    onPositiveClick: () => {
      chatStore.deleteMessages(conversationID)
    },
  })
}

function handleEnter(event: KeyboardEvent) {
  if (!isMobile.value) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault()
      handleSubmit()
    }
  }
  else {
    if (event.key === 'Enter' && event.ctrlKey) {
      event.preventDefault()
      handleSubmit()
    }
  }
}

function handleStop() {
  if (loadingRef.value) {
    controller.abort()
    loadingRef.value = false
  }
}



/***  ***/

const getAssistantPrompt = (assistant_message: Chat.ChatMessage): string => {
  let filters: any = chatStore.messages.filter(x => x.id == assistant_message.request_id)

  if (filters.length > 0 && filters[0].content) {
    return filters[0].content
  }

  return ""
}

const covertPromptAppendHistory = (prompt: String) => {


  let messages = []

  if (settingStore.multiRoundChat > 1) {
    let start = 0;

    if (chatStore.messages.length > (settingStore.multiRoundChat-1)*2) {
      start = chatStore.messages.length - (settingStore.multiRoundChat-1)*2
    }     

    for (let item of chatStore.messages.slice(start)) {
      messages.push({
        role: item.role,
        content: getTextPart(item.content)
      })
    }
  }



  messages.push({
    role: "user",
    content: prompt
  })

  return messages

}

function getTextPart(input: string) {
  if (!input) {
    return ""
  }


  const startTag = '<think>';
  const endTag = '</think>';

  if (input.includes(startTag) && input.includes(endTag)) {
    return input.replace(input.substring(input.indexOf(startTag), input.indexOf(endTag) + endTag.length), '').trim();
  }

  return input
}

function getThinkPart(input: string) {
  if (!input) {
    return ""
  }

  const startTag = '<think>';
  const endTag = '</think>';

  if (input.includes(startTag) && input.includes(endTag)) {
    const startIndex = input.indexOf(startTag) + startTag.length;
    const endIndex = input.indexOf(endTag);
    return input.substring(startIndex, endIndex).trim();
  }

  return undefined
}

function convertUpdatedDate(d: any) {
  const utcDate = dayjs.utc(d)
  const localDate = utcDate.local()

  return localDate.format('YYYY-MM-DD HH:mm:ss')

}



/*** ***/


// 可优化部分
// 搜索选项计算，这里使用value作为索引项，所以当出现重复value时渲染异常(多项同时出现选中效果)
// 理想状态下其实应该是key作为索引项,但官方的renderOption会出现问题，所以就需要value反renderLabel实现
const searchOptions = computed(() => {
  if (promptRef.value.startsWith('/')) {
    return promptTemplate.value.filter((item: { key: string }) => item.key.toLowerCase().includes(promptRef.value.substring(1).toLowerCase())).map((obj: { value: any }) => {
      return {
        label: obj.value,
        value: obj.value,
      }
    })
  }
  else {
    return []
  }
})

// value反渲染key
const renderOption = (option: { label: string }) => {
  for (const i of promptTemplate.value) {
    if (i.value === option.label)
      return [i.key]
  }
  return []
}

const placeholder = computed(() => {
  if (isMobile.value)
    return t('chat.placeholderMobile')
  return t('chat.placeholder')
})

const buttonDisabled = computed(() => {
  return !conversationID || loadingRef.value || !promptRef.value || promptRef.value.trim() === ''
})

const footerClass = computed(() => {
  let classes = ['p-4']
  if (isMobile.value)
    classes = ['sticky', 'left-0', 'bottom-0', 'right-0', 'p-2', 'pr-3', 'overflow-hidden']
  return classes
})



onMounted(async () => {
  await chatStore.fetchConversations()

  if (chatStore.conversations.length == 0) {
    await chatStore.addConversation({ title: t('chat.newChat'), isEdit: false, seq_num: new Date().getTime() })
  } 


  await activeConversation()

  await updateMessages()
 

  scrollToBottom()

  if (inputRef.value && !isMobile.value)
    inputRef.value?.focus()
})

onUnmounted(() => {
  if (loadingRef.value)
    controller.abort()
})
/*** ***/
async function activeConversation() {
  if (!route.params.conversationID) {
    await chatStore.setActiveConversation(chatStore.conversations[0].id)
  } 

  const filters = chatStore.conversations.filter(x => x.id == chatStore.active);

  if (filters.length == 0) {
    await chatStore.setActiveConversation(chatStore.conversations[0].id)

    conversationRef.value = chatStore.conversations[0]
  } else {
    conversationRef.value = filters[0]
  } 
}

async function updateMessages() {
  await chatStore.batchUpdateMessages(chatStore.active, false)
}

/*** ***/

</script>

<template>
  <div class="flex flex-col w-full h-full">
    <HeaderComponent @export="handleExport" @handle-clear="handleClear" />
    <main class="flex-1 overflow-hidden">
      <div id="scrollRef" ref="scrollRef" class="h-full overflow-hidden overflow-y-auto">
        <div id="image-wrapper" class="w-full max-w-screen-xl m-auto dark:bg-[#101014]"
          :class="[isMobile ? 'p-2' : 'p-4']">
          <template v-if="!dataSources.length">
            <div class="flex items-center justify-center mt-4 text-center text-neutral-300">
              <SvgIcon icon="ri:bubble-chart-fill" class="mr-2 text-3xl" />
              <span>Aha~</span>
            </div>
          </template>
          <template v-else>
            <div>
              <Message v-for="(item, index) of dataSources" :key="index" :date-time="convertUpdatedDate(item.updated_at)"
                :text="getTextPart(item.content)" :think="getThinkPart(item.content)" :inversion="item.inversion"
                :error="item.error" :loading="item.loading" @regenerate="onRegenerate(item)"
                @delete="handleDelete(item)" />
              <div class="sticky bottom-0 left-0 flex justify-center">
                <NButton v-if="loadingRef" type="warning" @click="handleStop">
                  <template #icon>
                    <SvgIcon icon="ri:stop-circle-line" />
                  </template>
                  {{ t('common.stopResponding') }}
                </NButton>

              </div>
            </div>
          </template>
        </div>
      </div>
    </main>
    <footer :class="footerClass">
      <div class="w-full max-w-screen-xl m-auto">
        <div class="flex items-center justify-between space-x-2">
          <HoverButton v-if="!isMobile" @click="handleClear">
            <span class="text-xl text-[#4f555e] dark:text-white">
              <SvgIcon icon="ri:delete-bin-line" />
            </span>
          </HoverButton>
          <HoverButton v-if="!isMobile" @click="handleExport">
            <span class="text-xl text-[#4f555e] dark:text-white">
              <SvgIcon icon="ri:download-2-line" />
            </span>
          </HoverButton>
          <!--
          <HoverButton @click="toggleUsingContext">
            <span class="text-xl" :class="{ 'text-[#4b9e5f]': usingContext, 'text-[#a8071a]': !usingContext }">
              <SvgIcon icon="ri:chat-history-line" />
            </span>
          </HoverButton>
          -->
          <NAutoComplete v-model:value="promptRef" :options="searchOptions" :render-label="renderOption">
            <template #default="{ handleInput, handleBlur, handleFocus }">
              <NInput ref="inputRef" v-model:value="promptRef" type="textarea" :placeholder="placeholder"
                :autosize="{ minRows: 1, maxRows: isMobile ? 4 : 8 }" @input="handleInput" @focus="handleFocus"
                @blur="handleBlur" @keypress="handleEnter" />
            </template>
          </NAutoComplete>
          <NButton type="primary" :disabled="buttonDisabled" @click="handleSubmit">
            <template #icon>
              <span class="dark:text-black">
                <SvgIcon icon="ri:send-plane-fill" />
              </span>
            </template>
          </NButton>
        </div>
      </div>
    </footer>
  </div>
</template>
