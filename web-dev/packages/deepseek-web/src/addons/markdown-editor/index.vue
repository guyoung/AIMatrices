<template>
    <div v-if="props.visible" class="mask">
        <div class="mask-content">
            <NCard title="Markdown" hoverable closable @close="handleClose">
                <MdEditor v-model="props.content" />
            </NCard>
        </div>
    </div>
</template>

<script setup lang='ts'>
import type { CSSProperties } from 'vue'
import { onMounted, computed, defineProps, ref } from 'vue'

import { NCard, } from 'naive-ui'

// NCard, NDataTable, NDivider, NInput, NList, NListItem,
// NPopconfirm, NSpace, NTabPane, NTabs, NThing, useMessage

import { MdEditor } from 'md-editor-v3';
import 'md-editor-v3/lib/style.css';

interface Props {
    visible: boolean,
    content: string
}

interface Emit {
    (e: 'update:visible', visible: boolean): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

const show = computed({
    get: () => props.visible,
    set: (visible: boolean) => emit('update:visible', visible),
})



const handleClose = () => {
    emit('update:visible', false)
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
</style>