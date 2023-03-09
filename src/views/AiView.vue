<!--
 * @Date: 2023-02-16 15:43:46
 * @LastEditors: shijianzhong 994129509@qq.com
 * @LastEditTime: 2023-02-21 11:03:20
 * @FilePath: /vue-project/src/views/AiView.vue
-->
<template>
    <div class="gpt-container">
        <div class="box-1" id="list-box" v-loading="loading">
            <div class="talk-list">
                <div v-for="(item, index) in talkList" :key="index" :id="`msg-${index}`">
                    <div class="talk-item" :class="item.type == 1 ? 'push' : 'pull'">
                        <div class="content">{{ item.content }}</div>
                    </div>
                </div>
            </div>
        </div>
        <div class="box-2">
            <el-input @keydown.enter="send" v-model="tempQs" placeholder="请输入内容">

            </el-input>
            <el-button type="primary" class="send" @click="send">发送</el-button>
            <el-button type="success" class="save" @click="saveNote">一键生产笔记</el-button>
        </div>
    </div>
</template>
<script setup lang="ts">
import { useAi, useAddData } from "@/libs/bridge"
import {useRouter} from "vue-router"
import {
    ref
} from 'vue'
const router = useRouter();
// const gpt = uniCloud.importObject("chatgpt")
// const cconfig = uniCloud.importObject("cconfig")
const talkList = ref([])
const tempQs = ref()
const loading = ref(false)
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
const send = async () => {
    talkList.value.push({
        type: 1,
        content: tempQs.value
    })
    useAi(tempQs.value).then((o: any) => {
        let res = JSON.parse(o)
        console.log(res)
        console.log(typeof res)
        loading.value=true
        if (res.choices.length > 0) {
            tempQs.value = ""
            for (let i = 0; i < res.choices.length; i++) {
                talkList.value.push({
                    type: 0,
                    content: res.choices[i].text
                })
                loading.value = false
            }
        }
    })
}
const saveNote = () => {

    if (talkList.value.length > 0) {

        form.value.item_name = talkList.value[0].content;
        talkList.value.forEach(o => {
            if (o.type == 1) {
                form.value.item_desc += `# ${o.content} \n`
            } else {
                form.value.item_desc += `### ${o.content} \n`
            }
        })

        useAddData(form.value, "add_item").then(()=>{
            router.push('/')
        })
    } else {

    }

}
</script>
<style lang="scss">
.gpt-container {
    height: 100%;
    padding: 16px;

    .box-1 {
        width: 100%;
        height: auto;
        padding-bottom: 200px;
        box-sizing: content-box;

    }

    .box-2 {
        position: fixed;
        display: flex;
        left: 8px;
        right: 8px;
        bottom: 8px;
        height: auto;
        border-top: #e5e5e5 solid 1px;
        box-sizing: content-box;
        background-color: #F3F3F3;


    }

    .talk-list {
        padding-bottom: 20px;

        .pic {
            width: 92px;
            height: 92px;
            border-radius: 50%;
            border: #fff solid 1px;
        }

        .content {
            padding: 16px;
            border-radius: 4px;
            max-width: 500px;
            word-break: break-all;
            line-height: 25px;
            position: relative;
        }

        /* 收到的消息 */
        .pull {
            display: flex;
            justify-content: flex-start;

            .content {
                margin-left: 32px;
                background-color: #dff9fb;

                &::after {
                    content: '';
                    display: block;
                    width: 0;
                    height: 0;
                    border-top: 16px solid transparent;
                    border-bottom: 16px solid transparent;
                    border-right: 20px solid #dff9fb;
                    position: absolute;
                    top: 13px;
                    left: -18px;
                }
            }
        }

        /* 发出的消息 */
        .push {
            display: flex;
            justify-content: flex-end;

            .content {
                margin-right: 32px;
                background-color: #a0e959;

                &::after {
                    content: '';
                    display: block;
                    width: 0;
                    height: 0;
                    border-top: 16px solid transparent;
                    border-bottom: 16px solid transparent;
                    border-left: 20px solid #a0e959;
                    position: absolute;
                    top: 13px;
                    right: -18px;
                }
            }
        }
    }
}
</style>