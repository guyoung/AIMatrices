<template>
    <div v-if="props.visible" class="mask">
        <div class="mask-content">
            <NCard title="Mermaid" hoverable closable @close="handleClose">
                <template #header-extra>         
                    <HoverButton  @click="zoom(1.2)">
                        <span class="text-xl text-[#4f555e] dark:text-white">
                            <SvgIcon icon="ri:zoom-in-line" />
                        </span>
                    </HoverButton>
                    <HoverButton  @click="zoom(0.8)">
                        <span class="text-xl text-[#4f555e] dark:text-white">
                            <SvgIcon icon="ri:zoom-out-line" />
                        </span>
                    </HoverButton>
                    <HoverButton  @click="saveAsMarkdown(props.content)">
                        <span class="text-xl text-[#4f555e] dark:text-white">
                            <SvgIcon icon="ri:markdown-line" />
                        </span>
                    </HoverButton>
                    <HoverButton  @click="saveAsPng()">
                        <span class="text-xl text-[#4f555e] dark:text-white">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 15 15" aria-hidden="true" width="15" height="15">
                                <path d="M2.5 6.5V6H2v.5zm8 4H10v.5h.5zm2 0v.5h.5v-.5zm1-7h.5v-.207l-.146-.147zm-3-3l.354-.354L10.707 0H10.5zm-4 6l.447-.224L6 6.5zm-.5 4v.5h1v-.5zm2.5 0l-.447.224A.5.5 0 0 0 9 10.5zm.5-4V6H8v.5zM2.5 7h1V6h-1zm.5 4V8.5H2V11zm0-2.5v-2H2v2zm.5-.5h-1v1h1zm.5-.5a.5.5 0 0 1-.5.5v1A1.5 1.5 0 0 0 5 7.5zM3.5 7a.5.5 0 0 1 .5.5h1A1.5 1.5 0 0 0 3.5 6zM10 6v4.5h1V6zm.5 5h2v-1h-2zm2.5-.5v-2h-1v2zM10.5 7H13V6h-2.5zM2 5V1.5H1V5zm11-1.5V5h1V3.5zM2.5 1h8V0h-8zm7.646-.146l3 3l.708-.708l-3-3zM2 1.5a.5.5 0 0 1 .5-.5V0A1.5 1.5 0 0 0 1 1.5zM1 12v1.5h1V12zm1.5 3h10v-1h-10zM14 13.5V12h-1v1.5zM12.5 15a1.5 1.5 0 0 0 1.5-1.5h-1a.5.5 0 0 1-.5.5zM1 13.5A1.5 1.5 0 0 0 2.5 15v-1a.5.5 0 0 1-.5-.5zm5-7v4h1v-4zm.053.224l2 4l.894-.448l-2-4zM8 6.5v4h1v-4z"/>
                            </svg>
                        </span>
                    </HoverButton>
                    <HoverButton  @click="saveAsSvg()">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 15 15" aria-hidden="true" width="15" height="15">
                            <path d="M13.5 3.5h.5v-.207l-.146-.147zm-3-3l.354-.354L10.707 0H10.5zm-8 6V6H2v.5zm0 2H2V9h.5zm2 0H5V8h-.5zm0 2v.5H5v-.5zm2-1H6v.207l.146.147zm1 1l-.354.354l.354.353l.354-.353zm1-1l.354.354L9 9.707V9.5zm2-3V6H10v.5zm0 4H10v.5h.5zm2 0v.5h.5v-.5zM2 5V1.5H1V5zm11-1.5V5h1V3.5zM2.5 1h8V0h-8zm7.646-.146l3 3l.708-.708l-3-3zM2 1.5a.5.5 0 0 1 .5-.5V0A1.5 1.5 0 0 0 1 1.5zM1 12v1.5h1V12zm1.5 3h10v-1h-10zM14 13.5V12h-1v1.5zM12.5 15a1.5 1.5 0 0 0 1.5-1.5h-1a.5.5 0 0 1-.5.5zM1 13.5A1.5 1.5 0 0 0 2.5 15v-1a.5.5 0 0 1-.5-.5zM5 6H2.5v1H5zm-3 .5v2h1v-2zM2.5 9h2V8h-2zM4 8.5v2h1v-2zm.5 1.5H2v1h2.5zM6 6v3.5h1V6zm.146 3.854l1 1l.708-.708l-1-1zm1.708 1l1-1l-.708-.708l-1 1zM9 9.5V6H8v3.5zM13 6h-2.5v1H13zm-3 .5v4h1v-4zm.5 4.5h2v-1h-2zm2.5-.5v-2h-1v2z"/>
                            </svg>
                    </HoverButton>
                </template>
                <div class="zoom-container">
                    <VueMermaidString class="mermaid-el" :value="props.content" />
                </div>
            </NCard>
        </div>
    </div>
</template>



<script setup lang='ts'>
import type { CSSProperties } from 'vue'
import { onMounted, computed, defineProps, ref } from 'vue'

import { NCard, NButton} from 'naive-ui'

import { HoverButton, SvgIcon } from '@/components/common'

// NCard, NDataTable, NDivider, NInput, NList, NListItem,
// NPopconfirm, NSpace, NTabPane, NTabs, NThing, useMessage

import VueMermaidString from "vue-mermaid-string";

import { encode } from 'js-base64';

const code1 = `
graph TD
    A[Python 入门] --> B[基础概念]
    A --> C[语法结构]
    A --> D[数据类型]
    A --> E[控制流程]
    A --> F[函数]
    A --> G[模块与包]
    A --> H[面向对象编程]
    A --> I[异常处理]
    A --> J[文件操作]
    A --> K[常用库]
`

let scale = 1

interface Props {
    visible: boolean,
    content: string
}

interface Emit {
    (e: 'update:visible', visible: boolean): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()


const handleClose = () => {
    emit('update:visible', false)
}

function zoom(factor) {
    scale *= factor;
    document.querySelector('.mermaid-el svg').style.transform = `scale(${scale})`;
  }

function saveAsMarkdown(text: string) {
  const blob = new Blob([text], { type: "text/plain;charset=utf-8" });
  const url = URL.createObjectURL(blob);


  const a = document.createElement("a");
  a.href = url;
  a.download = "download.md"; 
  a.click(); 

  URL.revokeObjectURL(url);
}



function saveAsPng() {
  const svgElement = document.querySelector('.mermaid-el svg')
  const svgData = new XMLSerializer().serializeToString(svgElement)
  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  
  const svgSize = svgElement.getBoundingClientRect()
  canvas.width = svgSize.width * 2
  canvas.height = svgSize.height * 2


  const img = new Image();
  img.onload = () => {
    ctx.drawImage(img, 0, 0, canvas.width, canvas.height)
    const link = document.createElement('a')
    link.download = 'download.png'
    link.href = canvas.toDataURL('image/png')
    link.click()
  };

  img.src = 'data:image/svg+xml;base64,' + encode(svgData)
}

function saveAsSvg() {
  const svgElement = document.querySelector('.mermaid-el svg')
  const svgData = new XMLSerializer().serializeToString(svgElement)
  const link = document.createElement('a')
  link.download = 'download.svg'
  link.href = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(svgData)
  link.click()
}



</script>
<style>

.mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.mask-content {
  min-width: 80%;    
  min-height: 300px;
  padding: 20px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.mermaid-el .node rect {
    /*height: 80px !important; */
    /* 高度自适应 */

}

.zoom-container {
  overflow: auto; /* 允许滚动 */
  width: 100%;
  min-height: 300px;
  max-height: 80vh;
}

.mermaid-el svg {
  transform-origin: 0 0; /* 缩放原点 */
  transition: transform 0.3s; /* 平滑过渡 */
}
</style>