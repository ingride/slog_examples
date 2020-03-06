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

# file

```
{"msg":"worker failed","level":"ERRO","ts":"2020-03-06T14:14:04.651867+00:00","port":8080,"host":"localhost","version":"0.1.0","worker_id":1}
{"msg":"server can't continue to work","level":"CRIT","ts":"2020-03-06T14:14:04.652198+00:00","port":8080,"host":"localhost","version":"0.1.0"}
```
