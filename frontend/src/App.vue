<script setup>
import Toggle from './components/Toggle.vue'
import Slider from './components/Slider.vue'
import Select from './components/Select.vue'
</script>

<template>
  <h1 class="text-3xl font-bold bg-cyan-700 rounded-lg">
    Hello, world!
  </h1>
  <Toggle left_text="Off" right_text="On"/>
  <Select name="mode" label="Mode:" options="ABC" selected="A" />
  <Slider name="foo" min="0" max="100" />
  <Slider name="bar" min="0" max="100" />
  <Slider name="baz" min="100" max="250" />

</template>
