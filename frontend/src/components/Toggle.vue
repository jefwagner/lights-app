<script setup>
    defineProps({
        left_text: String,
        right_text: String,
    })

    var isOn = false;

    function onOff() {
        console.log(!isOn)
    }
</script>

<template>
    <label class="relative flex justify-center items-center p-2 text-xl py-4 font-bold">
        <h2 class="text-right px-4">{{ left_text }}</h2>
        <input type="checkbox" v-model="isOn" class="absolute left-1/2 -translate-x-1/2 w-full h-full peer appearance-none rounded-md" @click="onOff"/>
        <span class="w-32 h-10 flex grow-0 items-center ml-4 p-1 bg-slate-800 rounded-lg duration-300 ease-in-out peer-checked:bg-cyan-700 after:w-20 after:h-8 after:bg-slate-400 after:rounded-lg after:shadow-md after:duration-300 peer-checked:after:bg-slate-100 peer-checked:after:translate-x-10"></span>
        <h2 class="text-left px-4">{{ right_text }}</h2>
    </label>
</template>
