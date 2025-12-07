const API_BASE_URL = "http://localhost:8000";

export async function listDocs() {
  const res = await fetch(`${API_BASE_URL}/docs`);
  if (!res.ok) {
    throw new Error(`Gagal memuat daftar dokumen (status ${res.status})`);
  }
  return await res.json();
}

export async function searchWords(query) {
  const res = await fetch(`${API_BASE_URL}/search`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ query }),
  });

  if (!res.ok) {
    throw new Error(`Search gagal (status ${res.status})`);
  }

  return await res.json();
}
