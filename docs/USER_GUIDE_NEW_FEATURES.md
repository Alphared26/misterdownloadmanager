# Panduan Penggunaan Fitur Baru — Mister Download Manager (Next-Gen)

Panduan komprehensif cara kerja dan penggunaan fitur-fitur baru pada Mister Download Manager.

---

## 1. Direct Video Resolution Popup (Tanpa Buka Window Utama)
* **Cara Pakai:**
  1. Buka halaman video (misal YouTube, Vimeo, atau stream video di web manapun).
  2. Klik tombol melayang **Download Video** yang muncul di atas video.
  3. Menu pop-up resolusi langsung muncul di tempat (menampilkan pilihan 4K, 1080p, 720p, 480p, dan opsi *Audio MP3*).
  4. Klik kualitas yang diinginkan — unduhan langsung berjalan tanpa perlu membuka jendela utama aplikasi terlebih dahulu.

---

## 2. Pengunduh Media Sosial (TikTok, Instagram, Twitter/X, Pinterest)
* **Fitur:**
  * **TikTok:** Otomatis mengambil video resolusi tertinggi tanpa watermark (*clean stream*).
  * **Instagram:** Mendownload Reels, Stories, video postingan, dan seluruh carousel album foto.
  * **Twitter/X:** Menangkap video dan GIF kualitas tertinggi dari sebuah tweet.
* **Cara Pakai:**
  * Cukup buka postingan sosmed di browser; ekstensi otomatis membaca cookie sesi aktif (sehingga akun private yang sudah login tetap bisa didownload). Klik tombol ekstensi MDM atau tekan opsi *Download with MDM*.

---

## 3. Tema Futuristik Cyberpunk (Sci-Fi HUD)
* **Cara Mengaktifkan:**
  * Buka menu **View → Theme → Futuristic**.
* **Keunggulan Visual:**
  * Palet warna obsidian pekat (`#080c14`) dan aksen neon cyan (`#00f0ff`).
  * Bar segmen multi-thread dengan animasi denyut (*pulse activity*) pada koneksi aktif.
  * Kurva kecepatan download/upload berupa gelombang *Bézier spline waveform* dinamis real-time.
  * Kotak telemetri instrumen: ping latensi server, hitungan retry paket, dan bendera negara server (`mdm-geo`).

---

## 4. Custom Wallpaper & Kontras Adaptif Otomatis
* **Cara Menggunakan Wallpaper:**
  * Masuk ke **Options → Theme / Appearance → Custom Wallpaper**.
  * Pilih file gambar (PNG, JPG, atau WebP) dari disk.
  * Atur slider *Opacity* (transparansi) dan *Blur*.
* **Mekanisme Kontras Adaptif:**
  * MDM otomatis menghitung rata-rata luminansi latar (WCAG 2.1).
  * Jika gambar terang: teks dan ikon otomatis charcoal/navy dengan batas gelap agar tetap terbaca.
  * Jika gambar gelap: teks otomatis neon cyan/putih menyala dengan aksen glowing.

---

## 5. Mini Floating Always-On-Top Desktop HUD Widget
* **Fungsi:** Widget melayang kecil di layar desktop untuk memantau speed download/upload tanpa membuka window besar.
* **Cara Pakai:**
  * Buka menu **Tasks → Show HUD Widget** (atau shortcut `Alt + W`).
  * Widget berbentuk lingkaran/lingkaran neon yang selalu berada di paling atas (*Always-on-Top*).
  * Menampilkan kecepatan transfer MB/s atau KB/s dan jumlah download yang sedang berjalan.
  * **Drag & Drop:** Seret link dari browser atau file `.torrent` langsung ke widget untuk memulai download instan.
  * Klik dua kali (double click) untuk membuka jendela utama; klik kanan untuk menyembunyikan widget.

---

## 6. Command Palette (`Ctrl + K` / Spotlight Search)
* **Fungsi:** Pencarian cepat berbasis keyboard untuk semua file dan aksi kontrol.
* **Cara Pakai:**
  * Tekan `Ctrl + K` di jendela manapun.
  * Ketik nama file atau tag (misal `#video`, `#tiktok`, `#document`).
  * Gunakan panah atas/bawah dan tekan `Enter` untuk langsung memilih aksi (*Pause All*, *Resume All*, *Gaming Mode*, dll).

---

## 7. Radial Quick Actions (Pie Menu)
* **Fungsi:** Menu konteks cepat melingkar berbasis gerakan kursor mouse.
* **Cara Pakai:**
  * Klik kanan pada daftar unduhan aktif.
  * Empat kuadran instan akan muncul melingkari kursor:
    * Kuadran Atas-Kanan: *Boost Threads* (menambah jumlah koneksi simultan).
    * Kuadran Bawah-Kanan: *Open Folder* (buka lokasi berkas di Explorer).
    * Kuadran Bawah-Kiri: *Copy Link* (salin URL unduhan ke clipboard).
    * Kuadran Atas-Kiri: *Pause / Resume*.
  * Gerakkan mouse ke arah opsi yang diinginkan untuk memilih dalam hitungan milidetik.

---

## 8. Profil Bandwidth Pintar (Smart Bandwidth Profiles)
* **Pilihan Profil:**
  1. **Gaming Mode:** Membatasi transfer rate latar belakang ke 250 KB/s dengan koneksi minimal agar ping game/voice chat tidak lag.
  2. **Work Hours:** Alokasi bandwidth stabil untuk keperluan jam kantor/meeting.
  3. **Night Boost:** Mode kecepatan penuh tanpa batas kecepatan dengan hingga 32 koneksi segmen simultan.

---

## 9. Auto-Refresh Expired Links
* **Masalah:** Download file besar sering gagal di 90% karena token waktu download habis (HTTP 403/410).
* **Solusi Otomatis:**
  * MDM secara otomatis membuka tab latar belakang tersembunyi ke halaman web asal, mengambil token unduhan baru, dan menyambungkan kembali koneksi unduhan tanpa mereset byte yang telah selesai diunduh.

---

## 10. In-Flight Media Previewer
* **Fungsi:** Memeriksa dan memutar file video/audio yang baru terunduh sebagian (mulai 15% - 20%).
* **Cara Pakai:**
  * Klik kanan file video yang sedang berjalan → pilih **Preview in Player**.
  * MDM membedah header kontainer MP4/WebM/MP3 dari file parsial `.mdm.part` sehingga Anda bisa memastikan kualitas dan keaslian video sebelum download tuntas.

---

## 11. Recursive Site Grabber
* **Fungsi:** Mengunduh seluruh berkas dari suatu website berdasarkan hierarki folder.
* **Cara Pakai:**
  * Buka menu **Tasks → Run Site Grabber**.
  * Masukkan URL root, tentukan kedalaman perayapan (*depth* 1 hingga 5), dan centang tipe berkas target (misal Dokumen PDF, Gambar, atau Arsip ZIP).

---

## 12. Checksum Verifier & Sound Synthesizer
* **Verifikasi Hash:** Mendukung kalkulasi instan MD5 dan SHA-256 pasca-unduh untuk memastikan file tidak korup atau disisipi malware.
* **Sound Synthesizer:** Menggantikan bunyi klik klasik dengan nada synth modern (C5-C6 ascending chime saat selesai, soft descending tone saat error).
