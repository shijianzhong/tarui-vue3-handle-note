<!--
 * @Date: 2022-11-15 20:18:54
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-01 16:44:39
 * @FilePath: /vue-project/src/components/TaskItems/TaskTypeItems.vue
-->
<template>
    <div class="task-type-item-container">
        <el-scrollbar>
            <draggable v-if="list?.length != 0" class="list-group" group="list-group" v-model="list"
                tag="transition-group" :component-data="{ tag: 'div' }" @start="drag = true" @end="drag = false"
                animation=1000>
                <template #item="{ element }">
                    <div>
                        <TaskItemVue :item="element"></TaskItemVue>
                    </div>
                </template>

            </draggable>
            <div class="button" >
                <el-button type="primary" size="small" @click="dialogFormVisible = !dialogFormVisible">新增</el-button>
            </div>
        </el-scrollbar>
        <el-dialog v-model="dialogFormVisible" title="事项">
            <TaskItemForm @addItemSuccess="dialogFormVisible = !dialogFormVisible"></TaskItemForm>
        </el-dialog>
    </div>

</template>
<script lang="ts" setup>
import draggable from 'vuedraggable'
import TaskItemVue from '@/components/TaskItems/TaskItem.vue';
import TaskItemForm from '@/components/TaskItems/TaskItemForm.vue';
import { computed, reactive, ref, watch } from 'vue'
import { useTaskStore } from '@/stores/task';
const taskStore = useTaskStore();
const dialogFormVisible = ref(false)
const props = defineProps({
    list: Array,
    code: Number
})
let _list = computed(() => props.list)
const drag = ref(false);
// watch(_list, (newValue) => {
//     taskStore.setItems(_list, props.code)
// })

</script>
<style lang="scss">
.task-type-item-container {
    padding: 8px 12px;
    width: 20%;
    min-width: 260px;
    margin-right: 16px;
    border: 1px dashed var(--el-color-info);
    .el-scrollbar__view{
        display: flex;
        flex-direction: column;
    }
    .button {
        width: 100%;
        text-align: center;

        .el-button {
            width: 80%;
        }
    }

    .list-group{
        width: 100%;
    }
}

.list-group-item {
    cursor: move;
}

.list-group-item i {
    cursor: pointer;
}
</style>