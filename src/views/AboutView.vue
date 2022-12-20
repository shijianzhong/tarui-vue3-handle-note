<!--
 * @Date: 2022-11-01 14:14:04
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-12-03 19:40:54
 * @FilePath: /vue-project/src/views/AboutView.vue
-->
<template>
  <div class="about">
    <h1>This is an about page111122</h1>
  </div>
</template>
<script setup lang="ts">
import { onMounted } from 'vue';
import {invoke} from '@tauri-apps/api';
  onMounted(()=>{
    callGreet();
    getItems();
  })
  const callGreet = ()=>{
    invoke('greet',{name: 'world'}).then(res=>{
      console.log(res)
    })
  }
  const getItems = ()=>{
    invoke('get_items').then(res=>{
    })
  }
</script>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
