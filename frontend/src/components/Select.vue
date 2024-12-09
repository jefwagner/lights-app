<script setup>
    const props = defineProps({
        name: String,
        label: String,
        options: Array,
        selected: String,
    })

    import { onMounted } from 'vue'

    onMounted(() => {
        for (var i=0; i<props.options.length; i++) {
            console.log(props.options[i], props.options[i]  == props.selected)
        }
    })

    function selectMode() {
        for (var i=0; i<props.options.length; i++) {
            if (props.options[i] == props.selected) {
                console.log(i)
            }
        }
    }

</script>

<template>
    <div class="flex">
        <label :for="`select-${props.name}`" class="block mb-2 text-lg font-bold px-2 self-center">{{ props.label }}</label>
        <select :id="`select-${props.name}`" v-model="props.selected" @change="selectMode"
            class="bg-slate-700 border border-slate-600 text-lg rounded-lg flex-grow focus:ring-cyan-700 focus:border-cyan-700 block p-2.5">
            <option v-for="opt in props.options" :value="opt">{{ opt }}</option>
        </select>
    </div>
</template>