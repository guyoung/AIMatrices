<template>
    <v-dialog v-model="show" width="auto" min-width="80vw" min-height="80vh" :fullscreen="fullscreen" scrollable>
        <v-card>
            <v-card-title>
                <v-toolbar height="36">
                    <v-toolbar-title>Code Run</v-toolbar-title>
                    <v-spacer></v-spacer>
                    <v-btn v-if="props.code" @click="handleCode()" size="32">
                        <v-icon :size="24">mdi-play-circle-outline</v-icon>
                    </v-btn>
                    <v-btn @click="saveAsMarkdown(props.code)" size="32">
                        <v-icon :size="24" v-html="icons.icon_markdown_line" />
                    </v-btn>
                    <v-btn v-if="result"  @click="saveAsPng(result)" size="32">
                        <v-icon :size="24" v-html="icons.icon_png_line" />
                    </v-btn>
                    <v-btn v-if="result"  @click="saveAsSvg(result)" size="32">
                        <v-icon :size="24" v-html="icons.icon_svg_line" />
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

            <v-card-text class="px-2">
                <v-row>
                    <v-col cols="12" md="6" class="pa-2">
                        <codemirror v-model="code" placeholder="" :autofocus="true" :indent-with-tab="true"
                            :tab-size="2" :extensions="extensions" @ready="handleReady" style="height: 100%;" />
                    </v-col>
                    <v-col cols="12" md="6" class="pa-2">
                        <div id="svg" v-if="result" v-html="result"></div>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
    </v-dialog>
</template>

<script setup lang='ts'>
import { computed, defineProps, ref, onMounted } from 'vue'

import { Codemirror } from 'vue-codemirror'

import { markdown } from '@codemirror/lang-markdown'

import { fetchGraphvizHandler } from '@/api'

import { encode } from 'js-base64';

import * as icons from "../svg-icons"

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

const extensions = [markdown()]

const code = ref("")
const result = ref("")

onMounted(async () => {
    code.value = props.code

    await handleCode()
})

const handleCode = async () => {
    if (code.value) {
        result.value = ""
        try {
            const res = await fetchGraphvizHandler(code.value)
            result.value = res

        } catch (e: any) {
            result.value = e.message
        }
    }
}

// Codemirror EditorView instance ref
const view = ref(null)
const handleReady = (payload: any) => {
    view.value = payload.view
}

const handleClose = () => {
    emit('update:visible', false)
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


function saveAsPng(text: string) {
   




    const img = new Image();
    img.onload = () => {
        const canvas = document.createElement('canvas')
        const ctx = canvas.getContext('2d')
        canvas.width = img.width * 2
        canvas.height = img.height * 2
        
        ctx.drawImage(img, 0, 0, canvas.width, canvas.height)
        const link = document.createElement('a')
        link.download = 'download.png'
        link.href = canvas.toDataURL('image/png')
        link.click()
    };

    img.src = 'data:image/svg+xml;base64,' + encode(text)
}

function saveAsSvg(text: string) {
    const link = document.createElement('a')
    link.download = 'download.svg'
    link.href = 'data:image/svg+xml;charset=utf-8,' + encodeURIComponent(text)
    link.click()
}

</script>
<style>
.v-card-title {
    padding: 0px 0px !important
}
</style>