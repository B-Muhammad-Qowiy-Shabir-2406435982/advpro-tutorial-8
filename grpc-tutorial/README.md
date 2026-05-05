1. Perbedaan utama terletak pada pola komunikasi:

- Unary RPC: 1 request → 1 response
Cocok untuk operasi sederhana seperti login atau pembayaran.
- Server Streaming RPC: 1 request → banyak response
Cocok untuk pengambilan data bertahap seperti histori transaksi.
- Bi-directional Streaming RPC: client & server saling kirim stream
Cocok untuk komunikasi real-time seperti chat atau live updates.

2. Beberapa aspek penting:

- Authentication: memastikan identitas user (JWT, OAuth)
- Authorization: mengatur akses (role-based access control)
- Encryption: menggunakan TLS (HTTPS di gRPC)
- Data validation: mencegah input berbahaya
- Rate limiting: mencegah abuse / DDoS

3. Beberapa masalah yang sering muncul:

- Concurrency complexity (async & stream sulit dikelola)
- Memory leak jika stream tidak ditutup
- Deadlock / blocking jika channel tidak sinkron
- Error handling sulit (stream bisa gagal di tengah jalan)
- Scalability issue (banyak koneksi real-time)

4. Kelebihan & Kekurangan

= Kelebihan:
- Integrasi mudah dengan tokio::mpsc
- Cocok untuk async streaming
- Simple untuk implementasi

= Kekurangan:
- Bergantung pada channel (overhead memory)
- Perlu manual handling (close channel, error)
- Kurang fleksibel dibanding stream custom

5. Struktur Code untuk Reusability & Modularity

= Pisahkan:
- service logic
- proto definition
- transport layer

= Gunakan:
- module (mod)
- trait abstraction

= Hindari hardcode logic di handler

6. Untuk production-level, perlu:

- Validasi input (amount tidak negatif)
- Integrasi database
- Integrasi payment gateway
- Error handling (failed transaction)
- Logging & audit trail
- Idempotency (hindari double payment)

7. Dampak gRPC pada Arsitektur Sistem

= Positif:
- High performance (binary protocol)
- Strong contract (proto schema)
- Cocok untuk microservices

= Tantangan:
- Tidak semua platform native support
- Debugging lebih sulit (tidak human-readable)
- Butuh tooling tambahan

8. HTTP/2 vs HTTP/1.1 / WebSocket

= HTTP/2 (gRPC):
- Multiplexing (banyak request dalam 1 koneksi)
- Lebih cepat & efisien
- Built-in streaming

= Kekurangan:
- Lebih kompleks
- Tidak semua client support langsung

= Dibanding HTTP/1.1:
HTTP/1.1 → lebih sederhana, tapi lebih lambat

= Dibanding WebSocket:
WebSocket → cocok real-time, tapi tidak structured seperti gRPC

9. REST vs gRPC (Real-time)

= REST:
- request-response
- tidak real-time (butuh polling)

= gRPC (streaming):
- real-time communication
- lebih responsif
- efisien untuk data terus-menerus

10. Protocol Buffers vs JSON

= Protocol Buffers:
- Lebih kecil & cepat
- Strong typing (schema jelas)
- Lebih efisien

= Kekurangan:
- Tidak human-readable
- Perlu compile (protoc)

= JSON:
- Mudah dibaca & fleksibel
- Tapi lebih besar & lambat