/*
 * @Date: 2022-11-01 14:14:04
 * @LastEditors: shijianzhong shijianzhong
 * @LastEditTime: 2022-11-30 09:19:01
 * @FilePath: /vue-project/src/main.ts
 */
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import router from './router'
import mavonEditor from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'
import './assets/main.css'

const app = createApp(App)
app.use(mavonEditor)
app.use(ElementPlus)
app.use(createPinia())
app.use(router)

app.mount('#app')
