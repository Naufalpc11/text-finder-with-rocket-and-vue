<script setup>
import { onMounted, ref } from "vue";
import { fetchDocs, searchWords, uploadTextFiles } from "./api";

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
  <main style="padding: 2rem; font-family: system-ui, sans-serif;">
    <h1>Text Search with Rocket + Vue</h1>

    <!-- Upload -->
    <section style="margin-bottom: 2rem;">
      <h2>1. Upload File .txt (2–6 file)</h2>
      <input type="file" multiple accept=".txt" @change="handleFileChange" />
      <button style="margin-left: 0.5rem;" @click="uploadFiles">
        Upload ke Server
      </button>

      <div v-if="selectedFiles.length" style="margin-top: 0.5rem;">
        <strong>File dipilih:</strong>
        <ul>
          <li v-for="file in selectedFiles" :key="file.name">
            {{ file.name }}
          </li>
        </ul>
      </div>

      <div v-if="docsOnServer.length" style="margin-top: 0.5rem;">
        <strong>Dokumen di server:</strong>
        <ul>
          <li v-for="doc in docsOnServer" :key="doc.id">
            [{{ doc.id }}] {{ doc.name }}
          </li>
        </ul>
      </div>
    </section>

    <!-- Search -->
    <section style="margin-bottom: 2rem;">
      <h2>2. Pencarian Kata</h2>
      <p>
        Isi 1 kata (single thread) atau 2 kata (otomatis parallel dengan Rayon /
        multi-thread).
      </p>
      <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
        <input v-model="word1" placeholder="Kata 1 (misal: kami)" />
        <input v-model="word2" placeholder="Kata 2 (opsional, misal: mahasiswa)" />
        <button @click="doSearch">Cari</button>
      </div>
    </section>

    <p>{{ message }}</p>

    <!-- Hasil search -->
    <section v-if="searchResult" style="margin-top: 1rem;">
      <h2>Hasil Pencarian</h2>
      <div v-for="wordRes in searchResult.results" :key="wordRes.word">
        <h3>
          Kata: "{{ wordRes.word }}" — total kemunculan:
          {{ wordRes.total_count }}
        </h3>
        <ul>
          <li v-for="doc in wordRes.per_doc" :key="doc.doc_id">
            [{{ doc.doc_id }}] {{ doc.doc_name }} → {{ doc.count }} kali
          </li>
        </ul>
      </div>
    </section>
  </main>
</template>
