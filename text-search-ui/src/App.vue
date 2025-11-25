<script setup>
import { onMounted, ref } from 'vue'
import IconTextFinder from './assets/Icontextfinder.png'
import HomePage from './views/HomePage.vue'

const isLoading = ref(true)

onMounted(() => {
  setTimeout(() => {
    isLoading.value = false
  }, 1000)
})
</script>

<template>
  
  <Transition name="fade">
    <div v-if="isLoading" class="fixed inset-0 bg-sky-400 z-50 flex items-center justify-center">
      <div class="text-center">
        
        <div class="relative mb-8">
          
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="w-32 h-32 border-4 border-white/30 rounded-full animate-spin-slow"></div>
          </div>
          
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="w-24 h-24 border-4 border-white/50 rounded-full animate-spin" style="animation-direction: reverse;"></div>
          </div>
          
          <div class="relative z-10 flex items-center justify-center">
            <div class="w-20 h-20 bg-white rounded-full flex items-center justify-center shadow-2xl animate-pulse-slow p-3">
              <img :src="IconTextFinder" alt="Text Finder Icon" class="w-full h-full object-contain" />
            </div>
          </div>
        </div>
        
        
        <h1 class="text-4xl font-bold text-white mb-4 animate-bounce-slow">Text Search Tool</h1>
        <div class="flex gap-2 justify-center">
          <div class="w-3 h-3 bg-white rounded-full animate-bounce" style="animation-delay: 0s;"></div>
          <div class="w-3 h-3 bg-white rounded-full animate-bounce" style="animation-delay: 0.2s;"></div>
          <div class="w-3 h-3 bg-white rounded-full animate-bounce" style="animation-delay: 0.4s;"></div>
        </div>
      </div>
    </div>
  </Transition>

  
  <div class="min-h-screen bg-sky-400 p-4 md:p-8">
    <HomePage />
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
