<script lang="ts" setup>
import { computed, onMounted, onUnmounted, onUpdated, ref } from 'vue'
import MarkdownIt from 'markdown-it'
import mdKatex from '@traptitech/markdown-it-katex'
import mila from 'markdown-it-link-attributes'
import hljs from 'highlight.js'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { t } from '@/locales'
import { copyToClip } from '@/utils/copy'
import { useSettingStore } from '@/store'

import MarkdownEditor from '@/addons/markdown-editor/index.vue'
import MermaidEditor from '@/addons/mermaid-editor/index.vue'
import HtmlEditor from '@/addons/html-editor/index.vue'
import CodeRunner from '@/addons/code-runner/index.vue'
import GraphvizEditor from '@/addons/graphviz-editor/index.vue'


interface Props {
  inversion?: boolean
  error?: boolean
  text?: string
  loading?: boolean
  asRawText?: boolean
}

const props = defineProps<Props>()

const { isMobile } = useBasicLayout()

const textRef = ref<HTMLElement>()

const settingStore = useSettingStore()

const previewComponents: any = {}

for (let language of settingStore.previewLanguages) {
  previewComponents[language] = false
}



const previewComponentsRef = ref(previewComponents)
const handledCode = ref("")
const handledCodeLanage = ref("")

const mdi = new MarkdownIt({
  html: false,
  linkify: true,
  highlight(code, language) {

    const validLang = !!(language && hljs.getLanguage(language))
    if (validLang) {
      const lang = language ?? ''
      return highlightBlock(hljs.highlight(code, { language: lang }).value, lang, language)
    }
    return highlightBlock(hljs.highlightAuto(code).value, '', language)
  },
})

mdi.use(mila, { attrs: { target: '_blank', rel: 'noopener' } })
mdi.use(mdKatex, { blockClass: 'katexmath-block rounded-md p-[10px]', errorColor: ' #cc0000' })

const wrapClass = computed(() => {
  return [
    'text-wrap',
    'min-w-[20px]',
    'rounded-md',
    isMobile.value ? 'p-2' : 'px-3 py-2',
    props.inversion ? 'bg-[#d2f9d1]' : 'bg-[#f4f6f8]',
    props.inversion ? 'dark:bg-[#a1dc95]' : 'dark:bg-[#1e1e20]',
    props.inversion ? 'message-request' : 'message-reply',
    { 'text-red-500': props.error },
  ]
})

const text = computed(() => {
  const value = props.text ?? ''

  if (!props.asRawText)
    return mdi.render(value)
  return value
})



function highlightBlock(str: string, language?: string, language2?: string) {
  let html = `<pre class="code-block-wrapper"><div class="code-block-header">`

  let codeLanguage = language || language2

  if (codeLanguage && settingStore.previewLanguages.includes(codeLanguage.toLowerCase())) {
    html += `<span class="code-block-header__${codeLanguage} hover:cursor-pointer">${codeLanguage}</span>`
  } else {
    html += `<span class="code-block-header__lang">${codeLanguage}</span>`
  }

  html += `<span class="code-block-header__copy">${t('chat.copyCode')}</span>`

  html += `</div><code class="hljs code-block-body ${codeLanguage}">${str}</code></pre>`

  return html
}

function addCopyEvents() {
  if (textRef.value) {
    const copyBtn = textRef.value.querySelectorAll('.code-block-header__copy')
    copyBtn.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code = btn.parentElement?.nextElementSibling?.textContent
        if (code) {
          copyToClip(code).then(() => {
            btn.textContent = t('chat.copied')
            setTimeout(() => {
              btn.textContent = t('chat.copyCode')
            }, 1000)
          })
        }
      })
    })
  }
}

function removeCopyEvents() {
  if (textRef.value) {
    const copyBtn = textRef.value.querySelectorAll('.code-block-header__copy')
    copyBtn.forEach((btn) => {
      btn.removeEventListener('click', () => { })
    })
  }
}

function addPreviewEvents(language: string) {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll(`.code-block-header__${language}`)
    btns.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code = btn.parentElement?.nextElementSibling?.textContent
        if (code) {
          handledCode.value = code
          previewComponentsRef.value[language] = true
        }
      })
    })
  }
}



function removePreviewEvents(language: string) {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll(`.code-block-header__${language}`)
    btns.forEach((btn) => {
      btn.removeEventListener('click', () => { })
    })
  }
}


onMounted(() => {
  addCopyEvents()

  for (let language of settingStore.previewLanguages) {
    addPreviewEvents(language)
  }
})

onUpdated(() => {
  addCopyEvents()

  for (let language of settingStore.previewLanguages) {
    addPreviewEvents(language)
  }

})

onUnmounted(() => {
  removeCopyEvents()
  for (let language of settingStore.previewLanguages) {
    removePreviewEvents(language)
  }
})
</script>

<template>
  <div class="text-black" :class="wrapClass">
    <div ref="textRef" class="leading-relaxed break-words">
      <div v-if="!inversion">
        <div v-if="!asRawText" class="markdown-body" :class="{ 'markdown-body-generate': loading }" v-html="text" />
        <div v-else class="whitespace-pre-wrap" v-text="text" />
      </div>
      <div v-else class="whitespace-pre-wrap" v-text="text" />
    </div>
  </div>
  <MarkdownEditor :code="handledCode" v-if="previewComponentsRef.markdown" v-model:visible="previewComponentsRef.markdown" />
  <MermaidEditor :code="handledCode" v-if="previewComponentsRef.mermaid" v-model:visible="previewComponentsRef.mermaid" />
  <HtmlEditor :code="handledCode" v-if="previewComponentsRef.html" v-model:visible="previewComponentsRef.html" />
  <GraphvizEditor :code="handledCode"  v-if="previewComponentsRef.dot"
    v-model:visible="previewComponentsRef.dot" />
  <CodeRunner :code="handledCode" :language="handledCodeLanage" v-if="previewComponentsRef.python"
    v-model:visible="previewComponentsRef.python" />
</template>

<style lang="less">
@import url(./style.less);
</style>
