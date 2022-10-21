Personal Multi-Page-Website with client server separation using a gRPC-web API for Christopher Scholz build with [Tonic](https://github.com/hyperium/tonic).
* grpc-web Client: `Express`, `React`, `ReactRouter`, `grpc-web` to create the different pages
* gRPC-web Server: `Tonic`, `Tonic-Web` which sets up gRPC-web over http/1.1 

The app can be run locally or via docker.

Running locally is a bit of a hassle, as you need to download the correct versions. Protobuf-Javascript even needs to be compiled, as the current release does not work properly. 

Prerequisites for local setup (docker sets them up automatically)
* protoc https://github.com/protocolbuffers/protobuf/releases/tag/v21.8
* protobuf grpc-web generator https://github.com/grpc/grpc-web/releases/tag/1.4.1
* protobuf-javascript generator @3ff6090 https://github.com/protocolbuffers/protobuf-javascript/tree/3ff6090f139d71453062fb96c66e9aff801709c2

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