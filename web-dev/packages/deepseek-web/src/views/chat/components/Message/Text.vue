<script lang="ts" setup>
import { computed, onMounted, onUnmounted, onUpdated, ref } from 'vue'
import MarkdownIt from 'markdown-it'
import mdKatex from '@traptitech/markdown-it-katex'
import mila from 'markdown-it-link-attributes'
import hljs from 'highlight.js'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { t } from '@/locales'
import { copyToClip } from '@/utils/copy'

import MarkdownEditor from '@/addons/markdown-editor/index.vue'
import MermaidEditor from '@/addons/mermaid-editor/index.vue'
import HtmlEditor from '@/addons/html-editor/index.vue'
import CodeRunner from '@/addons/code-runner/index.vue'


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

const showMarkdownEditor = ref(false)
const showMermaidEditor = ref(false)
const showCodeRunner = ref(false)
const showHtmlEditor = ref(false)
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

  if ((language && language.toLowerCase() == 'markdown') || (language2 && language2.toLowerCase() == 'markdown')) {
    html += `<span class="code-block-header__markdown hover:cursor-pointer">${language || language2}</span>`
  }
  else if ((language && language.toLowerCase() == 'mermaid') || (language2 && language2.toLowerCase() == 'mermaid')) {
    html += `<span class="code-block-header__mermaid hover:cursor-pointer">${language || language2}</span>`
  }
  else if ((language && language.toLowerCase() == 'html') || (language2 && language2.toLowerCase() == 'html')) {
    html += `<span class="code-block-header__html hover:cursor-pointer">${language || language2}</span>`
  }
  else if ((language && language.toLowerCase() == 'python') || (language2 && language2.toLowerCase() == 'python')) {
    html += `<span class="code-block-header__python hover:cursor-pointer">${language || language2}</span>`
  }
  else {
    html += `<span class="code-block-header__lang">${language}</span>`
  }

  html += `<span class="code-block-header__copy">${t('chat.copyCode')}</span>`

  html += `</div><code class="hljs code-block-body ${language}">${str}</code></pre>`
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
            btn.textContent = '复制成功'
            setTimeout(() => {
              btn.textContent = '复制代码'
            }, 1000)
          })
        }
      })
    })
  }
}

function addMarkdownEvents() {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll('.code-block-header__markdown')
    btns.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code = btn.parentElement?.nextElementSibling?.textContent
        if (code) {
          handledCode.value = code
          showMarkdownEditor.value = true
        }
      })
    })
  }
}

function addMermaidEvents() {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll('.code-block-header__mermaid')
    btns.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code = btn.parentElement?.nextElementSibling?.textContent
        if (code) {
          handledCode.value = code
          showMermaidEditor.value = true
        }
      })
    })
  }
}

function addHtmlEvents() {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll('.code-block-header__html')
    btns.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code = btn.parentElement?.nextElementSibling?.textContent
        if (code) {
          handledCode.value = code
          handledCodeLanage.value = "html"
          showHtmlEditor.value = true
        }
      })
    })
  }
}

function addCodeRunEvents() {
  if (textRef.value) {
    let btns = textRef.value.querySelectorAll('.code-block-header__python')
    btns.forEach((btn) => {
      btn.addEventListener('click', () => {
        const code = btn.parentElement?.nextElementSibling?.textContent
        if (code) {
          handledCode.value = code
          handledCodeLanage.value = "python"
          showCodeRunner.value = true
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

function removeMarkdownEvents() {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll('.code-block-header__markdown')
    btns.forEach((btn) => {
      btn.removeEventListener('click', () => { })
    })
  }
}

function removeMermaidEvents() {
  if (textRef.value) {
    const btns = textRef.value.querySelectorAll('.code-block-header__mermaid')
    btns.forEach((btn) => {
      btn.removeEventListener('click', () => { })
    })
  }
}

function removeHtmlEvents() {
  if (textRef.value) {
    let btns = textRef.value.querySelectorAll('.code-block-header_html')
    btns.forEach((btn) => {
      btn.removeEventListener('click', () => { })
    })
  }
}


function removeCodeRunEvents() {
  if (textRef.value) {
    let btns = textRef.value.querySelectorAll('.code-block-header__python')
    btns.forEach((btn) => {
      btn.removeEventListener('click', () => { })
    })
  }
}

onMounted(() => {
  addCopyEvents()
  addMarkdownEvents()
  addMermaidEvents()
  addHtmlEvents()
  addCodeRunEvents()
})

onUpdated(() => {
  addCopyEvents()
  addMarkdownEvents()
  addMermaidEvents()
  addHtmlEvents()
  addCodeRunEvents()
})

onUnmounted(() => {
  removeCopyEvents()
  removeMarkdownEvents()
  removeMermaidEvents()
  removeHtmlEvents()
  removeCodeRunEvents()
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
  <MarkdownEditor :code="handledCode" v-if="showMarkdownEditor" v-model:visible="showMarkdownEditor" />
  <MermaidEditor :code="handledCode"  v-if="showMermaidEditor" v-model:visible="showMermaidEditor" />
  <HtmlEditor :code="handledCode"   v-if="showHtmlEditor" v-model:visible="showHtmlEditor" />
  <CodeRunner :code="handledCode":language="handledCodeLanage"  v-if="showCodeRunner" v-model:visible="showCodeRunner" />
</template>

<style lang="less">
@import url(./style.less);
</style>
