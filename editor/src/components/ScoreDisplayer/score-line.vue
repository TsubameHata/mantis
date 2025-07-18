<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useImgGeometry } from '../../store/documentState';

const {
    vertical = false,
    stroke,
    draggable = true,
    dash = []
} = defineProps<{
    vertical?: boolean,
    draggable?: boolean,
    dash?: number[],
    stroke?: string
}>();

const height = defineModel<number>("height", {required:true});

const {width:imgWidth, height:imgHeight} = storeToRefs(useImgGeometry());

const lineConfig = computed(()=>{
    let dragBoundFunc: (pos:{x:number,y:number})=>{x:number,y:number};
    if(!vertical){
        dragBoundFunc = (pos)=>{
            return {
                x:0,
                y:pos.y
            };
        };
    } else {
        dragBoundFunc = (pos)=>{
            return {
                x: pos.x,
                y:0
            };
        };
    };
    
    let config = {
        stroke: stroke?stroke:(vertical?"rgb(0,255,0)":"rgb(255,0,0)"),
        strokeWidth: Math.max(Math.floor(imgWidth.value/200)+1, 3),
        points: [0,0],
        dash,
        draggable,
        dragBoundFunc,
    };
    if(!vertical){
        config.points = [0, height.value, imgWidth.value, height.value];
    } else {
        config.points = [height.value, 0, height.value, imgHeight.value];
    };
    return config
});

const onDragEnd = computed(()=>{
    return (e:any)=>{
        const newX = e.target.x();
        const newY = e.target.y();
        if(!vertical){
            e.target.y(0);
            height.value += Math.floor(newY);
        } else {
            e.target.x(0);
            height.value += Math.floor(newX);
        }
    };
})

const mouseEnter = ()=>{document.body.style.cursor="grab"};
const mouseleave = ()=>{document.body.style.cursor="default"};
const mouseDown = ()=>{document.body.style.cursor="grabbing"};

const emit = defineEmits<{
    click: [e:MouseEvent]
}>();
</script>

<template>
<v-line
    :config="lineConfig" 
    @mouseenter="mouseEnter"
    @mouseleave="mouseleave"
    @mousedown="mouseDown"
    @dragend="onDragEnd"
    @click="(e:MouseEvent)=>emit('click', e)"></v-line>
</template>

<style scoped>
</style>