<script setup>
import { listDocs, searchWords } from '@/api.js';
import { computed, onMounted, ref } from "vue";
import IconTextFinder from '../assets/Icontextfinder.png';

const availableDocs = ref([]);
const searchQuery = ref("");
const isSearching = ref(false);
const searchResults = ref(null);
const errorMessage = ref("");

const canSearch = computed(() => {
  return (
    availableDocs.value.length >= 1 &&
    searchQuery.value.trim().length > 0
  );
});

onMounted(async () => {
  try {
    availableDocs.value = await listDocs();
  } catch (error) {
    errorMessage.value = "Gagal memuat daftar dokumen dari server";
    console.error(error);
  }
});

async function performSearch() {
  if (!canSearch.value) return;

  isSearching.value = true;
  errorMessage.value = "";
  searchResults.value = null;

  try {
    const result = await searchWords(searchQuery.value.trim());
    searchResults.value = {
      query: searchQuery.value.trim(),
      words: result.results,
      benchmark: result.benchmark,
    };
  } catch (error) {
    errorMessage.value = "Terjadi kesalahan saat pencarian: " + error.message;
    console.error(error);
  } finally {
    isSearching.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-100 via-purple-50 to-pink-100 py-8 px-4">
    <!-- Header -->
    <div class="max-w-5xl mx-auto bg-gradient-to-r from-sky-600 to-indigo-600 rounded-3xl shadow-2xl p-6 mb-6 text-center">
      <div class="flex items-center justify-center gap-4 mb-3">
        <img :src="IconTextFinder" alt="Logo" class="h-16 w-16" />
        <h1 class="text-4xl font-extrabold text-white">Text Finder</h1>
      </div>
      <p class="text-white/90 text-lg">Cari kata-kata dalam file PDF dataset</p>
    </div>

    <!-- Dataset Info Section -->
    <div class="max-w-5xl mx-auto bg-white rounded-2xl shadow-2xl p-6 md:p-8 mb-6 transform transition-all">
      <h2 class="text-2xl font-bold text-sky-700 mb-4 flex items-center gap-2">
        <span>📁</span> Dataset Files
      </h2>
      
      <div v-if="availableDocs.length === 0" class="text-center py-8">
        <p class="text-gray-500 text-lg">⏳ Loading dataset files...</p>
      </div>
      
      <div v-else class="space-y-3">
        <div class="flex justify-between items-center mb-4">
          <p class="text-gray-700 font-semibold">
            <span class="text-2xl font-bold text-sky-600">{{ availableDocs.length }}</span> PDF files ready
          </p>
        </div>
        
        <div class="grid md:grid-cols-2 gap-3">
          <div 
            v-for="doc in availableDocs" 
            :key="doc.id"
            class="flex items-center gap-3 p-4 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors"
          >
            <span class="text-3xl">📕</span>
            <div class="flex-1 min-w-0">
              <p class="font-semibold text-gray-800 truncate">{{ doc.name }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Search Section -->
    <div class="max-w-5xl mx-auto bg-white rounded-2xl shadow-2xl p-6 md:p-8 mb-6 transform transition-all">
      <h2 class="text-2xl font-bold text-sky-700 mb-6 flex items-center gap-2">
        <span>🔍</span> Pencarian
      </h2>
      
      <div class="mb-6">
        <label class="block mb-2 font-semibold text-gray-700">Masukkan kata-kata (dipisahkan spasi):</label>
        <input 
          v-model="searchQuery" 
          type="text" 
          placeholder="Contoh: informatika teknologi komunikasi"
          @keyup.enter="performSearch"
          class="w-full px-4 py-3 border-2 border-gray-300 rounded-xl focus:border-sky-500 focus:ring-2 focus:ring-sky-200 outline-none transition-all"
        />
        <p class="mt-2 text-sm text-gray-600">Program akan mencari setiap kata di {{ availableDocs.length }} file PDF</p>
      </div>
      
      <button 
        @click="performSearch" 
        :disabled="!canSearch || isSearching"
        :class="[
          'w-full py-4 rounded-xl font-bold text-lg transition-all transform',
          canSearch && !isSearching
            ? 'bg-sky-600 text-white hover:scale-[1.02] hover:shadow-xl'
            : 'bg-gray-300 text-gray-500 cursor-not-allowed'
        ]"
      >
        <span v-if="isSearching" class="flex items-center justify-center gap-2">
          <span class="animate-spin">⏳</span> Mencari...
        </span>
        <span v-else>🔍 Cari</span>
      </button>
    </div>

    <!-- Error Message -->
    <div v-if="errorMessage" class="max-w-5xl mx-auto bg-red-50 border-2 border-red-300 text-red-700 px-6 py-4 rounded-xl mb-6 text-center font-semibold animate-shake">
      ⚠️ {{ errorMessage }}
    </div>

    <!-- Search Results -->
    <div v-if="searchResults" class="max-w-5xl mx-auto bg-white rounded-2xl shadow-2xl p-6 md:p-8 animate-fade-in">
      <h2 class="text-2xl font-bold text-sky-700 mb-6 flex items-center gap-2">
        <span>📊</span> Hasil Pencarian
      </h2>
      
      <!-- Benchmark Results -->
      <div class="mb-8 p-6 bg-gradient-to-r from-purple-500 to-pink-500 rounded-2xl text-white">
        <h3 class="text-xl font-bold mb-4 flex items-center gap-2">
          <span>⚡</span> Performa Benchmark
        </h3>
        <div class="grid md:grid-cols-3 gap-4">
          <div class="bg-white/20 backdrop-blur p-4 rounded-xl">
            <p class="text-sm opacity-90 mb-1">Sequential (Tanpa Parallel)</p>
            <p class="text-2xl font-bold">{{ searchResults.benchmark.sequential_ms.toFixed(3) }} ms</p>
          </div>
          <div class="bg-white/20 backdrop-blur p-4 rounded-xl">
            <p class="text-sm opacity-90 mb-1">Parallel (Multiprocessing)</p>
            <p class="text-2xl font-bold">{{ searchResults.benchmark.parallel_ms.toFixed(3) }} ms</p>
          </div>
          <div class="bg-white/20 backdrop-blur p-4 rounded-xl">
            <p class="text-sm opacity-90 mb-1">Speedup</p>
            <p class="text-2xl font-bold">{{ searchResults.benchmark.speedup.toFixed(2) }}x</p>
          </div>
        </div>
        <p class="mt-4 text-sm opacity-90">
          {{ searchResults.benchmark.speedup > 1 
            ? `🚀 Parallel processing ${searchResults.benchmark.speedup.toFixed(2)}x lebih cepat!` 
            : '⏱️ Sequential sama cepat atau lebih cepat (data terlalu kecil untuk parallelization overhead)' 
          }}
        </p>
      </div>

      <!-- Word Results -->
      <div class="mb-6">
        <h3 class="text-xl font-bold text-gray-800 mb-4">Kata yang Dicari: "{{ searchResults.query }}"</h3>
        
        <div class="space-y-4">
          <div 
            v-for="wordResult in searchResults.words" 
            :key="wordResult.word"
            class="border-2 border-sky-300 rounded-xl p-6 bg-sky-50"
          >
            <div class="flex justify-between items-center mb-4">
              <h4 class="text-xl font-bold text-sky-700">"{{ wordResult.word }}"</h4>
              <span class="bg-sky-600 text-white px-6 py-2 rounded-full font-bold text-lg">
                Total: {{ wordResult.total_count }}x
              </span>
            </div>
            
            <div v-if="wordResult.per_doc && wordResult.per_doc.length > 0" class="space-y-2">
              <p class="font-semibold text-gray-700 mb-2">Ditemukan di:</p>
              <div 
                v-for="doc in wordResult.per_doc" 
                :key="doc.doc_id"
                class="flex justify-between items-center p-3 bg-white rounded-lg"
              >
                <span class="font-medium text-gray-800">
                  📕 {{ doc.doc_name }}
                </span>
                <span class="bg-sky-500 text-white px-4 py-1 rounded-full font-bold">
                  {{ doc.count }}x
                </span>
              </div>
            </div>
            <div v-else class="text-gray-500 italic">
              Kata tidak ditemukan di dokumen manapun
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-10px); }
  75% { transform: translateX(10px); }
}

.animate-fade-in {
  animation: fade-in 0.6s ease-out;
}

.animate-shake {
  animation: shake 0.5s ease-in-out;
}
</style>
