# rusty-paste
CLI pastebin and server with rust. Both the server and client are in `/server` and `/client` respectively.

To build the server, run:

```cargo build --release --bin server``` 

TO build the client, run:

```cargo build --release --bin client``` 

## Usage

```
Usage: client.exe --url <URL> <CONTENT>

Arguments:
  <CONTENT>

Options:
  -u, --url <URL>
  -h, --help       Print help
  -V, --version    Print version      
```

## Features

**NOTE: For more information, check out the README pages in each of the client and server folders (WIP)**

### Server: 
- Basic pastebin with storage and short, 8-key id, not requiring any copying
- Dynamically changing reset time (Resets first paste content when over limit, definitely a feature)
- SSL-enabled by default (WIP)

### Client:
- Simple, lightweight and performant code
- Optional, all client features can be done with more time using cURL
- Clipboard features, auto copy from CLI (WIP)

## Why?
For fun :) But, also because sometimes, for a local self-hosted setup, a lot of people just want a dumping ground for pastes that can be easily copied and pasted, without any frills.