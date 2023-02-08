<!--
 * @Date: 2022-11-01 14:14:04
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-08 13:30:53
 * @FilePath: /vue-project/src/views/HomeView.vue
-->
<template>
  <el-tabs type="border-card" class="main-tabs">
    <el-tab-pane v-for="item in item_types" :key="item.item_type">
      <template #label>
        <span class="custom-tabs-label">
          <el-icon></el-icon>
          <span>{{ item.item_type_name }}</span>
        </span>
      </template>
      <TaskView :item_type="item.item_type"></TaskView>
    </el-tab-pane>
    <el-tab-pane label="废纸篓" name="rubbish">
      <TaskView item_type="1024"></TaskView>
    </el-tab-pane>
  </el-tabs>
  <el-dialog v-model="dialogNewItemType" v-if="dialogNewItemType" :title="dialogObj.title"
    :fullscreen="dialogObj.fullscreen">
    <el-form v-if="dialogObj.content == 'new_item_type'" :model="ItemTypeForm">
      <el-form-item label="类型名称" :label-width="formLabelWidth">
        <el-input v-model="ItemTypeForm.item_type_name" autocomplete="off" placeholder="ex：生活" />
      </el-form-item>
      <el-form-item label="描述" :label-width="formLabelWidth">
        <el-input v-model="ItemTypeForm.item_type_desc" type="textarea" placeholder="关于该类型的描述，不填也行" />
      </el-form-item>
    </el-form>
    <itemEditor key="new_item" v-else :item="dialogObj.itemObj"></itemEditor>
    <!-- -if="dialogObj.content == 'new_item'" -->
    <!-- <itemEditor key="edit_item" v-else-if="dialogObj.content == 'edit_item'" :item="dialogObj.itemObj"></itemEditor> -->
    <template #footer>
      <span class="dialog-footer">
        <el-button size="small" @click="dialogNewItemType = false">取消</el-button>
        <el-button size="small" type="primary" @click="saveForm">
          保存
        </el-button>
      </span>
    </template>

  </el-dialog>
  <!-- <el-button  type="primary" :icon="Edit" circle /> -->
  <el-dropdown class="new-btn-fixed">
    <span class="el-dropdown-link">
      <el-button type="primary" :icon="Edit" circle />
    </span>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item @click="newItemTypeAction">新建类型</el-dropdown-item>
        <el-dropdown-item @click="newItemAction">
          新建事项
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>
<script lang="ts" setup>
import TaskView from './TaskView.vue'
import { ref, reactive, onMounted } from "vue";
import { emit, listen } from '@tauri-apps/api/event';
import { ElMessage } from 'element-plus'
import { useGetItemTypes, useAddData } from '@/libs/bridge'
import { useTaskStore } from '@/stores/task';
import itemEditor from "@/components/ItemEditor.vue";
import { useGetItems } from '@/libs/bridge'
import {
  Check,
  Delete,
  Edit,
  Message,
  Search,
  Star,
} from '@element-plus/icons-vue'
const taskStore = useTaskStore();
const formLabelWidth = '80px'

const getItem = () => {
  useGetItems().then((res: any) => {
    console.log("获取全局数据")
    console.log(res)
    taskStore.setAllData(res)
  })
}
onMounted(() => {
  getItem();
})

listen('getItems', () => {
  getItem();
  dialogNewItemType.value = false;
})

let dialogObj = ref({
  title: "",
  content: "",
  fullscreen: false,
  itemObj: {}
})
const newItemTypeAction =()=>{
  openDialog('new_item_type');
}
const newItemAction =()=>{
  openDialog('new_item');
}
const openDialog = (msg:any,msgObj:any={})=>{
  switch (msg) {
    case "new_item_type":
      dialogObj.value = {
        title: "新建类型",
        content: "new_item_type",
        fullscreen: false,
        itemObj: {}
      }
      break;
    case "new_item":
      dialogObj.value = {
        title: "新建事项",
        content: "new_item",
        fullscreen: true,
        itemObj: {}
      }
      break;
    case "edit_item":
      dialogObj.value = {
        title: "编辑事项",
        content: "edit_item",
        fullscreen: true,
        itemObj: msgObj.payload.item
      }
      break;
  }
  dialogNewItemType.value = true;
}
listen('rust_event', (msg: any) => {
  console.log("h---------")
  if (!msg.payload.message) {
    msg.payload = JSON.parse(msg.payload);
  }
  console.log(msg.payload.message)
  openDialog(msg.payload.message,msg)
})
const ItemTypeForm = reactive({
  item_type: '',
  item_type_name: '',
  item_type_desc: '',
})
let dialogNewItemType = ref(false);
let item_types = ref<any[]>([]);

const getItemTypes = () => {
  useGetItemTypes().then((res: any) => {
    item_types.value = res;
    taskStore.setItemTypes(res);
  })
}
getItemTypes();
const saveForm = () => {
  console.log("执行我了奥--------------",dialogObj.value.content)
  switch (dialogObj.value.content) {
    case 'new_item_type':
      saveItemType();
      break;
    case 'new_item':
      emit('saveFrom');
      break;
    case 'edit_item':
      emit('saveFrom', 'edit_item');
      break;
  }
}
const saveItemType = () => {
  useAddData(ItemTypeForm, 'add_item_type')
    .then(res => {
      if (res) {
        ElMessage({
          message: '亲，类型新增成功了呦',
          type: 'success',
        })
        dialogNewItemType.value = false
        getItemTypes();
      }
    })
    .catch(err => {
      console.log(`报错了${err}`)
    })
}
</script>
<style lang="scss">
.main-tabs {
  height: 100vh;

  .el-tabs__content {
    padding: 32px;
    height: calc(100vh - 40px);
    color: #6b778c;
    font-weight: 600;

    .el-tab-pane {
      height: 100%;
    }
  }
}

.main-tabs .custom-tabs-label .el-icon {
  vertical-align: middle;
}

.main-tabs .custom-tabs-label span {
  vertical-align: middle;
  margin-left: 4px;
}

.new-btn-fixed {
  position: fixed;
  right: 20px;
  bottom: 20px;
  // width:45px;
  // height: 45px;
  // border-radius: 50%;
  // background-color: var(--el-color-primary);
}
</style>