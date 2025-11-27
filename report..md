# TextSearch: Web Service Pencarian Teks Multi-Berkas Berbasis Rust dan Rocket
_A Functional Programming Approach with Rust_  
**Authors:** Abdullah Adiwarman Wildan, Daniel Belawa Koten, Dimas Ramadhani, Naufal Tiarana Putra

---

## Abstract
Provide a concise summary of the project, its goals, the technologies used (Rust + Axum/Rocket/GTK4+/Tauri), and how functional programming principles were applied.

---

## Introduction  

Perkembangan teknologi informasi membuat volume data tekstual yang dihasilkan pengguna meningkat sangat pesat, mulai dari catatan kuliah, log aplikasi, hingga dokumentasi proyek. Namun, proses pencarian teks secara manual pada banyak berkas (`.txt`) masih sering dilakukan dengan cara tradisional: membuka satu per satu file dan menggunakan fitur *find* bawaan editor. Pendekatan ini memakan waktu, rawan kesalahan, dan tidak efisien ketika jumlah file sudah mencapai beberapa buah dengan ukuran yang cukup besar.  

Berdasarkan permasalahan tersebut, proyek ini mengusulkan sebuah **aplikasi *Text Search Tool*** yang memungkinkan pengguna mengunggah 2 hingga 6 berkas teks kemudian melakukan pencarian hingga dua kata kunci secara bersamaan. Aplikasi tidak hanya menghitung jumlah kemunculan kata di setiap berkas, tetapi juga menampilkan potongan kalimat yang relevan serta menyorot (*highlight*) kata yang dicari. Dengan demikian, pengguna dapat memperoleh konteks kemunculan kata secara cepat tanpa harus membaca seluruh isi dokumen.  

Bahasa pemrograman **Rust** dipilih karena menawarkan kombinasi kinerja tinggi, keamanan memori, serta dukungan yang baik terhadap pemrograman *concurrent* dan *parallel*. Hal ini penting karena proses pencarian teks pada beberapa berkas dirancang untuk dijalankan secara **paralel di level CPU** dengan memanfaatkan **multi-threading** dan *parallel iterator* dari crate **Rayon**: saat pengguna mengunggah sedikitnya dua berkas, setiap berkas dapat diproses pada *thread* yang berbeda, dan ketika pengguna mencari lebih dari satu kata kunci, pencarian untuk tiap kata dijalankan secara paralel di seluruh dokumen.  

Integrasi konsep **pemrograman fungsional** dalam proyek ini diwujudkan melalui penggunaan *iterator*, fungsi-fungsi tingkat tinggi (*higher-order functions*), pemrosesan data yang bersifat *immutable* sebisa mungkin. Pendekatan ini membuat alur transformasi teks—mulai dari pembacaan berkas, pemecahan baris, normalisasi kata, hingga perhitungan frekuensi—menjadi lebih deklaratif, ringkas, dan mudah diuji. Dengan demikian, prinsip-prinsip pemrograman fungsional tidak hanya menjadi konsep teoretis, tetapi benar-benar diaplikasikan dalam desain logika aplikasi yang berjalan di atas eksekusi multi-threaded.  

Keunikan solusi yang dikembangkan ada pada kombinasi **Rust sebagai backend dengan kerangka kerja web Rocket** serta **antarmuka frontend berbasis Vue**. Backend bertanggung jawab terhadap pemrosesan teks yang intensif secara komputasi sekaligus mengelola eksekusi paralel di beberapa *thread*, sementara frontend memberikan pengalaman interaktif berupa unggah berkas, form pencarian, serta tampilan hasil dengan *highlight* kata kunci. Pemisahan yang jelas antara lapisan logika dan presentasi, ditambah penerapan prinsip pemrograman fungsional dan pemanfaatan multi-threading di level CPU, menjadikan proyek ini tidak hanya relevan sebagai tugas akhir mata kuliah Pemrograman Fungsional, tetapi juga berpotensi dikembangkan lebih lanjut sebagai alat bantu praktis dalam analisis teks.  

---

## Background and Concepts
Dalam era digital saat ini, pengelolaan dan pencarian informasi dari berbagai sumber data menjadi semakin penting. Dengan meningkatnya volume data yang dihasilkan, metode pencarian yang efisien dan efektif menjadi krusial untuk meningkatkan produktivitas. Pencarian teks dalam berkas-berkas besar sering kali menjadi tantangan, terutama ketika pengguna harus berurusan dengan banyak file secara bersamaan.

Konsep pemrograman fungsional menawarkan pendekatan yang berbeda dalam menangani masalah ini. Dengan memanfaatkan fungsi sebagai unit dasar pemrograman, kita dapat menciptakan solusi yang lebih modular, konsisten, dan mudah dipahami. Paradigma ini juga mendorong penggunaan data yang immutable, sehingga dapat mengurangi kesalahan akibat perubahan status yang tidak terduga. Selain itu, penggunaan higher-order functions, iterator chains, dan penanganan error berbasis Option serta Result membuat proses transformasi dan analisis teks menjadi lebih terstruktur dan aman.

Berikut ini adalah konsep-konsep teoretis yang menjadi dasar dalam pengembangan aplikasi TextSearch, sekaligus mendasari pemilihan teknologi dan arsitektur sistem yang digunakan pada proyek ini.

### Technology Stack

**Backend:**
- **Rust** - Bahasa pemrograman utama untuk backend yang dipilih karena performanya yang tinggi dan keamanan memorinya.
- **Rocket v0.5.1** - Framework web untuk membangun API yang menangani request pencarian teks. Rocket menyediakan routing dan JSON handling yang mudah digunakan.
- **Rayon v1.11** - Library untuk pemrosesan paralel yang memungkinkan pencarian teks berjalan secara concurrent pada multiple threads, sehingga lebih cepat saat memproses banyak file.
- **Serde v1.0** - Library untuk serialisasi dan deserialisasi data JSON, memudahkan pertukaran data antara backend dan frontend.
- **rocket_cors v0.6** - Middleware untuk menangani Cross-Origin Resource Sharing (CORS), diperlukan agar frontend dapat berkomunikasi dengan backend.

**Frontend:**
- **Vue.js v3.5.22** - Framework JavaScript untuk membangun user interface yang reaktif dan interaktif.
- **Vite v7.1.11** - Build tool modern yang menyediakan development server dengan Hot Module Replacement (HMR) untuk mempercepat proses development.
- **Tailwind CSS v4.1.17** - Framework CSS utility-first untuk styling yang cepat dan konsisten.
- **PostCSS & Autoprefixer** - Tools untuk memproses CSS dan menambahkan vendor prefixes secara otomatis.

Aplikasi ini menggunakan Rust untuk backend yang bertugas mencari teks di dalam file secara cepat dengan multi-threading, dan Vue.js untuk frontend yang menampilkan tampilan website agar mudah digunakan. Backend fokus pada kecepatan pemrosesan pencarian, sedangkan frontend fokus pada kemudahan pengguna saat upload file dan melihat hasil pencarian.

---

## Source Code and Explanation

## Screenshot
### Upload File (.txt)
![Screenshot Upload File](./screenshot/upload_file_txt.png)
### Masukan 2 Kata yang ingin Dicari dalam File
![Screenshot Upload File](./screenshot/search_2_word.png)
### Hasil Pencarian berdasarkan Kata yang Dicari
![Screenshot Upload File](./screenshot/result_word.png)
### Hasil Pencarian Kata per-File 
![Screenshot Upload File](./screenshot/result_detail_1.png)
![Screenshot Upload File](./screenshot/result_detail_2.png)
![Screenshot Upload File](./screenshot/result_detail_3.png)
![Screenshot Upload File](./screenshot/result_detail_4.png)
![Screenshot Upload File](./screenshot/result_detail_5.png)
![Screenshot Upload File](./screenshot/result_detail_6.png)
## Conclusion
tunggu seendaknya technology stack udah teisi baru kulanjut