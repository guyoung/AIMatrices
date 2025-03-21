<script lang="ts" setup>
import { computed, nextTick, onMounted, ref } from 'vue'
import { HoverButton, SvgIcon } from '@/components/common'
import { NSelect } from 'naive-ui'
import { useAppStore } from '@/store'
import { fetchModels } from '@/api'
import { useBasicLayout } from '@/hooks/useBasicLayout'


interface Props {
 
}

interface Emit {
  (ev: 'export'): void
  (ev: 'handleClear'): void
}

defineProps<Props>()

const emit = defineEmits<Emit>()

const appStore = useAppStore()

const { isMobile } = useBasicLayout()


const collapsed = computed(() => appStore.siderCollapsed)

function handleUpdateCollapsed() {
  appStore.setSiderCollapsed(!collapsed.value)
}

function onScrollToTop() {
  const scrollRef = document.querySelector('#scrollRef')
  if (scrollRef)
    nextTick(() => scrollRef.scrollTop = 0)
}

function handleExport() {
  emit('export')
}

function handleClear() {
  emit('handleClear')
}

const modelOptions = ref([])


onMounted(async() => {
  try {
    const data: any = await fetchModels()

    modelOptions.value = data.models  
  
  }
  finally {
   
  }
})

const model = computed({
   get() {
    if (appStore.model) {
      return appStore.model
    } else {   
      if (modelOptions.value && modelOptions.value.length > 0) {
        appStore.setModel(modelOptions.value[0].value)

        return modelOptions.value[0].value
      }
      
    }
    
  },
  set(value: String) {
    appStore.setModel(value)
  },
})

</script>

<template>
  <header
    class="sticky top-0 left-0 right-0 z-30 border-b dark:border-neutral-800 bg-white/80 dark:bg-black/20 backdrop-blur"
  >
    <div class="relative flex items-center justify-between min-w-0 overflow-hidden h-14">
      <div class="flex items-center">
        <button
          class="flex items-center justify-center w-11 h-11"
          @click="handleUpdateCollapsed"
        >
          <SvgIcon v-if="collapsed" class="text-2xl" icon="ri:align-justify" />
          <SvgIcon v-else class="text-2xl" icon="ri:align-right" />
        </button>
      </div>
      <h1
        class="flex-1 px-4 pr-6 overflow-hidden cursor-pointer select-none text-ellipsis whitespace-nowrap text-xl"
        @dblclick="onScrollToTop"
      >
      AIMatrices(DeepSeek)
      </h1>
      <div class="flex items-center space-x-2">                 
          <NSelect
            style="width: 240px; margin-right: 5px"
            :value="model"
            :options="modelOptions"
            @update-value="value => appStore.setModel(value)"
          />
      
        <HoverButton @click="handleExport"  v-if="isMobile">
          <span class="text-xl text-[#4f555e] dark:text-white">
            <SvgIcon icon="ri:download-2-line" />
          </span>
        </HoverButton>
        <HoverButton @click="handleClear"  v-if="isMobile">
          <span class="text-xl text-[#4f555e] dark:text-white">
            <SvgIcon icon="ri:delete-bin-line" />
          </span>
        </HoverButton>
      </div>
    </div>
  </header>
</template>
