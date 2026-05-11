## Tutorial 8: Event-Driven Architecture - Subscriber

### a. What is AMQP?
**AMQP (Advanced Message Queuing Protocol)** adalah sebuah standar protokol jaringan terbuka (*open standard protocol*) pada *application layer* yang dirancang khusus untuk *message-oriented middleware* (seperti RabbitMQ). Protokol ini memungkinkan berbagai sistem atau aplikasi (seperti *publisher* dan *subscriber*) untuk saling berkomunikasi dan bertukar data secara asinkron, aman, dan andal melalui mekanisme antrian pesan (*message queuing*) dan perutean (*routing*).

### b. What does `guest:guest@localhost:5672` mean?
String tersebut adalah URL atau *Connection URI* yang digunakan oleh aplikasi kita untuk melakukan autentikasi dan terhubung ke *Message Broker* (RabbitMQ). Berikut adalah rincian dari setiap bagiannya:
- **`guest` pertama**: Merupakan *username* (nama pengguna) default bawaan dari RabbitMQ.
- **`guest` kedua**: Merupakan *password* (kata sandi) default untuk username tersebut.
- **`localhost`**: Menunjukkan alamat *host* atau server tempat RabbitMQ berjalan. `localhost` berarti server tersebut berjalan di komputer lokal (komputer kita sendiri, yang dalam tutorial ini dijalankan via Docker).
- **`5672`**: Merupakan nomor *port* standar/default yang digunakan oleh protokol AMQP untuk jalur komunikasi pengiriman pesan.