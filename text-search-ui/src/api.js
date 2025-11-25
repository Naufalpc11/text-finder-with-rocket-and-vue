const API_BASE_URL = "http://localhost:8000";


export async function uploadTextFiles(filesPayload) {
  const res = await fetch(`${API_BASE_URL}/upload`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(filesPayload),
  });

  if (!res.ok) {
    throw new Error(`Upload gagal (status ${res.status})`);
  }

  return await res.json();
}

export async function deleteAllDocs() {
  const res = await fetch(`${API_BASE_URL}/docs`, {
    method: "DELETE",
  });

  if (!res.ok) {
    const msg = await res.text();
    throw new Error(msg || `Gagal delete semua (status ${res.status})`);
  }

  return await res.json();
}

export async function deleteDocById(id) {
  const res = await fetch(`${API_BASE_URL}/docs/${id}`, {
    method: "DELETE",
  });

  if (!res.ok) {
    const msg = await res.text();
    throw new Error(
      msg || `Gagal delete dokumen dengan id ${id} (status ${res.status})`
    );
  }

  return await res.json();
}

export async function searchWords(words) {
  const res = await fetch(`${API_BASE_URL}/search`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ words }),
  });

  if (!res.ok) {
    throw new Error(`Search gagal (status ${res.status})`);
  }

  return await res.json();
}
