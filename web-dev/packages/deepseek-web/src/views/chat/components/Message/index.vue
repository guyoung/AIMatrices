<script setup lang='ts'>
import { computed, ref } from 'vue'
import { NDropdown, useMessage } from 'naive-ui'
import AvatarComponent from './Avatar.vue'
import TextComponent from './Text.vue'

import { SvgIcon } from '@/components/common'
import { useIconRender } from '@/hooks/useIconRender'
import { t } from '@/locales'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { copyToClip } from '@/utils/copy'

import MarkdownEditor from '@/addons/markdown-editor/index.vue'

import { fetchPandocHandler } from "@/api"


interface Props {
  dateTime?: string
  text?: string
  think?: string
  inversion?: boolean
  error?: boolean
  loading?: boolean
}

interface Emit {
  (ev: 'regenerate'): void
  (ev: 'delete'): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

const { isMobile } = useBasicLayout()

const { iconRender } = useIconRender()

const message = useMessage()

const textRef = ref<HTMLElement>()

const asRawText = ref(props.inversion)

const messageRef = ref<HTMLElement>()

const showMarkdownEditor = ref(false)



const options = computed(() => {
  const common = [
    {
      label: t('chat.copy'),
      key: 'copyText',
      icon: iconRender({ icon: 'ri:file-copy-2-line' }),
    },
    {
      label: t('common.delete'),
      key: 'delete',
      icon: iconRender({ icon: 'ri:delete-bin-line' }),
    },
    {
      label: t('chat.menuEditMarkdown'),
      key: 'markdown',
      icon: iconRender({ icon: 'ri:markdown-line' }),
    },

    {
      label: t('chat.menuExport'),
      key: 'export',
      icon: iconRender({ icon: 'ri:download-line' }),
      children: [{
        label: 'Markdown',
        key: 'export-md',
        icon: iconRender({ icon: 'ri:download-line' }),
      },
      {
        label: 'Html',
        key: 'export-html',
        icon: iconRender({ icon: 'ri:download-line' }),
      },
      {
        label: 'Word',
        key: 'export-docx',
        icon: iconRender({ icon: 'ri:download-line' }),
      },
      {
        label: 'PowerPoint',
        key: 'export-pptx',
        icon: iconRender({ icon: 'ri:download-line' }),
      },
      {
        label: 'Latex',
        key: 'export-latex',
        icon: iconRender({ icon: 'ri:download-line' }),
      },
      ]
    },
  ]

  if (!props.inversion) {
    common.unshift({
      label: asRawText.value ? t('chat.preview') : t('chat.showRawText'),
      key: 'toggleRenderType',
      icon: iconRender({ icon: asRawText.value ? 'ri:code-line' : 'ri:code-box-line' }),
    })
  }

  return common
})

function handleSelect(key: string) {
  if (key == 'copyText') {
    handleCopy()
    return
  }

  if (key == 'toggleRenderType') {
    asRawText.value = !asRawText.value
    return
  }

  if (key == 'delete') {
    emit('delete')
    return
  }

  if (key == 'markdown') {
    showMarkdownEditor.value = true
    return
  }

  if (key.startsWith("export-")) {
    let format = key.replace("export-", "")

    if (format == "md") {
      handleExportMarkdown(props.text)

      return
    } else {
      handleExportPandoc(props.text, format)

      return
    }
  }
}

function handleRegenerate() {
  messageRef.value?.scrollIntoView()
  emit('regenerate')
}

async function handleCopy() {
  try {
    await copyToClip(props.text || '')
    message.success('复制成功')
  }
  catch {
    message.error('复制失败')
  }
}

function handleExportMarkdown(text: string) {
  const blob = new Blob([text], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);


  const a = document.createElement("a");
  a.href = url;
  a.download = "export.md";
  a.click();

  URL.revokeObjectURL(url);
}

async function handleExportPandoc(text: string, format: string) {
  let filename = "export." + format;

  try {

    let blob = await fetchPandocHandler(filename, text)

    const url = URL.createObjectURL(blob);


    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();

    URL.revokeObjectURL(url);

  }
  catch (error) {
    console.error("Error:", error);

  }
}



</script>

<template>
  <div ref="messageRef" class="flex w-full mb-6 overflow-hidden" :class="[{ 'flex-row-reverse': inversion }]">
    <div class="flex items-center justify-center flex-shrink-0 h-8 overflow-hidden rounded-full basis-8"
      :class="[inversion ? 'ml-2' : 'mr-2']">
      <AvatarComponent :image="inversion" />
    </div>
    <div class="overflow-hidden text-sm " :class="[inversion ? 'items-end' : 'items-start']">
      <p class="text-xs text-[#b4bbc4]" :class="[inversion ? 'text-right' : 'text-left']">
        {{ dateTime }}
      </p>
      <div class="flex items-end gap-1 mt-2" :class="[inversion ? 'flex-row-reverse' : 'flex-row']">

        <blockquote class="border-l-4 border-gray-400 pl-4 italic text-gray-600">
          {{ think }}
        </blockquote>

      </div>
      <div class="flex items-end gap-1 mt-2" :class="[inversion ? 'flex-row-reverse' : 'flex-row']">
        <TextComponent ref="textRef" :inversion="inversion" :error="error" :text="text" :loading="loading"
          :as-raw-text="asRawText" />
        <div class="flex flex-col">
          <button v-if="!inversion"
            class="mb-2 transition text-neutral-300 hover:text-neutral-800 dark:hover:text-neutral-300"
            @click="handleRegenerate">
            <SvgIcon icon="ri:restart-line" />
          </button>
          <NDropdown :trigger="isMobile ? 'click' : 'hover'" :placement="!inversion ? 'right' : 'left'"
            :options="options" @select="handleSelect">
            <button class="transition text-neutral-300 hover:text-neutral-800 dark:hover:text-neutral-200">
              <SvgIcon icon="ri:more-2-fill" />
            </button>
          </NDropdown>
        </div>
      </div>
    </div>
  </div>
  <MarkdownEditor :code="props.text" v-if="showMarkdownEditor" v-model:visible="showMarkdownEditor" />


</template>
