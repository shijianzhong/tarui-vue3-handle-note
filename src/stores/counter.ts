/*
 * @Date: 2022-11-01 14:14:04
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-11-16 19:27:59
 * @FilePath: /vue-project/src/stores/counter.ts
 */
import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  const doubleCount = computed(() => count.value * 2)
  function increment() {
    count.value++
  }

  return { count, doubleCount, increment }
})
