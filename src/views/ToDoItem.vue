<!--
 * @Date: 2022-11-14 21:02:40
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-11-15 11:34:53
 * @FilePath: /vue-project/src/views/ToDoItem.vue
-->
<template>
    <div class="daylist">
        <h2>每日清单 <span class="current_time"></span></h2>
        <p>双击文字内容编辑修改</p>
        <p>双击条目的空白部分删除该条目</p>
        <input type="text" v-model="item" @keyup.enter="addItem"  id="input" placeholder="Type here..."  />
        <ul class="list">
            <li class="list_item" v-for="(item,index) in items" @dblclick="">
                <span class="list_item_text">{{item}}</span>
            </li>
        </ul>
    </div>
</template>
<script setup lang="ts">
    import {reactive, ref} from 'vue'
    const item = ref('');
    const items = reactive<string[]>([]);
    const addItem = ()=>{
        items.push(item.value);
        item.value="";
    }
    const removeItem = ()=>{

    }
</script>
<style>


.daylist {
    width: 100%;
    height: 100%;
    padding: 10px 10px;
    border-radius: 4px;
    position: absolute;
    left: 50%;
    top: 50%;
    -webkit-transform: translate(-50%, -50%);
    transform: translate(-50%, -50%);
    box-shadow: 0 10px 20px rgba(51, 51, 51, 0.2), 0 6px 6px rgba(0, 0, 0, 0.23);
}
h2 {
    text-align: center;
    font-size: 30px;
    margin-bottom: 5px;
}
.current_time {
    font-size: 10px;
    color: #999999;
}
p {
    font-size: 16px;
    text-align: center;
    color: #999999;
}
#input {
    width: 300px;
    height: 35px;
    border-radius: 4px;
    border: 2px solid #d9d9d9;
    outline: none;
    display: block;
    margin: 20px auto;
    color: #404040;
}
#input:focus {
    border: 2px solid #999999;
}
.daylist .list {
    display: block;
    margin: 0 auto;
    font-size: 16px;
    width: 300px;
    list-style-position: inside;
}
.daylist .list .list_item {
    color: #525252;
    padding: 10px 0px;
    border-bottom: 1px solid #d9d9d9;
    position: relative;
}
.list_item_text {
    position: relative;
    outline: none;
    padding: 2px 4px;
    border: 2px solid transparent;
    margin-bottom: 3px;
    line-height: 1.5;
    margin-bottom: 2px;
}
.list_item_text::-moz-selection {
    background: #93aac5;
}
.list_item_text::selection {
    background: #93aac5;
}
.list_item_text:focus {
    border: 2px solid #d9d9d9;
    border-radius: 4px;
}
.list_item:hover {
    background: whitesmoke;
}
.list_item time {
    font-size: 9px;
    color: #999999;
}
</style>