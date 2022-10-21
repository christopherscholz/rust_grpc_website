################
##### Builder
FROM node:buster as builder

# RUN apt-get update && apt-get install -y \
#     protobuf-compiler \
#     libprotobuf-dev

RUN mkdir /home/protobuf && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v21.8/protoc-21.8-linux-x86_64.zip -O /home/protoc-21.8-linux-x86_64.zip && \
    unzip /home/protoc-21.8-linux-x86_64.zip -d /home/protobuf && \
    cp -r /home/protobuf/bin/* /usr/bin && \
    chmod +x /usr/bin/protoc && \
    cp -r /home/protobuf/include/* /usr/include

RUN wget -O /usr/bin/protoc-gen-grpc-web https://github.com/grpc/grpc-web/releases/download/1.4.1/protoc-gen-grpc-web-1.4.1-linux-x86_64 && \
    chmod +x /usr/bin/protoc-gen-grpc-web

RUN npm install -g @bazel/bazelisk

RUN git clone https://github.com/protocolbuffers/protobuf-javascript /home/protobuf-javascript
WORKDIR /home/protobuf-javascript
RUN npm install --silent && \
    npm run build && \
    cp /home/protobuf-javascript/bazel-bin/generator/protoc-gen-js /usr/bin && \
    chmod +x /usr/bin/protoc-gen-js

# set working directory
WORKDIR /app

# add `/app/node_modules/.bin` to $PATH
ENV PATH /app/node_modules/.bin:$PATH

# install app dependencies
COPY package.json package-lock.json ./
COPY src ./src
COPY proto ./proto
COPY public ./public
RUN npm install --silent
RUN npm run build

################
##### Runtime
FROM node:alpine as runner

COPY --from=builder /app/build /app/build
WORKDIR /app
COPY express /app/express

WORKDIR /app/express
RUN npm install --silent

EXPOSE 3000
CMD ["npm", "start"]