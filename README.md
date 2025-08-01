# Rust Web Server

A multithreaded web server with graceful shutdown capabilities built in Rust, following [Chapter 21 of "The Rust Programming Language"](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) book.

## Features

- **Multithreaded request handling** using a custom thread pool
- **Graceful shutdown** with proper cleanup of resources
- **HTTP request parsing** and response generation
- **Static file serving** from the file system
- **Connection management** with configurable thread pool size
- **Signal handling** for clean server termination

## Architecture

The server implements:
- A custom `ThreadPool` for managing worker threads
- Request routing and HTTP response handling
- Graceful shutdown mechanism that allows in-flight requests to complete
- Resource cleanup to prevent memory leaks

## Installation

```bash
git clone https://github.com/alexmatcov/multithreaded-web-server.git
cd multithreaded-web-server
cargo build --release
```

# Usage

## Basic Server Setup

```bash
# Run in development mode
cargo run

# Run the release binary
./target/release/rust-webserver
```

The server will start on `127.0.0.1:7878` by default.

## Graceful shutdown

Send a termination signal to gracefully shut down the server:

```bash
# Using Ctrl+C in the terminal
# Or send SIGTERM
kill -TERM <pid>
```

## Configuration

The server can be configured by modifying the source code:
- **Port**: Change the binding address in `main.rs`
- **Thread pool size**: Adjust the number of worker threads
- **Static file directory**: Modify the file serving path

## Examples

### Accessing the server
```bash
# Basic request (to the main page)
curl http://127.0.0.1:7878/

# Simulate a slow request
curl http://127.0.0.1:7878/sleep
```

### Server responses
- `GET /` - Returns the main page
- `GET /sleep` - Simulates a slow request (for testing multithreading)
- `GET /hello.html` - Serves static HTML file
- Other paths return a 404 page

## Development

### Development server
```bash
cargo run
```

### Building for production
```bash
cargo build --release
```

## Learning Concepts

This project demonstrates advanced Rust concepts:

- **Multithreading and concurrency**
  - Custom thread pool implementation
  - Worker thread management
  - Channel-based communication

- **Network programming**
  - TCP listener and stream handling
  - HTTP request parsing
  - Response generation

- **Resource management**
  - Graceful shutdown patterns
  - Proper cleanup of threads and connections
  - RAII (Resource Acquisition Is Initialization)

- **Error handling**
  - Robust error handling with `Result<T, E>`
  - Graceful degradation on errors

- **System programming**
  - Signal handling
  - Process lifecycle management
  - File system operations

## Project Structure

```
src/
├── main.rs           # Server entry point and request handling
├── lib.rs            # Thread pool implementation
├── 404.html          # Response for unexpected requests
├── Cargo.lock        # Dependency lock file
├── Cargo.toml        # Project configuration and dependencies
└── hello.html        # HTML response for successful requests
```

## Performance Notes

- The server uses a fixed-size thread pool to limit resource usage
- Graceful shutdown ensures all in-flight requests complete before termination
- Static file serving is optimized for small to medium files