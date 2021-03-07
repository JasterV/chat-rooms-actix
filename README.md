# Actix Chat rooms

> Chat room application backend which allows you to create, join, leave chat rooms & send message to other anonymous users. Built using Actix Web, Actix & Actix WebSocket actors

## Build

```sh
cargo build
```

## Run

```sh
cargo run
```

## Todo List

+ HeartBeat on ws session actor to check clients state
+ Add a redis db to store messages & active rooms