<!--
 * @Date: 2022-11-17 09:25:32
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-01 16:44:30
 * @FilePath: /vue-project/src/components/TaskItems/TaskItemForm.vue
-->
<template>
    <el-form :model="form" label-width="80px">
        <el-form-item label="标题">
            <el-input v-model="form.item_name" autocomplete="off" />
        </el-form-item>
        <el-form-item label="类型">
            <el-select v-model="form.item_type">
                <el-option label="工作" value="1" />
                <el-option label="生活" value="2" />
            </el-select>
        </el-form-item>
        <el-form-item label="备注">
            <el-input v-model="form.item_desc" type="textarea" />
        </el-form-item>
        <el-form-item label="优先级">
            <el-radio-group v-model="form.item_priority">
                <el-radio label="1">一般</el-radio>
                <el-radio label="2">重要</el-radio>
                <el-radio label="3">紧急</el-radio>
            </el-radio-group>
        </el-form-item>
        <el-form-item label="时间">
            <el-date-picker v-model="form.item_start" type="datetime" 
                :default-time="defaultTime" />
            <el-date-picker v-model="form.item_end" type="datetime" 
                :default-time="defaultTime" />
        </el-form-item>
        <el-form-item>
            <el-button type="primary" size="small" @click="onSubmit">Create</el-button>
            <el-button>Cancel</el-button>
        </el-form-item>
    </el-form>
</template>
<script setup lang="ts">
import { ElMessage } from 'element-plus'
import { reactive, ref, defineEmits, unref, isRef } from 'vue'
import { useGreet,  useAddData } from '@/libs/bridge'
import { emit } from '@tauri-apps/api/event';
const props = defineProps({
    value: Object,
})
const emits = defineEmits(['addItemSuccess'])
const { greetRes } = useGreet();
const defaultTime = 
    new Date(new Date().getFullYear(), new Date().getMonth()+1,new Date().getDate() , new Date().getHours(), new Date().getMinutes(), new Date().getSeconds());
let form = reactive({
    item_id:0,
    item_name: '',
    item_type: '1',
    item_desc: '',
    item_priority: '1',
    item_start: '',
    item_end: '',
    item_state:0
})

const onSubmit = () => {
    useAddData(form,"add_item")
        .then(res => {
            if (res) {
                emits('addItemSuccess')
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
</script>