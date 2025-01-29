# Here's what the tech stack looks like:

Frontend: Rust, Leptos/WebAssembly

Backend: Rust, Leptos (For rendering html on the server) + Axum/Tokio + Tower (tracing, compression, etc)

## Methodology
All image processing is done on the server, while visualization and calculations are done on the client with HTML canvas
