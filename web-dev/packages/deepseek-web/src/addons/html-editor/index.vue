<template>
    <v-dialog v-model="show" width="auto" min-width="80vw" min-height="80vh" :fullscreen="fullscreen" scrollable>
        <v-card>
            <v-card-title>
                <v-toolbar height="36">
                    <v-toolbar-title>Html</v-toolbar-title>
                    <v-spacer></v-spacer>
                    <v-btn v-if="preview" @click="preview = false" size="32">
                        <v-icon :size="24">mdi-code-tags</v-icon>
                    </v-btn>
                    <v-btn v-if="!preview" @click="preview = true" size="32">
                        <v-icon :size="24">mdi-web</v-icon>
                    </v-btn>
                    <v-btn v-if="fullscreen" @click="fullscreen = false;" size="32">
                        <v-icon :size="24">mdi-window-restore</v-icon>
                    </v-btn>
                    <v-btn v-if="!fullscreen" @click="fullscreen = true;" size="32">
                        <v-icon :size="24">mdi-window-maximize</v-icon>
                    </v-btn>
                    <v-btn @click="show = false" size="32">
                        <v-icon :size="24">mdi-close</v-icon>
                    </v-btn>
                </v-toolbar>
            </v-card-title>
            <v-card-text style="display: flex; flex-direction: column; ">
                <iframe v-show="preview" ref="previewFrame" class="preview-iframe"
                    style="width: 100%; border: none;"></iframe>
                <codemirror v-show="!preview" v-model="codeRef" placeholder="" :autofocus="true" :indent-with-tab="true"
                    :tab-size="2" :extensions="extensions" @ready="handleReady" style="height: 100%;" />

            </v-card-text>
        </v-card>
    </v-dialog>
</template>

<script setup lang='ts'>

import { computed, defineProps, ref, onMounted, watch, nextTick } from 'vue'

import { Codemirror } from 'vue-codemirror'
import { markdown } from '@codemirror/lang-markdown'
import IframeResizer from '@iframe-resizer/vue/sfc'



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


const previewFrame = ref(null)

const codeRef = ref('')


watch(codeRef, (newValue, oldValue) => {
    preview_html(newValue)
})

onMounted(async () => {
    codeRef.value = props.code

    nextTick(() => {
        const previewContainer = document.querySelector('.v-card-text');

        console.log(previewContainer)

        resizeObserver.observe(previewContainer);

        /***
        const iframe = previewFrame.value;
        const observer = new MutationObserver(adjustHeight);

        iframe.addEventListener('load', () => {
            const doc = iframe.contentDocument;
            observer.observe(doc.body, { subtree: true, childList: true });
        });

        adjustHeight()
        ***/

    })

    /***
    window.addEventListener('resize', adjustHeight);
    ***/


})

const extensions = [markdown()]

const preview = ref(true)

const preview_html = (html) => {
    const iframe = previewFrame.value;
    const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;

    iframeDoc.open();
    iframeDoc.write(html);
    iframeDoc.close();

    //adjustHeight()
}

const resizeObserver = new ResizeObserver((entries) => {
    for (let entry of entries) {
        let height = entry.contentRect.height

        if (previewFrame.value && previewFrame.value.style) {
            previewFrame.value.style.height = `${height}px`;
        }

    }
});


/***
const adjustHeight = () => {

    console.log(previewContainer.value.parentNode.offsetHeight)

    if (previewContainer.value && previewContainer.value.parentNode && previewContainer.value.parentNode.offsetHeight) {
        let height = previewContainer.value.parentNode.offsetHeight - 32

        previewFrame.value.style.height = `${height}px`;
    }
}

const triggerResize = () => {
      window.dispatchEvent(new Event('resize'));
    };
***/

// Codemirror EditorView instance ref
const view = ref(null)
const handleReady = (payload: any) => {
    view.value = payload.view
}

const handleClose = () => {
    emit('update:visible', false)
}


</script>
<style>
.v-card-title {
    padding: 0px 0px !important
}
</style>