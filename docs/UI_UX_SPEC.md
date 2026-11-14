# Mister Download Manager (MDM) — UI/UX Specification

Dokumen spesifikasi desain antarmuka dan pengalaman pengguna (UI/UX) untuk pembaruan arsitektur visual Mister Download Manager. Desain berlandaskan prinsip *clean-room*, ergonomi desktop modern, densitas informasi optimal, serta performa rendering native 60 FPS berbasis Rust + egui.

---

## 1. Direct Video Resolution Popup (Independent Modal Viewport)

### 1.1. Konsep & Arsitektur Trigger
Popup modal instan independen yang terpicu saat ekstensi peramban (Chrome, Firefox, Edge) mendeteksi stream media di tab aktif atau pengguna menekan tombol *Download Video* di overlay peramban.

* **Non-blocking Execution**: Dialog berjalan pada viewport mandiri via `mdm-app` tanpa perlu merender atau membangunkan (*wake-up*) jendela utama MDM dari status system tray.
* **Handshake Native Messaging**: Ekstensi mengirimkan manifes video terurai (DASH/HLS/MPD/m3u8/direct streams) via IPC Native Messaging stdin/stdout berformat JSON kompak.

### 1.2. Dimensi & Posisi Viewport
* **Dimensi Standar**: Lebar 520 px, Tinggi 460 px (DPI-scaled secara dinamis).
* **Posisi Default**: Mengikuti posisi pointer pengguna saat klik, dengan *bounding clamp* otomatis agar seluruh dialog tetap berada dalam viewport monitor aktif (tidak terpotong tepi layar).
* **Window Frame**: Borderless HUD minimalis dengan sudut tajam/chamfered 2 px dan border aksen neon 1 px.

### 1.3. Anatomi Layout Antarmuka

```
+-----------------------------------------------------------------------+
| [>] MDM Media Sniffer :: Stream Analyzer                      [_] [X] |
+-----------------------------------------------------------------------+
| [Thumbnail]  Cyberpunk 2077 - Official Phantom Liberty 4K Trailer     |
| [ 160x90  ]  Source: youtube.com/@CyberpunkGame | Duration: 02:44     |
| [ Preview ]  Audio Codec: Opus 160kbps | Direct Stream: HTTPS / HLS   |
+-----------------------------------------------------------------------+
| [ Video + Audio (Muxed) ]  [ Audio Only ]  [ Video Only ]  [ Subs ]   |
+-----------------------------------------------------------------------+
| Resolution     Format    Codec       Bitrate      Est. Size    Status |
|-----------------------------------------------------------------------|
| (*) 2160p 4K   MKV       AV1         24.5 Mbps    482.4 MB     [Best] |
| ( ) 1440p 2K   MP4       VP9         12.1 Mbps    238.1 MB            |
| ( ) 1080p FHD  MP4       H.264       6.4 Mbps     126.0 MB    [Fast]  |
| ( ) 720p HD    MP4       H.264       3.2 Mbps     63.0 MB             |
| ( ) 480p SD    MP4       H.264       1.5 Mbps     29.5 MB             |
+-----------------------------------------------------------------------+
| Subtitles: [x] English (Auto-generated)  [ ] Indonesian  [+ More]     |
| Save To:   D:/Downloads/Videos/Cyberpunk/                 [ Browse ]  |
| Category:  [ Video (auto) v ]      Queue: [ Download Immediately v ]  |
+-----------------------------------------------------------------------+
| [>] Download Now (Enter)   [+] Add to Queue   [Audio MP3]   [ Cancel ]|
+-----------------------------------------------------------------------+
```

### 1.4. Interaksi & Kontrol Keyboard
* `Enter` / `Numpad Enter`: Menjalankan *Download Now* pada stream terpilih.
* `Esc`: Membatalkan dan menutup viewport popup tanpa alokasi memori lanjutan.
* `Up` / `Down` Arrow: Navigasi cepat antar baris resolusi.
* `1` - `5`: Shortcut langsung memilih resolusi 1 (tertinggi) hingga 5.
* `M`: Toggle cepat tab *Audio Only* (MP3/Opus/FLAC extractor).
* `Space`: Toggle pilihan subtitle checkbox.

---

## 2. Futuristic Sci-Fi Download Progress Window

Pengganti dialog transfer klasik IDM dengan tampilan instrumen telemetri sci-fi / HUD militer yang informatif, presisi, dan bebas distraksi.

### 2.1. Dimensi & Grid Frame
* **Dimensi**: 560 px x 440 px.
* **Canvas Style**: Latar belakang obsidian pekat (`#080c14`) dengan aksen grid bergaris halus 1 px berjarak 20 px (`rgba(27, 44, 68, 0.4)`).
* **Corner Treatment**: Chamfered 45 derajat 2 px pada sudut luar dialog.

### 2.2. Segment Worker Pulse Visualizer (Multi-Thread Pipeline)
* **Kanal Koneksi**: Grid visualisasi 8, 16, atau 32 worker thread horizontal.
* **Chunk State Coding**:
  * *Idle / Allocating*: Dark Slate (`#101d2e`).
  * *Connecting / Handshake*: Amber Pulse (`#ffb703`).
  * *Downloading Active*: Glowing Neon Cyan (`#00f0ff`) dengan efek gelombang sinus mikro pada thread dengan throughput tertinggi.
  * *Chunk Verified & Flushed*: Emerald Solid (`#00ff9d`).
  * *Worker Error / Retrying*: Crimson Blink (`#ff0055`).
* **Micro-meter**: 13 baris vertikal sub-segment bar yang menampilkan densitas blok data byte secara nyata tanpa interpolasi fiktif.

### 2.3. Spline Speed Waveform (Real-Time Bandwidth Graph)
* **Rendering Engine**: Kurva spline Bézier kubik yang dihitung per frame (60 FPS) dengan egui painter.
* **Timeline Window**: 60 detik histori transfer terakhir.
* **Grid Skala**: Sumbu vertikal adaptif (auto-scaling antara KB/s, MB/s, dan GB/s) dengan garis bantu horizontal dan penanda *Peak Speed* bertitik terang.
* **Shader Underfill**: Gradien vertikal dari neon cyan transparan (`rgba(0, 240, 255, 0.25)`) di bagian puncak hingga transparan total di baseline.

### 2.4. Telemetry Box & Server Intelligence
Panel indikator diagnostik real-time di bagian bawah jendela progress:

```
+-----------------------------------------------------------------------+
| TELEMETRY MATRIX                                                      |
| Server: edge-04.lax.streamcdn.net [104.26.12.89]  [FLAG: US]  RTT: 28ms|
| Proto:  HTTP/3 (QUIC-v1) | TLS 1.3 | X25519-ChaCha20-Poly1305         |
| Active: 16/16 Streams    | Buffer: 98% Health     | Disk: 0.4ms write |
| Speed:  24.85 MB/s (Peak: 26.10 MB/s) | ETA: 00:01:14 | Size: 1.82 GB |
+-----------------------------------------------------------------------+
```

* **Latency Indicator**:
  * Hijau Neon (`< 60 ms`): RTT optimal.
  * Kuning Amber (`60 - 150 ms`): Latensi sedang.
  * Merah Neon (`> 150 ms`): Server lambat / saturasi rute.
* **Protocol & Cipher Badges**: Label kompak dengan border 1 px yang mengidentifikasi transport layer aktif.

---

## 3. Mini Floating Always-On-Top Desktop HUD Widget

Widget desktop mengambang yang ringkas, transparan, dan tidak mengganggu aktivitas multitasking pengguna.

### 3.1. Dimensi & Mode Tampilan
1. **Mode Circular Gauge (Default)**: 128 px x 128 px.
2. **Mode Telemetry Bar (Docked)**: 240 px x 48 px saat ditempelkan ke tepi layar.

### 3.2. Circular Speed Gauge
* **Outer Arc (270° Sweep)**: Kecepatan download aktif saat ini relatif terhadap kapasitas link maksimum atau rata-rata rolling tertinggi.
* **Inner Arc (360° Closed)**: Persentase penyelesaian keseluruhan antrean download aktif.
* **Center Readout**:
  * Teks Utama: Angka kecepatan tebal (`24.8 MB/s`).
  * Teks Sekunder: Jumlah antrean aktif (`3 DL | ETA 04m`).
* **Visual Glow**: Pendaran neon cyan adaptif yang berdenyut selaras dengan fluktuasi kecepatan transfer data.

### 3.3. Quick Drop-Zone (Drag & Drop Target)
* **Target Menerima**:
  * Teks URL (HTTP, HTTPS, FTP).
  * File `.torrent` dan Magnet URI.
  * File `.metalink`.
  * File teks berisi daftar batch link (`.txt`).
* **State Hover / Dragover**:
  * Gauge luar berputar 180 derajat dan bertransformasi menjadi ring target bertitik hijau (`#00ff9d`).
  * Teks tengah berubah menjadi prompt: `[ DROP LINK HERE ]`.
  * Memberikan respon haptic visual (border pendaran melebar 4 px).

### 3.4. Window Behavior & Ergonomi
* **Always-On-Top (WS_EX_TOPMOST)**: Bertahan di atas semua jendela aplikasi.
* **Glassmorphism Backdrop**: Komposisi acrylic/mica pada Windows dengan keburaman latar (*gaussian blur radius 20*) dan alpha background 0.85.
* **Magnetic Edge Docking**: Menempel otomatis pada tepi layar monitor jika dilepaskan dalam jarak `< 24 px`.
* **Ghost Mode / Click-Through**: Pintasan `Win + Ctrl + M` membuat widget tembus klik (`WS_EX_TRANSPARENT`), sehingga tidak menghalangi klik pada aplikasi di bawahnya.
* **Double-Click**: Memulihkan dan menampilkan jendela utama MDM seketika.

---

## 4. Custom Wallpaper + Adaptive Contrast Engine

Sistem engine adaptasi cerdas yang menjamin seluruh teks, border kontrol, dan visualisasi HUD terbaca jelas (WCAG 2.2 AAA & APCA compliant) di atas wallpaper kustom atau transparansi desktop apa pun.

### 4.1. Algoritma Analisis Luminansi Ruang Warna (OKLab & APCA)
Tidak menggunakan rumus luminansi RGB linear usang. Menggunakan model persepsi warna **OKLab** ($L^*$) dan **APCA** (Accessible Perceptual Contrast Algorithm):

$$\text{Contrast Ratio} \ge 7.0:1 \quad (\text{WCAG AAA untuk Teks Normal})$$
$$\text{Contrast Ratio} \ge 3.0:1 \quad (\text{WCAG AA untuk Elemen UI & Border})$$

### 4.2. Mekanisme Dynamic Backing Scrim
* **Spatial 9-Point Sampling**: Engine mengambil sampel warna piksel di 9 kuadran tepat di belakang jendela widget/popup.
* **Luminance Classification**:
  * *Bright Background* ($L^* > 0.65$): Engine secara dinamis menyisipkan lapisan scrim gelap adaptif (`rgba(8, 12, 20, 0.82)`) dengan filter desaturasi lokal.
  * *Dark Background* ($L^* < 0.35$): Lapisan scrim tetap transparan ringan (`rgba(8, 12, 20, 0.45)`) untuk memaksimalkan efek sci-fi HUD.
  * *High Variance / Busy Wallpaper*: Tingkat blur ditingkatkan dari 12 px menjadi 28 px untuk meredam noise visual di balik teks kritis.
* **Text Legibility Shaders**: Teks telemetri memiliki drop-shadow kontur directional 1 px (`rgba(0, 0, 0, 0.85)`) yang otomatis diaktifkan saat nilai kontras latar turun di bawah ambang batas aman.

---

## 5. Command Palette (Ctrl+K) & Radial Quick Actions (Pie Menu)

Navigasi kilat untuk pengguna tingkat lanjut tanpa harus mengarungi menu bar tradisional.

### 5.1. Command Palette (`Ctrl + K` / `Cmd + K`)
* **Viewport**: Modal tengah layar berukuran 600 px x 380 px dengan input field fokus langsung.
* **Fuzzy Search Engine**: Pencarian toleran salah ketik untuk:
  * Tindakan aplikasi (*e.g.*, `Resume All`, `Pause All`, `Speed Limit 5MB`, `Verify Hash`).
  * Download aktif berdasarkan nama file atau domain.
  * Kategori dan antrean scheduler (*e.g.*, `Move to Night Queue`).
  * Pengaturan preferensi (*e.g.*, `Toggle Dark Mode`, `Theme: Futuristic`).
* **Visual Syntax**: Menampilkan badge shortcut keyboard di sisi kanan setiap baris hasil pencarian.

### 5.2. Radial Quick Actions (Pie Menu)
Menu lingkaran berbasis gestur yang muncul pada posisi kursor saat pengguna menekan dan menahan tombol tengah mouse (*middle-click*) atau *right-click* gesture pada baris download atau widget HUD.

```
                  [ 1. RESUME / START ]
                           (N)
         [ 8. VERIFY HASH ] \ | / [ 2. SPEED LIMITER ]
                     (NW)    \|/   (NE)
     [ 7. QUEUE TRANSFER ] -- O -- [ 3. OPEN FILE/DIR ]
                     (SW)    /|\   (SE)
         [ 6. DELETE/TRASH] / | \ [ 4. COPY URL/INFO ]
                           (S)
                   [ 5. PAUSE / STOP ]
```

* **8-Slice Angular Partitioning**: Setiap aksi memiliki sektor 45 derajat dengan ikon simbolik dan label singkat.
* **Dead-Zone Tengah**: Radius 16 px di bagian tengah mencegah kesalahan trigger saat kursor tidak bergerak sengaja.
* **Muscle Memory Flick**: Pengguna berpengalaman dapat melakukan flick cepat ke arah tertentu (*misal:* flick ke arah bawah untuk langsung Pause) dalam `< 100 ms` tanpa perlu menunggu animasi menu selesai terbuka.
* **Haptic Visual Feedback**: Sektor yang disorot langsung menyala dalam aksen neon cyan dengan ekspansi radius sebesar 4 px.

---

## 6. Integrasi Tema & Spesifikasi Token

Konfigurasi tema diwujudkan dalam file tema TOML resmi MDM pada path:  
`crates/app/assets/themes/futuristic.toml`

### Ringkasan Token Warna & Metrik:
* **Background Utama**: `#080c14` (Deep Space Obsidian)
* **Surface / Panel**: `#0d1524` (Cyberpunk Sub-surface)
* **Aksen Utama**: `#00f0ff` (Luminescent Cyan)
* **Status Sukses / Optimal**: `#00ff9d` (Neon Emerald)
* **Status Warning / Latensi Sedang**: `#ffb703` (Neon Amber)
* **Status Error / Kritis**: `#ff0055` (Neon Crimson)
* **Border & Grid**: `#1b2c44` (Telemetry Grid Navy)
* **Radius Sudut**: `2 px` (Crisp Chamfered Cybernetic Corners)
