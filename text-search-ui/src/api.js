// src/api.js
const API_BASE_URL = "http://localhost:8000/api";

export async function uploadTextFiles(fileObjects) {
  const res = await fetch(`${API_BASE_URL}/upload`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(fileObjects),
  });

  if (!res.ok) {
    throw new Error(`HTTP error! status: ${res.status}`);
  }

  return await res.json();
}

export async function fetchDocs() {
  const res = await fetch(`${API_BASE_URL}/docs`);
  if (!res.ok) {
    throw new Error(`HTTP error! status: ${res.status}`);
  }
  return await res.json();
}
