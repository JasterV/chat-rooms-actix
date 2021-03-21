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

## How to interact with the app

### Connect

First, you will need to connect to the app by web socket:

Ex: ```ws://localhost:<$PORT>```

### Send messages

To execute any operation you have to send a json which has the following format:

```json
{
    "ty": "Create" | "Join" | "Leave" | "Msg",
    "data": ""
}
```

### Available operations

+ #### Create a room: 
    ```json
    {
        "ty": "Create",
        "data": ""
    }
    ```
    + Response:
        ```json
        {
            "ty": "Info",
            "data": <room-id>
        }
        ```

+ #### Join a room: 
    ```json
    {
        "ty": "Join",
        "data": <room-id>
    }
    ```
    + Response:
        ```json
        {
            "ty": "Info",
            "data": "Joined"
        }
        ```

+ #### Leave a room: 
    ```json
    {
        "ty": "Leave",
        "data": ""
    }
    ```
    + Response:
        ```json
        {
            "ty": "Info",
            "data": "Room leaved"
        }
        ```

+ #### Send a msg to the room: 
    ```json
    {
        "ty": "Msg",
        "data": "Hello, World"
    }
    ```


### Error responses

If an error occurs, the server will send back a json with the following format:

```json
{
    "ty": "Err",
    "data" <error-message>
}
```

## Todo List

+ Add a redis db to store messages & active rooms