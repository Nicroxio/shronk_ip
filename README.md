# Shronk-Ip-Rust
A rust implementation of the SHRoNK-IP [Standard](https://github.com/SHRoNK-Corporation/shronk-ip-spec/blob/main/RFS0001.md) 
## Build Instructions
`docker build .`
## Compose
```Compose
services:
  server:
    volumes:
      - ./dbip-city-lite-2023-10.mmdb:/app/dbip-city-lite-2023-10.mmdb
    build:
      context: .
      target: final
    ports:
      - 8000:8000
```
