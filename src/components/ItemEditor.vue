<!--
 * @Date: 2022-11-29 15:20:46
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-08 13:34:42
 * @FilePath: /vue-project/src/components/ItemEditor.vue
-->
<template>
    <div class="item-editor-container">
        <div class="editor-top">
            <el-input v-model="form.item_name" autocomplete="off" placeholder="标题" />
            <el-select v-model="form.item_type">
                <el-option v-for="item in taskStore.itemTypes" :label="item.item_type_name" :value="item.item_type" />
            </el-select>
        </div>
        <div class="editor-top">
            <el-date-picker v-model="form.item_end" type="datetime" placeholder="截止时间" format="YYYY/MM/DD hh:mm:ss"
                value-format="YYYY-MM-DD hh:mm:ss" />
            <el-select v-model="form.item_priority">
                <el-option label="一般" value="1" />
                <el-option label="重要" value="2" />
                <el-option label="紧急" value="3" />
            </el-select>
            <el-select v-model="form.item_desc_type" @change="itemDescTypeChange">
                <el-option label="文本" value="1" />
                <el-option label="画版" value="2" />

            </el-select>
        </div>
        <div class="editor-content">
            <mavon-editor v-if="(form.item_desc_type == '1')" v-model="form.item_desc"></mavon-editor>
            <c-canvas v-else v-model="form.item_desc" view-type="edit" item-id="cavans"></c-canvas>
        </div>
    </div>
</template>
<script setup lang="ts">
import { useTaskStore } from '@/stores/task'
import { ElMessage } from 'element-plus'
import CCanvas from './CCanvas.vue'
import { reactive, ref, onUnmounted, unref, isRef, onMounted, watch, watchEffect, onBeforeMount, onBeforeUnmount } from 'vue'
import { useGreet, useAddData, useUpdateItem } from '@/libs/bridge'
import { emit, once } from '@tauri-apps/api/event';

let unlisten_saveFrom = reactive(null)
const taskStore = useTaskStore();
const props = defineProps({
    item: <any>Object
})


let form = ref({
    item_id: '0',
    item_name: '',
    item_type: "1",
    item_type_name: '',
    item_desc: '',
    item_priority: "1",
    item_start: '',
    item_end: '',
    item_state: 0,
    item_desc_type: "1"
})

console.log(form.value)

watchEffect((newValue) => {
    if (Object.keys(props.item).length > 0) {
        form.value = props.item;
        console.log(form.value)
    }
})

const itemDescTypeChange = () => {
    // console.log('初始化的时候就到这里了', form.value.item_desc_type)

}

const dateFormat = (dat: Date) => {
    //获取年月日，时间
    var year = dat.getFullYear();
    var mon = (dat.getMonth() + 1) < 10 ? "0" + (dat.getMonth() + 1) : dat.getMonth() + 1;
    var data = dat.getDate() < 10 ? "0" + (dat.getDate()) : dat.getDate();
    var hour = dat.getHours() < 10 ? "0" + (dat.getHours()) : dat.getHours();
    var min = dat.getMinutes() < 10 ? "0" + (dat.getMinutes()) : dat.getMinutes();
    var seon = dat.getSeconds() < 10 ? "0" + (dat.getSeconds()) : dat.getSeconds();
    var newDate = year + "-" + mon + "-" + data + " " + hour + ":" + min + ":" + seon;
    return newDate;
}
onBeforeMount(async() => {
    unlisten_saveFrom = await once('saveFrom', (e) => {
        if (form.value.item_desc_type == '2') {
            // console.log(canvas.value.toObject())
            // console.log(JSON.stringify(canvas.value.toJSON()))
            // form.value.item_desc = JSON.stringify(canvas.value.toJSON())
            // console.log('到涂鸦这里了啊 啊 啊！！！！！');
        }
        form.value.item_start = dateFormat(new Date())
        console.log(form.value)
        console.log(e.payload)
        if (e.payload == "edit_item") {
            useUpdateItem(form.value)
                .then(res => {
                    if (res) {
                        emit('getItems')
                        ElMessage({
                            message: '更新成功',
                            type: 'success',
                        })
                    }
                })
        } else {
            useAddData(form.value, "add_item")
                .then(res => {
                    if (res) {
                        emit('getItems')
                        ElMessage({
                            message: '新增成功',
                            type: 'success',
                        })
                    }
                })
                .catch(err => {
                    console.log(`报错了${err}`)
                })
        }

    })
})

onBeforeUnmount(()=>{
    unlisten_saveFrom()
})



</script>
<style lang="scss">
.item-editor-container {
    .editor-top {
        display: flex;
        margin-bottom: 8px;
    }

    // .editor-content {
    //     .canvas-container {

    //     }
    // }
}
</style>