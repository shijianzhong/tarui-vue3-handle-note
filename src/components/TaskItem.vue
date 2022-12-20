<!--
 * @Date: 2022-11-29 15:20:47
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-09 10:56:15
 * @FilePath: /vue-project/src/components/TaskItem.vue
-->
<template>
  <el-card class="task-item-container">
    <div class="top">
      <div class="top-left">
        <h3 style="font-weight: bold;" :class="{'first-p':item.item_priority==1 ,
      'second-p':item.item_priority==2,'third-p':item.item_priority==3
      }">{{ item?.item_name }}</h3>
        <div>{{ item?.item_end }}</div>
      </div>
      <div class="top-right">
        <IconEdit style="height:50%" @click="itemEditor"></IconEdit>
        <IconDelete style="height:50%" @click="deleteItem"></IconDelete>
      </div>
    </div>
    <div class="content">
      <el-scrollbar>
        <mavon-editor v-if="item?.item_desc_type == '1'" key="preview" v-model="item.item_desc" :subfield="false"
          placeholder="112" defaultOpen="preview" :toolbarsFlag="false" :boxShadow="false"
          previewBackground="#fff"></mavon-editor>
        <CCanvas v-else v-model="item.item_desc" view-type="view" :itemId="item.item_id"></CCanvas>
      </el-scrollbar>
    </div>
  </el-card>
</template>
<script setup lang="ts">
import IconEdit from './icons/IconEdit.vue';
import IconDelete from './icons/IconDelete.vue';
import { ref, watchEffect, onMounted, nextTick } from 'vue'
import { ElMessage } from 'element-plus'
import { emit } from '@tauri-apps/api/event';
import { useDeleteItemByItemId } from "@/libs/bridge"
import { fabric } from 'fabric'
import CCanvas from './CCanvas.vue';

const props = defineProps({
  item: Object
})

const itemEditor = () => {
  emit('rust_event', JSON.stringify({ message: "edit_item", item: props.item }))
}
const deleteItem = () => {
  let real = props.item?.item_type == '1024';
  useDeleteItemByItemId(props.item?.item_id, real).then(res => {
    if (res) {
      emit('getItems')
      ElMessage({
        message: real ? '彻底删除了，给你的电脑硬盘减负了地球上的一只蚂蚁那么微不足道' : '删除了，数据不会真的丢失奥，都在你本地数据库呢',
        type: 'success',
      })
    }
  })
}
</script>
<style lang="scss">
.task-item-container {
  height: 350px;
  margin-bottom: 16px;


  .top {
    height: 50px;
    display: flex;
    justify-content: space-between;
    .first-p{
      color:black
    }
    .second-p{
      color:green
    }
    .third-p{
      color:red
    }
    .top-right {
      display: flex;
      align-items: center;
      width: 80px;
      padding: 2px;
      .item-priority{
        width:10px;
        height: 10px;
        background-color: red;
      
      }
    }
  }

  .content {
    height: 300px;

    .v-note-wrapper {
      width: 100%;
      min-width: 100% !important;
      height: 100%;
      min-height: 100% !important;
    }
  }
}
</style>