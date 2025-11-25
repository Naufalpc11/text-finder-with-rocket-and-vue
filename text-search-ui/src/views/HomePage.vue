<script setup>
import { onMounted, ref } from "vue";
import { fetchDocs, searchWords, uploadTextFiles } from "../api";

const selectedFiles = ref([]);
const message = ref("");
const docsOnServer = ref([]);

const word1 = ref("");
const word2 = ref("");
const searchResult = ref(null);

const handleFileChange = (event) => {
  selectedFiles.value = Array.from(event.target.files);
};

async function uploadFiles() {
  if (selectedFiles.value.length === 0) {
    message.value = "Pilih minimal 1 file .txt dulu.";
    return;
  }

  if (selectedFiles.value.length < 2 || selectedFiles.value.length > 6) {
    message.value = "Untuk project ini, pilih 2 sampai 6 file .txt.";
    return;
  }

  try {
    message.value = "Membaca file dan upload...";
    const payload = await Promise.all(
      selectedFiles.value.map(
        (file) =>
          new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => {
              resolve({
                name: file.name,
                content: reader.result,
              });
            };
            reader.onerror = () => reject(reader.error);
            reader.readAsText(file, "UTF-8");
          })
      )
    );

    const result = await uploadTextFiles(payload);
    message.value = `Upload sukses. Total dokumen di server: ${result.total_files}.`;

    docsOnServer.value = await fetchDocs();
  } catch (err) {
    console.error(err);
    message.value = "Upload gagal: " + err.message;
  }
}

async function doSearch() {
  const words = [];
  if (word1.value.trim()) words.push(word1.value);
  if (word2.value.trim()) words.push(word2.value);

  if (words.length === 0) {
    message.value = "Isi minimal 1 kata pencarian.";
    return;
  }

  try {
    message.value = "Melakukan pencarian...";
    const result = await searchWords(words);
    searchResult.value = result;
    message.value =
      words.length >= 2
        ? "Pencarian selesai (mode parallel / multi-thread)."
        : "Pencarian selesai (single thread).";
  } catch (err) {
    console.error(err);
    message.value = "Search gagal: " + err.message;
  }
}

onMounted(async () => {
  try {
    docsOnServer.value = await fetchDocs();
  } catch (err) {
    console.warn("Belum ada dokumen di server.");
  }
});
</script>

<template>
  <div class="max-w-7xl mx-auto">
    <!-- Header -->
    <div class="text-center mb-8 animate-fade-in">
      <h1 class="text-4xl md:text-5xl font-bold text-white mb-2 text-shadow-lg">Text Search Tool</h1>
      <p class="text-white/90 text-lg">Upload 2-6 file .txt dan cari 2 kata secara bersamaan</p>
    </div>

    <!-- Upload Section -->
    <div class="bg-white rounded-2xl shadow-2xl p-6 md:p-8 mb-6 transform transition-all">
      <h2 class="text-2xl font-bold text-sky-700 mb-6 flex items-center gap-2">
        <span>📁</span> Upload File
      </h2>
      
      <div class="text-center mb-6">
        <input 
          type="file" 
          id="fileInput" 
          multiple 
          accept=".txt"
          @change="handleFileUpload"
          :disabled="!canUploadMore"
          class="hidden"
        />
        <label 
          for="fileInput" 
          :class="[
            'inline-block px-8 py-4 rounded-xl font-semibold text-lg cursor-pointer transition-all transform',
            canUploadMore 
              ? 'bg-sky-600 text-white hover:scale-105 hover:shadow-xl' 
              : 'bg-gray-300 text-gray-500 cursor-not-allowed'
          ]"
        >
          <span v-if="canUploadMore">Pilih File (.txt)</span>
          <span v-else>Maksimal 6 file tercapai</span>
        </label>
        <p class="mt-3 text-gray-600 font-medium">{{ uploadedFiles.length }} / 6 file diupload</p>
      </div>

      <!-- Uploaded Files List -->
      <div v-if="uploadedFiles.length > 0" class="mt-6">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-lg font-semibold text-gray-800">File yang Diupload:</h3>
          <button 
            @click="clearAllFiles" 
            class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors font-medium"
          >
            Hapus Semua
          </button>
        </div>
        
        <div class="space-y-3">
          <div 
            v-for="file in uploadedFiles" 
            :key="file.id"
            class="flex justify-between items-center p-4 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors"
          >
            <div class="flex items-center gap-4">
              <span class="text-3xl">📄</span>
              <div>
                <p class="font-semibold text-gray-800">{{ file.name }}</p>
                <p class="text-sm text-gray-500">{{ file.size }}</p>
              </div>
            </div>
            <button 
              @click="removeFile(file.id)" 
              class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors font-bold"
            >
              ✕
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-2xl shadow-2xl p-6 md:p-8 mb-6 transform transition-all">
      <h2 class="text-2xl font-bold text-sky-700 mb-6 flex items-center gap-2">
        <span></span> Pencarian
      </h2>
      
      <div class="flex flex-col md:flex-row items-center gap-4 mb-6">
        <div class="flex-1 w-full">
          <label class="block mb-2 font-semibold text-gray-700">Kata Pertama:</label>
          <input 
            v-model="searchWord1" 
            type="text" 
            placeholder="Masukkan kata pertama..."
            @keyup.enter="performSearch"
            class="w-full px-4 py-3 border-2 border-gray-300 rounded-xl focus:border-sky-500 focus:ring-2 focus:ring-sky-200 outline-none transition-all"
          />
        </div>
        
        <div class="text-2xl font-bold text-sky-600 mt-0 md:mt-8">+</div>
        
        <div class="flex-1 w-full">
          <label class="block mb-2 font-semibold text-gray-700">Kata Kedua:</label>
          <input 
            v-model="searchWord2" 
            type="text" 
            placeholder="Masukkan kata kedua..."
            @keyup.enter="performSearch"
            class="w-full px-4 py-3 border-2 border-gray-300 rounded-xl focus:border-sky-500 focus:ring-2 focus:ring-sky-200 outline-none transition-all"
          />
        </div>
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
        <span v-else>Cari</span>
      </button>
    </div>

    <div v-if="errorMessage" class="bg-red-50 border-2 border-red-300 text-red-700 px-6 py-4 rounded-xl mb-6 text-center font-semibold animate-shake">
      ⚠️ {{ errorMessage }}
    </div>

    <div v-if="searchResults" class="bg-white rounded-2xl shadow-2xl p-6 md:p-8 animate-fade-in ">
      <h2 class="text-2xl font-bold text-sky-700 mb-6 flex items-center gap-2">
        <span></span> Hasil Pencarian
      </h2>
      
      <div class="mb-8">
        <div class="bg-sky-600 text-white p-6 rounded-2xl shadow-xl">
          <h3 class="text-xl font-bold mb-4">Total Kemunculan</h3>
          
          <div class="space-y-3">
            <div class="flex justify-between items-center p-4 bg-white/10 rounded-xl backdrop-blur">
              <span class="font-semibold text-lg">"{{ searchResults.word1 }}"</span>
              <span class="bg-white/30 px-4 py-2 rounded-full font-bold text-xl">{{ searchResults.totalCount1 }}x</span>
            </div>
            
            <div class="flex justify-between items-center p-4 bg-white/10 rounded-xl backdrop-blur">
              <span class="font-semibold text-lg">"{{ searchResults.word2 }}"</span>
              <span class="bg-white/30 px-4 py-2 rounded-full font-bold text-xl">{{ searchResults.totalCount2 }}x</span>
            </div>
            
            <div class="flex justify-between items-center p-4 bg-white/20 rounded-xl backdrop-blur border-2 border-white/30">
              <span class="font-semibold text-lg">File dengan KEDUA kata</span>
              <span class="bg-green-500 px-4 py-2 rounded-full font-bold text-xl">{{ searchResults.filesWithBothWords }} file</span>
            </div>
          </div>
        </div>
      </div>

      <div>
        <h3 class="text-xl font-bold text-gray-800 mb-4">Detail:</h3>
        
        <div class="space-y-4">
          <div 
            v-for="result in searchResults.fileResults" 
            :key="result.fileName"
            :class="[
              'border-2 rounded-2xl p-6 hover:scale-[1.02] hover:shadow-xl transition-all',
              result.hasBothWords 
                ? 'border-green-500 bg-green-50' 
                : result.hasNoWords
                  ? 'border-red-500 bg-red-50'
                  : 'border-yellow-500 bg-yellow-50'
            ]"
          >
            <div class="flex flex-wrap justify-between items-center gap-3 mb-4">
              <h4 class="text-lg font-bold text-gray-800 flex items-center gap-2">
                <span>📄</span> {{ result.fileName }}
              </h4>
              <span 
                :class="[
                  'px-4 py-2 rounded-full font-semibold text-sm',
                  result.hasBothWords 
                    ? 'bg-green-500 text-white' 
                    : result.hasNoWords
                      ? 'bg-red-500 text-white'
                      : 'bg-yellow-500 text-white'
                ]"
              >
                <span v-if="result.hasBothWords">✓ Kedua kata ditemukan</span>
                <span v-else-if="result.hasNoWords">Tidak ada kata</span>
                <span v-else>Tidak lengkap</span>
              </span>
            </div>
            
            <div class="grid md:grid-cols-2 gap-4 mb-4">
              <div class="bg-white p-4 rounded-xl flex justify-between items-center shadow">
                <span class="font-semibold text-gray-700">"{{ searchResults.word1 }}":</span>
                <span class="bg-sky-600 text-white px-4 py-2 rounded-full font-bold">{{ result.count1 }}x</span>
              </div>
              <div class="bg-white p-4 rounded-xl flex justify-between items-center shadow">
                <span class="font-semibold text-gray-700">"{{ searchResults.word2 }}":</span>
                <span class="bg-sky-600 text-white px-4 py-2 rounded-full font-bold">{{ result.count2 }}x</span>
              </div>
            </div>

            <!-- Context Lines for Word 1 -->
            <div v-if="result.linesWithWord1.length > 0" class="mt-4 p-4 bg-white rounded-xl border-l-4 border-sky-600">
              <h5 class="font-bold text-sky-600 mb-3">Konteks "{{ searchResults.word1 }}":</h5>
              <div class="space-y-2">
                <div 
                  v-for="line in result.linesWithWord1" 
                  :key="line.lineNumber"
                  class="flex gap-3 p-3 bg-gray-50 rounded-lg font-mono text-sm"
                >
                  <span class="text-sky-600 font-bold shrink-0">Baris {{ line.lineNumber }}:</span>
                  <span class="text-gray-700 break-all" v-html="highlightText(line.content, searchResults.word1, searchResults.word2)"></span>
                </div>
                <p v-if="result.count1 > 3" class="text-gray-600 italic text-sm mt-2">
                  ... dan {{ result.count1 - 3 }} kemunculan lainnya
                </p>
              </div>
            </div>

            <!-- Context Lines for Word 2 -->
            <div v-if="result.linesWithWord2.length > 0" class="mt-4 p-4 bg-white rounded-xl border-l-4 border-sky-500">
              <h5 class="font-bold text-sky-500 mb-3">Konteks "{{ searchResults.word2 }}":</h5>
              <div class="space-y-2">
                <div 
                  v-for="line in result.linesWithWord2" 
                  :key="line.lineNumber"
                  class="flex gap-3 p-3 bg-gray-50 rounded-lg font-mono text-sm"
                >
                  <span class="text-sky-500 font-bold shrink-0">Baris {{ line.lineNumber }}:</span>
                  <span class="text-gray-700 break-all" v-html="highlightText(line.content, searchResults.word1, searchResults.word2)"></span>
                </div>
                <p v-if="result.count2 > 3" class="text-gray-600 italic text-sm mt-2">
                  ... dan {{ result.count2 - 3 }} kemunculan lainnya
                </p>
              </div>
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
