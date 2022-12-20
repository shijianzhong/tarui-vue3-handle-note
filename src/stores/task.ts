// @ts-nocheck
/*
 * @Date: 2022-11-01 14:14:04
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-11-30 15:46:29
 */
import { ref, computed, reactive } from 'vue'
import { defineStore } from 'pinia'
// import {useGetItems} from '@/libs/bridge'
export const useTaskStore = defineStore('task', () => {
  let typeItemsStore =reactive(new Map())
  /**
   * 类目
   */
  let itemTypes = ref<any[]>([]);
  
  /**
   * 设置类目
   * @param val 
   */
  function setItemTypes(val){
    itemTypes.value =val;
  }
  //初始化数据
  function setAllData(data){
    this.typeItemsStore = data;
  }
  return { typeItemsStore, setAllData,setItemTypes,itemTypes}
})
