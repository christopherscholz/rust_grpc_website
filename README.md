Personal Multi-Page-Website with client server separation using a gRPC-web API for Christopher Scholz build with [Tonic](https://github.com/hyperium/tonic).
* Using `React`, `ReactRouter` to create the different pages
* gRPC-web API via `grpc-web` and `Tonic` (Server, including CORS Headers)

The app can be run locally or via docker.

For the `local` setup run
* server
    ```
    cd server
    cargo run
    ```
* client
    ```
    cd client
    npm start
    ```

For the `docker` setup run
```
docker-compose up
```

The javascript proto Client stub is generated via 
```
cd client
protoc --proto_path=proto page.proto --grpc-web_out=import_style=commonjs,mode=grpcweb:src/pages --js_out=import_style=commonjs,binary:src/pages
```