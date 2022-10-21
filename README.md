Personal Multi-Page-Website with client server separation using a gRPC-web API for Christopher Scholz build with [Tonic](https://github.com/hyperium/tonic).
* Using `React`, `ReactRouter` to create the different pages
* gRPC-web API via `grpc-web` and `Tonic` (Server, including CORS Headers)

The app can be run locally or via docker.

prerequisites for local setup (docker sets them up automatically)
* protoc installed https://github.com/protocolbuffers/protobuf/releases
* protobuf grpc-web generator installed https://github.com/grpc/grpc-web/releases/tag/1.4.1
* protobuf-javascript generator installed https://github.com/protocolbuffers/protobuf-javascript/releases

For the `local` setup run
* server
    ```
    cd server
    cargo run
    ```
* client (using react-script)
    ```
    cd client
    npm install
    npm start
    ```
* client (using express)
    ```
    cd client
    npm install
    npm run build
    cd express
    npm install
    npm start
    ```

For the `docker` setup run
```
docker-compose up
```