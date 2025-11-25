<script setup>
import { ref } from "vue";
import { fetchDocs, uploadTextFiles } from "./api";

const selectedFiles = ref([]);
const message = ref("");
const docsOnServer = ref([]);

const handleFileChange = (event) => {
  selectedFiles.value = Array.from(event.target.files);
};

async function uploadFiles() {
  if (selectedFiles.value.length === 0) {
    message.value = "Pilih minimal 1 file .txt dulu.";
    return;
  }

  if (selectedFiles.value.length < 2 || selectedFiles.value.length > 6) {
    message.value = "Untuk projectmu: pilih 2 sampai 6 file .txt.";
    // masih boleh lanjut kalau mau, tapi di sini aku stop saja:
    return;
  }

  try {
    message.value = "Membaca file dan upload...";
    // baca isi file jadi teks
    const payload = await Promise.all(
      selectedFiles.value.map(
        (file) =>
          new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => {
              resolve({
                name: file.name,
                content: reader.result, // isi file sebagai string
              });
            };
            reader.onerror = () => reject(reader.error);
            reader.readAsText(file, "UTF-8");
          })
      )
    );

    const result = await uploadTextFiles(payload);
    message.value = `Upload sukses. Total dokumen di server sekarang: ${result.total_files}. (ID baru: [${result.doc_ids.join(
      ", "
    )}])`;

    // refresh daftar dokumen di server
    docsOnServer.value = await fetchDocs();
  } catch (err) {
    console.error(err);
    message.value = "Upload gagal: " + err.message;
  }
}
</script>

<template>
  <main style="padding: 2rem; font-family: system-ui, sans-serif;">
    <h1>Text Search – Upload .txt</h1>

    <p>Pilih 2–6 file .txt untuk di-upload ke backend Rocket.</p>

    <input
      type="file"
      multiple
      accept=".txt"
      @change="handleFileChange"
    />

    <div style="margin-top: 1rem;">
      <button @click="uploadFiles">Upload ke Server</button>
    </div>

    <p style="margin-top: 1rem;">{{ message }}</p>

    <div v-if="selectedFiles.length" style="margin-top: 1rem;">
      <h2>File yang dipilih:</h2>
      <ul>
        <li v-for="file in selectedFiles" :key="file.name">
          {{ file.name }}
        </li>
      </ul>
    </div>

    <div v-if="docsOnServer.length" style="margin-top: 1rem;">
      <h2>Dokumen di server:</h2>
      <ul>
        <li v-for="doc in docsOnServer" :key="doc.id">
          [{{ doc.id }}] {{ doc.name }}
        </li>
      </ul>
    </div>
  </main>
</template>
