## Tutorial 8: Event-Driven Architecture - Subscriber

### a. What is AMQP?
**AMQP (Advanced Message Queuing Protocol)** adalah sebuah standar protokol jaringan terbuka (*open standard protocol*) pada *application layer* yang dirancang khusus untuk *message-oriented middleware* (seperti RabbitMQ). Protokol ini memungkinkan berbagai sistem atau aplikasi (seperti *publisher* dan *subscriber*) untuk saling berkomunikasi dan bertukar data secara asinkron, aman, dan andal melalui mekanisme antrian pesan (*message queuing*) dan perutean (*routing*).

### b. What does `guest:guest@localhost:5672` mean?
String tersebut adalah URL atau *Connection URI* yang digunakan oleh aplikasi kita untuk melakukan autentikasi dan terhubung ke *Message Broker* (RabbitMQ). Berikut adalah rincian dari setiap bagiannya:
- **`guest` pertama**: Merupakan *username* (nama pengguna) default bawaan dari RabbitMQ.
- **`guest` kedua**: Merupakan *password* (kata sandi) default untuk username tersebut.
- **`localhost`**: Menunjukkan alamat *host* atau server tempat RabbitMQ berjalan. `localhost` berarti server tersebut berjalan di komputer lokal (komputer kita sendiri, yang dalam tutorial ini dijalankan via Docker).
- **`5672`**: Merupakan nomor *port* standar/default yang digunakan oleh protokol AMQP untuk jalur komunikasi pengiriman pesan.

## Simulation slow subscriber
![slowsubscriber](slow-subscriber.png)

## Running at least three subscribers

![Multiple Subscribers Terminal](multiple-subscribers.png)
![Queue Reduces Faster](queue-reduces-faster.png)

**1. Explanation/Reflection of why it is like that:**
Ketika kita menjalankan tiga *subscriber* secara bersamaan (sambil tetap terhubung ke antrean/ *queue* yang sama di RabbitMQ), sistem menerapkan pola *Competing Consumers*. RabbitMQ secara otomatis mendistribusikan (memecah) beban pesan-pesan yang masuk ke tiga *subscriber* tersebut secara *Round-Robin* atau berdasarkan ketersediaan.

Inilah mengapa di terminal terlihat pesan diproses secara terpisah oleh masing-masing konsol (tidak ada pesan ganda yang diproses dua kali). Karena ada 3 "pekerja" yang memproses antrean secara paralel, penumpukan pesan (*bottleneck*) teratasi dan grafik *Queued messages* di RabbitMQ menurun jauh lebih cepat dibandingkan saat hanya ada 1 *subscriber*. Ini membuktikan bahwa arsitektur *Event-Driven* sangat mudah di-skalakan (*scalable*).

**2. Take a look at the code of publisher and subscriber, do you see something to improve?**
Ya, ada beberapa hal yang bisa diperbaiki dari kode *publisher* dan *subscriber* saat ini untuk level produksi:
* **Hardcoded Connection String:** URL RabbitMQ (`amqp://guest:guest@localhost:5672`) ditulis langsung di dalam kode (*hardcoded*). Seharusnya kredensial dan URL ini dimasukkan ke dalam *Environment Variables* (`.env`) agar lebih aman dan mudah diubah saat *deployment*.
* **Penggunaan `thread::sleep`:** Menggunakan `std::thread::sleep` dalam sistem yang memproses pesan secara asinkron bisa menahan/memblokir seluruh *thread*. Sebaiknya gunakan *asynchronous sleep* (seperti `tokio::time::sleep`) jika sistemnya berbasis *async*.
* **Tidak Ada Mekanisme Error Handling / Retry:** Jika *subscriber* gagal memproses pesan (misalnya karena koneksi *database* mati), pesan tersebut akan dianggap selesai atau hilang. Seharusnya ada mekanisme *Dead Letter Queue* (DLQ) atau *Retry* jika `handle()` mengembalikan nilai `Error`.