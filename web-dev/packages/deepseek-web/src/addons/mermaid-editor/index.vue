<template>
    <v-dialog v-model="show" width="auto" min-width="80vw" min-height="80vh" :fullscreen="fullscreen" scrollable>
        <v-card>
            <v-card-title>
                <v-toolbar height="36">
                    <v-toolbar-title>Mermaid</v-toolbar-title>
                    <v-spacer></v-spacer>

                    <v-btn @click="zoom(1.2)" size="32">
                        <v-icon v-html="icons.icon_zoom_in_line" />
                    </v-btn>
                    <v-btn @click="zoom(0.8)" size="32">
                        <v-icon :size="24" v-html="icons.icon_zoom_out_line" />
                    </v-btn>
                    <v-btn @click="saveAsMarkdown(props.code)" size="32">
                        <v-icon :size="24" v-html="icons.icon_markdown_line" />
                    </v-btn>
                    <v-btn @click="saveAsPng()" size="32">
                        <v-icon :size="24" v-html="icons.icon_png_line" />
                    </v-btn>
                    <v-btn @click="saveAsSvg()" size="32">
                        <v-icon :size="24" v-html="icons.icon_svg_line" />
                    </v-btn>
                    <v-btn v-if="preview" @click="preview = false" size="32">
                        <v-icon :size="24">mdi-code-tags</v-icon>
                    </v-btn>
                    <v-btn v-if="!preview" @click="preview = true" size="32">
                        <v-icon :size="24">mdi-chart-box-outline</v-icon>
                    </v-btn>
                    <v-btn v-if="fullscreen" @click="fullscreen = false" size="32">
                        <v-icon :size="24">mdi-window-restore</v-icon>
                    </v-btn>
                    <v-btn v-if="!fullscreen" @click="fullscreen = true" size="32">
                        <v-icon :size="24">mdi-window-maximize</v-icon>
                    </v-btn>
                    <v-btn @click="show = false" size="32">
                        <v-icon :size="24">mdi-close</v-icon>
                    </v-btn>
                </v-toolbar>
            </v-card-title>
            <v-card-text class="px-2 zoom-container">
                <VueMermaidString v-if="preview" class="mermaid-el" :value="code" />
                <codemirror v-if="!preview" v-model="code" placeholder="" :autofocus="true" :indent-with-tab="true"
                            :tab-size="2" :extensions="extensions" @ready="handleReady" style="height: 100%;" />
            </v-card-text>
        </v-card>
    </v-dialog>
</template>

<script setup lang='ts'>

import { computed, defineProps, ref, onMounted  } from 'vue'

import VueMermaidString from "vue-mermaid-string";

import { Codemirror } from 'vue-codemirror'

import { markdown } from '@codemirror/lang-markdown'


import * as icons from "../svg-icons"

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
    code: string
}

interface Emit {
    (e: 'update:visible', visible: boolean): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

const fullscreen = ref(false)

const show = computed({
    get: () => props.visible,
    set: (visible: boolean) => emit('update:visible', visible),
})

const code = ref("")

onMounted(async () => {
    code.value = props.code
})

const extensions = [markdown()]

const preview = ref(true)

// Codemirror EditorView instance ref
const view = ref(null)
const handleReady = (payload: any) => {
    view.value = payload.view
}

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
.v-card-title {
    padding: 0px 0px !important
}

.zoom-container {
    overflow: auto;
    /* 允许滚动 */
    width: 100%;
    height: 100%;
}

.mermaid-el svg {
    transform-origin: 0 0;
    /* 缩放原点 */
    transition: transform 0.3s;
    /* 平滑过渡 */
}


.mermaid-el .node rect {
    /*height: 80px !important; */
    /* 高度自适应 */

}
</style>