# netdrop

Netdrop is a simple file sharing web application that allows users to upload and download files. It is built using Rust, React and Tailwindcss.

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
