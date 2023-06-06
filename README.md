[![Check Build](https://github.com/angelsflyinhell/ping/actions/workflows/check.yml/badge.svg)](https://github.com/angelsflyinhell/ping/actions/workflows/check.yml)
[![Docker Image](https://github.com/angelsflyinhell/ping/actions/workflows/deploy.yml/badge.svg)](https://github.com/angelsflyinhell/ping/actions/workflows/deploy.yml)

# ping
Barebones, anonymous chat platform as a web server using SQLite and Rust.

## Usage
 In order to use ping, you will need some sort of client to interact with the server as ping is only a web server.
 As it has a very limited API, building your own client shouldn't be too difficult, if you're into programming.
 If not, you can look out for other people's clients or use one of the following:
 - [ping-cli](/) [PLANNED: UNDER CONSTRUCTION]
 - more coming soon..!

## Host it yourself with Docker

```bash
docker pull ghcr.io/angelsflyinhell/ping:latest
docker run -d -p 8080:80 ghcr.io/angelsflyinhell/ping:latest
```

## Develop your own client
You can find the API documentation [here](./API.md).
Finished clients can be added to the list above by creating a pull request.