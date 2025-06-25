# How to run
## 1 Run Otel collector docker image:

```
╰─$ docker run -p 4318:4318 -p 4317:4317 -p 8889:8889 -p 8888:8888 -v $(pwd)/config/config.yaml:/etc/otelcol-contrib/config.yaml otel/opentelemetry-collector-contrib:0.97.0 2>&1 | tee collector-output.txt
```

## 2 Run Rust App
```
cargo run
```


