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
                        <v-textarea variant="outlined" :value="result" readonly style="height: 100%;"></v-textarea>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
    </v-dialog>
</template>

<script setup lang='ts'>
import { computed, defineProps, ref, onMounted } from 'vue'

import { Codemirror } from 'vue-codemirror'

import { python } from '@codemirror/lang-python'

import { fetchCodeHandler } from '@/api'

interface Props {
    visible: boolean,
    code: string,
    language: string,
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

const extensions = ref()

const code = ref("")
const result = ref("")

onMounted(async () => {
    code.value = props.code

    if (props.language == "python") {
        extensions.value = [python()]
    }

    await handleCode()
})

const handleCode = async () => {
    if (code.value) {
        result.value = ""
        try {
            const res = await fetchCodeHandler(code.value, props.language)
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

</script>
<style>
.v-card-title {
    padding: 0px 0px !important
}
</style>