# slog_examples


# console

```
version: 0.1.0
 host: localhost
  port: 8080
   Mar 06 14:14:04.650 DEBG started
   Mar 06 14:14:04.651 DEBG 2 workers
   Mar 06 14:14:04.651 DEBG request, from: example.com
   Mar 06 14:14:04.651 INFO processing files, number_of_files: 12467, worker_pool: 25
   Mar 06 14:14:04.652 ERRO worker failed, worker_id: 1
   Mar 06 14:14:04.652 CRIT server can't continue to work
```

# console with custom serializer

```
starting port=8080 host=test-server-2 build-id=7.3.3-abcdef
 listening port=8080 host=test-server-2 build-id=7.3.3-abcdef
 connected port=42381 peer_addr=82.9.9.9 port=8080 host=test-server-2 build-id=7.3.3-abcdef
version message received length=2 port=42381 peer_addr=82.9.9.9 port=8080 host=test-server-2 build-id=7.3.3-abcdef
: connected port=18230 peer_addr=8.8.8.8 port=8080 host=test-server-2 build-id=7.3.3-abcdef
  weak encryption requested algo=xor port=42381 peer_addr=82.9.9.9 port=8080 host=test-server-2 build-id=7.3.3-abcdef
0.1.0
 response sent length=8 port=42381 peer_addr=82.9.9.9 port=8080 host=test-server-2 build-id=7.3.3-abcdef
  disconnected port=42381 peer_addr=82.9.9.9 port=8080 host=test-server-2 build-id=7.3.3-abcdef
host:  message received length=2 port=18230 peer_addr=8.8.8.8 port=8080 host=test-server-2 build-id=7.3.3-abcdef
localhost
 response sent length=8 port=18230 peer_addr=8.8.8.8 port=8080 host=test-server-2 build-id=7.3.3-abcdef
  port disconnected port=18230 peer_addr=8.8.8.8 port=8080 host=test-server-2 build-id=7.3.3-abcdef
 internal error port=8080 host=test-server-2 build-id=7.3.3-abcdef
 exit port=8080 host=test-server-2 build-id=7.3.3-abcdef
: 8080

```

# file

```
{"msg":"worker failed","level":"ERRO","ts":"2020-03-06T14:14:04.651867+00:00","port":8080,"host":"localhost","version":"0.1.0","worker_id":1}
{"msg":"server can't continue to work","level":"CRIT","ts":"2020-03-06T14:14:04.652198+00:00","port":8080,"host":"localhost","version":"0.1.0"}
```
