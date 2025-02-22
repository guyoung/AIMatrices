<script setup lang='ts'>
import { computed } from 'vue'
import { NInput, NPopconfirm, NScrollbar } from 'naive-ui'
import { SvgIcon } from '@/components/common'
import { useAppStore, useChatStore } from '@/store'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { debounce } from '@/utils/functions/debounce'
import { t } from '@/locales'

const { isMobile } = useBasicLayout()

const appStore = useAppStore()
const chatStore = useChatStore()

const dataSources = computed(() => chatStore.conversations)

async function handleSelect(conversation: Chat.ChatConversation) {
  if (isActive(conversation.id))
    return

  if (chatStore.active) {
     conversation.isEdit = false

    await chatStore.updateConversation(conversation)

  }    

  await chatStore.setActiveConversation(conversation.id)

  if (isMobile.value)
    appStore.setSiderCollapsed(true)
}

async function handleEdit(conversation: Chat.ChatConversation, isEdit: boolean, event?: MouseEvent) {
  event?.stopPropagation()

  conversation.isEdit = isEdit

  await chatStore.updateConversation(conversation)

}

async function handleDelete(conversation: Chat.ChatConversation, event?: MouseEvent | TouchEvent) {
  event?.stopPropagation()

  await chatStore.deleteMessages(conversation.id)

  await chatStore.deleteConversation(conversation)

  if (chatStore.conversations.length > 0) {
    await chatStore.setActiveConversation(chatStore.conversations[0].id)
  } else {
    let conversation = await chatStore.addConversation({ title: t('chat.newChat'), isEdit: false, seq_num: new Date().getTime() })

    if (conversation) {
      chatStore.setActiveConversation(conversation.id)
    }
  }


  if (isMobile.value)
    appStore.setSiderCollapsed(true)
}

const handleDeleteDebounce = debounce(handleDelete, 600)

async function handleEnter(conversation: Chat.ChatConversation, isEdit: boolean, event: KeyboardEvent) {
  event?.stopPropagation()

  if (event.key === 'Enter') {

    conversation.isEdit = isEdit

    await chatStore.updateConversation(conversation)
 
  }
}

function isActive(id: string | null) {
  return chatStore.active === id
}
</script>

<template>
  <NScrollbar class="px-4">
    <div class="flex flex-col gap-2 text-sm">
      <template v-if="!dataSources.length">
        <div class="flex flex-col items-center mt-4 text-center text-neutral-300">
          <SvgIcon icon="ri:inbox-line" class="mb-2 text-3xl" />
          <span>{{ $t('common.noData') }}</span>
        </div>
      </template>
      <template v-else>
        <div v-for="(item, index) of dataSources" :key="index">
          <a
            class="relative flex items-center gap-3 px-3 py-3 break-all border rounded-md cursor-pointer hover:bg-neutral-100 group dark:border-neutral-800 dark:hover:bg-[#24272e]"
            :class="isActive(item.id) && ['border-[#4b9e5f]', 'bg-neutral-100', 'text-[#4b9e5f]', 'dark:bg-[#24272e]', 'dark:border-[#4b9e5f]', 'pr-14']"
            @click="handleSelect(item)"
          >
            <span>
              <SvgIcon icon="ri:message-3-line" />
            </span>
            <div class="relative flex-1 overflow-hidden break-all text-ellipsis whitespace-nowrap">
              <NInput
                v-if="item.isEdit"
                v-model:value="item.title" size="tiny"
                @keypress="handleEnter(item, false, $event)"
              />
              <span v-else>{{ item.title }}</span>
            </div>
            <div v-if="isActive(item.id)" class="absolute z-10 flex visible right-1">
              <template v-if="item.isEdit">
                <button class="p-1" @click="handleEdit(item, false, $event)">
                  <SvgIcon icon="ri:save-line" />
                </button>
              </template>
              <template v-else>
                <button class="p-1">
                  <SvgIcon icon="ri:edit-line" @click="handleEdit(item, true, $event)" />
                </button>
                <NPopconfirm placement="bottom" @positive-click="handleDeleteDebounce(item, $event)">
                  <template #trigger>
                    <button class="p-1">
                      <SvgIcon icon="ri:delete-bin-line" />
                    </button>
                  </template>
                  {{ $t('chat.deleteHistoryConfirm') }}
                </NPopconfirm>
              </template>
            </div>
          </a>
        </div>
      </template>
    </div>
  </NScrollbar>
</template>
