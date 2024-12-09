<script setup>
    const props = defineProps({
        name: String,
        min: Number,
        max: Number,
    })

    import { ref, onMounted } from 'vue'

    const val = ref(0);

    onMounted(() => {
        val.value = (Number(props.max) + Number(props.min))/2;
    })

    function showVal() {
        console.log(val.value);
    }

</script>

<template>
    <div class="p-2 text-xl py-1 font-bold">
        <label :for="`range-${props.name}`" class="block mb-2 text-left text-gray-900 dark:text-white">{{ props.name }}: {{ val }}</label>
        <input 
            :id="`range-${props.name}`" 
            type="range" 
            :min="props.min" 
            :max="props.max" 
            v-model="val"
            @change="showVal"
            class="range accent-cyan-700 w-full h-2 bg-slate-200 rounded-lg appearance-none cursor-pointer dark:bg-slate-700"
        />
    </div>
</template>