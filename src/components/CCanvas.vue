<template>
    <div class="ccanvas-container">
        <div class="paint-toolbar" v-if="viewType=='edit'">
            <el-button size="small" v-if="drawPencilBtnState=='primary'" type="primary" @click="startDraw">画笔</el-button>
            <el-button size="small" v-else @click="startDraw">画笔</el-button>
            <el-button size="small" @click="addCicle">圆</el-button>
            <el-button size="small" @click="addTriangle">三角形</el-button>
            <el-button size="small" @click="addRect">方形</el-button>
            <el-color-picker size="small" style="margin-left: 10px;" v-model="color" show-alpha
                :predefine="predefineColors" @change="colorChange" />
            <el-input-number style="margin-left: 10px;" size="small" @change="strokeWidthChange" v-model="strokeWidth"
                :min="1" :max="100" />

            <el-input style="max-width: 150px;" v-model="text" @change="textChange" size="small"
                placeholder="输入尝试一下..." />
            <el-button style="margin-left: 10px;" size="small" v-if="isSelectedObj" @click="deleteObj">选中删除</el-button>
            <el-button size="small" @click="clearCanvas">全擦除</el-button>

        </div>
        <canvas :id="itemId" :width="canvasWidth" :height="canvasHeight"></canvas>
    </div>
</template>
<script lang="ts" setup>
import { fabric } from 'fabric'
import { ref, onMounted, watchEffect, nextTick, onUnmounted } from 'vue'
const emit = defineEmits<{
    (e: "update:modelValue", val: String): void;
}>();
const props = defineProps({
    modelValue: String,
    viewType:String,
    itemId:String
})

const strokeWidth = ref(1)
const color = ref('rgba(255, 69, 0, 0.68)')
const predefineColors = ref([
    '#ff4500',
    '#ff8c00',
    '#ffd700',
    '#90ee90',
    '#00ced1',
    '#1e90ff',
    '#c71585',
    'rgba(255, 69, 0, 0.68)',
    'rgb(255, 120, 0)',
    'hsv(51, 100, 98)',
    'hsva(120, 40, 94, 0.5)',
    'hsl(181, 100%, 37%)',
    'hsla(209, 100%, 56%, 0.73)',
    '#c7158577',
])
let canvasWidth = ref(0);
let canvasHeight = ref(0);
canvasWidth.value = document.body.clientWidth - 40;
canvasHeight.value = document.body.clientHeight - 300;
let canvas = ref<any>({});

onMounted(() => {
    initFabric();

    nextTick(() => {
        if (props.modelValue != "") {
            canvas.value.loadFromJSON(props.modelValue)
        }
    })
})
onUnmounted(()=>{
    canvas.value = null;
})
const dealContent = () => {
    let _val = JSON.stringify(canvas.value.toJSON());
    // let _val =canvas.value.toDataURL()
    emit("update:modelValue", _val);

}
const initFabric = () => {
    canvas.value = new fabric.Canvas(props.itemId);
    canvas.value.on('mouse:down', (options: any) => {
        if (options.target) {
            isSelectedObj.value = true
            // console.log('有对象被点击咯! ', options.target.type);
        } else {
            isSelectedObj.value = false
        }
        // console.log(options.e.clientX, options.e.clientY);
    })
    canvas.value.on('mouse:up', (options: any) => {
        // console.log('松开了')
        dealContent()
        // canvas.value.isDrawingMode = false;
    })

}
let drawPencilBtnState = ref("");
/**
 * 自由绘画
 */
const startDraw = () => {
    if (drawPencilBtnState.value == "") {
        drawPencilBtnState.value = "primary";
        canvas.value.freeDrawingBrush.width = strokeWidth.value;
        canvas.value.freeDrawingBrush.color = color.value;
        canvas.value.isDrawingMode = true;
    } else {
        drawPencilBtnState.value = "";
        canvas.value.isDrawingMode = false;
    }
    dealContent();

}
let _circle = ref<any>({})
const addCicle = () => {
    _circle.value = new fabric.Circle({
        radius: 40, fill: color.value, left: 100, top: 100
    });
    canvas.value.add(_circle.value);
    dealContent()
    // _circle.value.set({
    //     transparentCorners: false,
    //     cornerColor: 'blue',
    //     cornerStrokeColor: 'red',
    //     borderColor: 'red',
    //     padding: 10,
    //     cornerStyle: 'circle',
    //     borderDashArray: [3, 3]
    // });

}
let text = ref('');
let _text = ref(null)
const textChange = (val: any) => {
    if (val != '') {
        _text.value = new fabric.Text(val, { left: 100, top: 100 });
        canvas.value.add(_text.value);
    } else {
    }
    dealContent()
}

let _triangle = ref(null)
const addTriangle = () => {
    _triangle.value = new fabric.Triangle({
        width: 100, height: 50, fill: color.value, left: 50, top: 50
    });
    canvas.value.add(_triangle.value);
    dealContent()
}
let _rect = ref(null)
const addRect = () => {
    _rect.value = new fabric.Rect({
        width: 100, height: 150, fill: color.value, left: 50, top: 50
    });
    canvas.value.add(_rect.value);
    dealContent();
}
const colorChange = () => {
    setObejctStyle({
        fill: color.value
    });

}
const strokeWidthChange = () => {
    setObejctStyle({
        strokeWidth: strokeWidth.value
    });

}
const isSelectedObj = ref(false);
const deleteObj = () => {
    let activeObj = canvas.value.getActiveObject();
    if (activeObj.type == 'activeSelection') {
        activeObj.getObjects().forEach((o: any) => {
            canvas.value.remove(o)
        })
    } else {
        canvas.value.remove(activeObj);
    }

    canvas.value.requestRenderAll();
    dealContent()
}
const setObejctStyle = (style: any) => {
    let activeObj = canvas.value.getActiveObject();
    if (!activeObj) return;
    if (activeObj.type == 'activeSelection') {
        activeObj.getObjects().forEach((o: any) => {
            o.set(
                style
            );
        })
    } else {
        activeObj.set(
            style
        );

    }
    dealContent();
    canvas.value.requestRenderAll();

}
/**
 * 清空画布
 */
const clearCanvas = () => {
    canvas.value.clear();
    dealContent()
}

</script>
<style lang="scss">
canvas {
    border: 1px dashed red;
}

.paint-toolbar {
    display: flex;
    align-items: center;
    height: 40px;
}
</style>