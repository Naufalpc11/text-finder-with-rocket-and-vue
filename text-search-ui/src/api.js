const API_BASE_URL = "http://localhost:8000/api";

export async function pingHello() {
  const res = await fetch(`${API_BASE_URL}/hello`);
  if (!res.ok) {
    throw new Error("Failed to call backend");
  }
  const text = await res.text();
  return text;
}
