# Tokio Redis Server

## Run Server

```sh
telnet localhost 6379
Trying ::1...
telnet: connect to address ::1: Connection refused
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello
good
awesome
```

```sh
~/D/r/concurrency> env RUST_LOG=info cargo run --bin redis

dummy redis server...
2024-05-12T08:01:16.580211Z  INFO redis: listen on: 0.0.0.0:6379
2024-05-12T08:01:42.956828Z  INFO redis: accept conn from: 127.0.0.1:56519
2024-05-12T08:01:46.568321Z  INFO redis: read 7 bytes
2024-05-12T08:01:46.568692Z  INFO redis: read: hello

2024-05-12T08:01:55.507736Z  INFO redis: read 6 bytes
2024-05-12T08:01:55.507824Z  INFO redis: read: good

2024-05-12T08:02:00.182905Z  INFO redis: read 9 bytes
2024-05-12T08:02:00.182996Z  INFO redis: read: awesome
```

## read from redis-cli

```sh
2024-05-12T08:13:08.634248Z  INFO redis: listen on: 0.0.0.0:6379
2024-05-12T08:15:19.671641Z  INFO redis: accept conn from: 127.0.0.1:60988
2024-05-12T08:15:19.673064Z  INFO redis: read 27 bytes
2024-05-12T08:15:19.673275Z  INFO redis: read: "*3\r\n$3\r\nget\r\n$1\r\na\r\n$1\r\nb\r\n"

2024-05-12T08:23:57.356846Z  INFO redis: listen on: 0.0.0.0:6379
2024-05-12T08:24:05.790368Z  INFO redis: accept conn from: 127.0.0.1:63894
2024-05-12T08:24:05.791523Z  INFO redis: read 17 bytes
2024-05-12T08:24:05.791823Z  INFO redis: read: "*1\r\n$7\r\nCOMMAND\r\n"
2024-05-12T08:24:09.500634Z  INFO redis: read 27 bytes
2024-05-12T08:24:09.500743Z  INFO redis: read: "*3\r\n$3\r\nget\r\n$1\r\nh\r\n$1\r\nl\r\n"
2024-05-12T08:24:13.155680Z  WARN redis: Connection 127.0.0.1:63894 closed
```
