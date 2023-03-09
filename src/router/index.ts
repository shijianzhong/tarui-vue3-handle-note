/*
 * @Date: 2022-11-01 14:14:04
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-16 15:45:18
 * @FilePath: /vue-project/src/router/index.ts
 */
import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('../views/HomeView.vue')
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/ai',
      name: 'ai',
      component: () => import('../views/AiView.vue')
    },
  ]
})

export default router
