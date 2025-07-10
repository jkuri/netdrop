# Netdrop

Netdrop is a simple file sharing web application that allows users to upload and download files. It is built using Rust, React and TailwindCSS.

## Demo

Visit [https://netdrop.jankuri.eu](https://netdrop.jankuri.eu) to see it in action.

## Docker

To run the application using Docker, simply build and run the container:

```bash
docker build -t netdrop .
docker run --rm -p 8000:8000 netdrop
```

Or run pre-build image from Docker Hub:

```bash
docker run --rm -p 8000:8000 jkuri/netdrop
```

The application will be immediately available at `http://localhost:8000` and be fully functional.

## Development

To run the application in development mode, first install dependencies:

```bash
make install
```

Build the web frontend (needed for main program as it embeds the frontend in the binary):

```bash
make build-web
```

Run the web frontend in development mode:

```bash
make dev
```

Run the API in development mode (in separate terminal window):

```bash
make build
cargo run
```

The application will be available at `http://localhost:5173` and will automatically reload when changes are made to the source code.

## License

MIT
