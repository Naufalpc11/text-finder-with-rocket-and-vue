<script setup>
import { onMounted, ref } from "vue";
import { pingHello } from "./api";

const backendMessage = ref("Loading...");

onMounted(async () => {
  try {
    backendMessage.value = await pingHello();
  } catch (err) {
    backendMessage.value = "Error: " + err.message;
  }
});
</script>

<template>
  <main class="p-4">
    <h1>Frontend Vue</h1>
    <p>Respon dari Rocket: {{ backendMessage }}</p>
  </main>
</template>
