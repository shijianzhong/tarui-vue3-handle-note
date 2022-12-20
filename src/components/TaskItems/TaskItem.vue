<!--
 * @Date: 2022-11-15 21:18:20
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-03 19:40:21
 * @FilePath: /vue-project/src/components/TaskItems/TaskItem.vue
-->
<template>
    <div class="task-item-container">
        <el-row :gutter="20">
            <el-col :span="4">
                <el-checkbox>
                </el-checkbox>
            </el-col>
            <el-col :span="20">
                {{item?.item_name}}
            </el-col>
        </el-row>
        <el-row :gutter="20">
            <el-col :span="4">
                
            </el-col>
            <el-col :span="20">
                未完成
            </el-col>
        </el-row>
    </div>
</template>
<script lang="ts" setup>
const props = defineProps({
  item:Object
})

</script>
<style lang="scss">
.task-item-container{
    -ms-user-select:none;
    -khtml-user-select:none;
    -webkit-user-select: none;
    user-select: none;
    padding: 4px 16px;
    border-bottom-left-radius: 8px;
    border-top-left-radius: 8px;
    border: 1px solid var(--el-color-warning);
    transition: 0.5s;
    margin-bottom: 8px;
    &:hover{
        border-left: 10px solid var(--el-color-primary);
    }
}
</style>