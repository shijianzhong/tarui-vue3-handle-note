// @ts-nocheck
/*
 * @Date: 2022-11-17 15:02:39
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-17 14:18:39
 * @FilePath: /vue-project/src/libs/bridge.ts
 */
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api';

export function useGreet() {
  const greetRes = ref(null)
  const greetErr = ref(null)
  let res = invoke('greet', { name: 'world' })
    .then(res => greetRes.value = res)
    .catch(err => greetErr.value = err)
  return { greetRes, greetErr }
}
export function useAi(user_input) {
  return new Promise((resolve, reject) => {
    invoke('ai_qs', { userInput:`${user_input}` })
      .then(res => resolve(res))
      .catch(err => reject(err))
  })

}
export function useDeleteItemByItemId(item_id, real = false) {
  return new Promise((resolve, reject) => {
    invoke(real ? 'delete_item' : 'vitural_delete_item', { itemId: `${item_id}` })
      .then(res => {
        resolve(res)
      })
      .catch(e => reject(e))
  })
}
export function useUpdateItem(item) {
  return new Promise((resolve, reject) => {
    invoke('update_item', { item: JSON.stringify(item) })
      .then(res => {
        resolve(res)
      })
      .catch(e => reject(e))
  })
}
export function useGetItems() {
  return new Promise((resolve, reject) => {
    invoke('get_items')
      .then(res => {
        let val = JSON.parse(res)
        let _data = new Map();
        val.forEach(element => {
          let _key = element.item_type;
          if (_data.has(_key)) {
            _data.get(_key).push(element)
          } else {
            _data.set(_key, [
              element
            ])
          }
        });
        invoke('get_trash_items')
          .then(res => {
            let val = JSON.parse(res)
            val.forEach(element => {
              let _key = "1024";
              if (_data.has(_key)) {
                _data.get(_key).push(element)
              } else {
                _data.set(_key, [
                  element
                ])
              }
            });
            resolve(_data)
          })

      })
      .catch(e => reject(e))
  })
}
export function useGetItemTypes() {
  return new Promise((resolve, reject) => {
    invoke('get_item_types')
      .then(res => {
        let val = JSON.parse(res)
        resolve(val)
      })
      .catch(e => reject(e))
  })
}
// get_item_types
export function useAddData(item, apiStr) {
  return new Promise((resolve, reject) => {
    invoke(apiStr, { form: JSON.stringify(item) })
      .then(res => {
        resolve(res)
      })
      .catch(err => reject(err))
  })
}
// 按照惯例，组合式函数名以“use”开头
export function useBridge() {
  // 被组合式函数封装和管理的状态
  const x = ref(0)
  const y = ref(0)

  // 组合式函数可以随时更改其状态。
  function update(event) {
    x.value = event.pageX
    y.value = event.pageY
  }

  // 一个组合式函数也可以挂靠在所属组件的生命周期上
  // 来启动和卸载副作用
  onMounted(() => window.addEventListener('mousemove', update))
  onUnmounted(() => window.removeEventListener('mousemove', update))

  // 通过返回值暴露所管理的状态
  return { x, y }
}