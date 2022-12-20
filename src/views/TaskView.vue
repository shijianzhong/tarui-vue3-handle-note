<!--
 * @Date: 2022-11-15 20:18:54
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-03 19:41:38
 * @FilePath: /vue-project/src/views/TaskView.vue
-->
<template>
  <el-scrollbar class="task-view-container">
    <el-row :gutter="10">
      <el-col :xs="24" :sm="24" :md="12" :lg="6" :xl="6" v-for="i in items" :key="i.item_id">
        <TaskItem :item="i"></TaskItem>
      </el-col>
    </el-row>
  </el-scrollbar>


</template>
<script lang="ts" setup>
import TaskItem from "@/components/TaskItem.vue"
import { useTaskStore } from "@/stores/task";
import { computed, watchEffect } from 'vue'
import { useGetItems } from "@/libs/bridge";
const taskStore = useTaskStore()
let items = computed(() => taskStore.typeItemsStore.get(props.item_type))
const props = defineProps({
  item_type: String
})
</script>
<style lang="scss">
.task-view-container {
  height: 100%;

  .el-scrollbar__view {
    height: 100%;
    display: flex;
    justify-content: center;

    .el-row {
      width: 100%;
    }
  }

}
</style>