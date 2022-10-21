################
##### Builder
FROM node:buster as builder

# download and install protoc binary/includes (apk would install old version)
RUN mkdir /home/protobuf && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v21.8/protoc-21.8-linux-x86_64.zip -O /home/protoc-21.8-linux-x86_64.zip && \
    unzip /home/protoc-21.8-linux-x86_64.zip -d /home/protobuf && \
    cp -r /home/protobuf/bin/* /usr/bin && \
    chmod +x /usr/bin/protoc && \
    cp -r /home/protobuf/include/* /usr/include

# download and install protoc-gen-grpc-web
RUN wget -O /usr/bin/protoc-gen-grpc-web https://github.com/grpc/grpc-web/releases/download/1.4.1/protoc-gen-grpc-web-1.4.1-linux-x86_64 && \
    chmod +x /usr/bin/protoc-gen-grpc-web

# download, build and install protobuf-javascript
RUN npm install -g @bazel/bazelisk
RUN git clone https://github.com/protocolbuffers/protobuf-javascript /home/protobuf-javascript
WORKDIR /home/protobuf-javascript
# use specific version
RUN git reset --hard 3ff6090f139d71453062fb96c66e9aff801709c2
RUN npm install --silent && \
    npm run build && \
    cp /home/protobuf-javascript/bazel-bin/generator/protoc-gen-js /usr/bin && \
    chmod +x /usr/bin/protoc-gen-js

# set working directory
WORKDIR /usr/src/app

# build react app
COPY package.json package-lock.json ./
COPY src ./src
COPY proto ./proto
COPY public ./public
RUN npm install --silent
RUN npm run build

# create express app
COPY express/index.js express/package.json /usr/src/app/express/
WORKDIR /usr/src/app/express
RUN npm install --production=true --silent


################
##### Runtime
FROM alpine:3.16.2 as runner

# create node user (copied from node:alpine)
RUN addgroup -g 1000 node && \
    adduser -u 1000 -G node -s /bin/sh -D node

# add nodejs and dumb-init
RUN apk add --no-cache nodejs-current
RUN apk add --no-cache dumb-init

# set to production environment
ENV NODE_ENV production

# get express app from builder
COPY --from=builder /usr/src/app/build /usr/src/app/build
COPY --from=builder /usr/src/app/express /usr/src/app/express

# set the working dir to express app
WORKDIR /usr/src/app/express

# run it
EXPOSE 3000
USER node
CMD ["dumb-init", "node", "index.js"]