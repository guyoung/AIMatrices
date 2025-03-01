<template>
    <v-dialog v-model="show" width="auto" min-width="80vw" min-height="80vh" :fullscreen="fullscreen" scrollable>
        <v-card>
            <v-card-title>
                <v-toolbar height="36">
                    <v-toolbar-title>Markdown</v-toolbar-title>
                    <v-spacer></v-spacer>
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
                <MdEditor v-model="code" style="height: 100%;" />
            </v-card-text>
        </v-card>
    </v-dialog>
</template>

<script setup lang='ts'>
import { computed, defineProps, ref, onMounted } from 'vue'

import { MdEditor } from 'md-editor-v3';
import 'md-editor-v3/lib/style.css';

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



const handleClose = () => {
    emit('update:visible', false)
}

</script>
<style>
.v-card-title {
    padding: 0px 0px !important
}
</style>