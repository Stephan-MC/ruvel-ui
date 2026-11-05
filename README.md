# Ruvel

<p align="center">
  <strong>The developer experience of Laravel. The uncompromising performance of Rust.</strong>
</p>

<p align="center">
  <a href="https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/"><img src="https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/badge.svg" alt="Web Frameworks Benchmark Rank" /></a>
  <a href="https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/"><img src="https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/badge-language.svg" alt="Web Frameworks Benchmark Rust Rank" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-2024%20edition-orange.svg" alt="Rust 2024 Edition" /></a>
  <a href="https://crates.io/crates/ruvel"><img src="https://img.shields.io/badge/crates.io-v0.1.4-blue.svg" alt="Crates.io" /></a>
</p>

---

## 💡 Philosophy & Developer Experience (DX) Orientation

Modern web development has long faced a trade-off: **developer happiness vs. raw performance**.

For years, developers had to choose between the productivity, expressiveness, and joyful developer ergonomics of high-level MVC frameworks, or the lower-level, complex boilerplate required to achieve blazing speeds and memory efficiency.

**Ruvel bridges this divide.**

Built on Rust (2024 edition) and Tokio, Ruvel is designed for developers who love clean architecture, predictable structure, and elegant code, but refuse to compromise on throughput, latency, or concurrency.

- **Intuitive Architecture:** An immediately recognizable structure (`app/`, `bootstrap/`, `routes/`, `config/`, `database/`) that scales gracefully from prototypes to enterprise workloads.
- **Expressive Routing & Middleware:** Define routes fluently, group endpoints, and pass requests through clean, composable middleware layers using the classic "onion" pattern.
- **Zero-Boilerplate Controllers:** Simple, readable action methods with automatic response formatting for text, HTML, and JSON.
- **Service Container & Providers:** Two-phase bootstrapping (`register` and `boot`) with lock-free service resolution and singleton management.
- **Fearless Concurrency & Safety:** Guaranteed memory safety, thread safety, and zero garbage collection overhead without sacrificing expressiveness.

---

## ⚡ Performance Metrics

Ruvel delivers elite, bare-metal throughput and sub-millisecond latencies under extreme concurrency.

According to verified results from [The Benchmarker / Web Frameworks Benchmark](https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/):

| Metric | Concurrency 64 | Concurrency 256 | Concurrency 512 |
|:---|:---:|:---:|:---:|
| **Requests / Second** | **229,028** | **208,547** | **206,753** |
| **P50 Latency** | **0.28 ms** | **1.24 ms** | **2.41 ms** |
| **P90 Latency** | 0.39 ms | 1.75 ms | 3.52 ms |
| **P99 Latency** | **0.47 ms** | **2.08 ms** | **4.74 ms** |
| **P99.999 Latency** | 0.74 ms | 2.91 ms | 7.03 ms |
| **HTTP Errors** | **0** | **0** | **0** |
| **Timeouts / Socket Errors** | **0** | **0** | **0** |

### Benchmark Highlights

- 🏆 **Ranked #16 out of 392 frameworks** globally across all programming languages.
- 🦀 **Ranked #4 out of 33 frameworks** in the Rust ecosystem.
- Over **3.4 million requests** handled per test run with **0 dropped connections** and **0 error responses**.

[![Web Frameworks Benchmark](https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/badge.svg)](https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/)
[![Web Frameworks Benchmark](https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/badge-language.svg)](https://web-frameworks-benchmark.vercel.app/frameworks/rust/ruvel/)

---

## ❤️ Tribute to Laravel

> *"Laravel has always been about developer happiness, aesthetic syntax, and the pursuit of elegant architecture."*

Ruvel exists in humble homage to the **[Laravel](https://laravel.com)** framework and its creator, **Taylor Otwell**, along with the entire Laravel community. 

Laravel revolutionized the web ecosystem by proving that framework design should be human-centric. It elevated conventions over configuration, brought poetry to application structure, and demonstrated that writing backend code should be a joyful, creative craft.

Ruvel carries that torch into the systems programming landscape. It adopts the architectural conventions, clean mental models, and developer-first design patterns popularized by Laravel, bringing them to life in the high-performance async Rust ecosystem.

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.85+ (Rust 2024 edition)
- Cargo

### Installation & Run

```bash
# Clone the repository
git clone https://github.com/Stephan-MC/ruvel-ui.git
cd ruvel-ui

# Run in development mode
cargo run
```

For maximum performance benchmarks, build in release mode:

```bash
cargo run --release
```

### Environment Configuration

The server defaults to `0.0.0.0:3000`. You can configure the host and port via environment variables:

```bash
HOST=127.0.0.1 PORT=8080 cargo run --release
```

| Route | Handler | Description |
|:---|:---|:---|
| `GET /` | `WelcomeController@welcome` | Default welcome response |
| `GET /welcome` | Inline Route Handler | Greeting endpoint |

---

## 📂 Application Structure

The repository follows a clean, modular structure mirroring standard MVC conventions:

```text
├── main.rs                     # Tokio asynchronous runtime entrypoint
├── bootstrap/
│   └── app.rs                  # Application configuration & route bootstrapping
├── app/
│   ├── http/
│   │   └── controllers/        # Resource & action controllers (e.g. WelcomeController)
│   └── providers/              # Service providers (lifecycle bindings & boot routines)
├── routes/
│   ├── web.rs                  # Web browser routes & middleware groups
│   └── api.rs                  # API routes
├── config/                     # Application & environment configuration
├── database/
│   ├── migrations/             # Schema definitions and migrations
│   └── seeders/                # Database seeders
└── resources/                  # Templates, views, and static assets
```

### 1. Bootstrapping (`bootstrap/app.rs`)

Configuring the application is fluent and straightforward:

```rust
use crate::routes::web;
use ruvel::foundation::Application;

pub async fn create_app() -> Application {
    Application::configure("")
        .with_routing(web::routes)
        .create()
}
```

### 2. Defining Routes (`routes/web.rs`)

Define HTTP verb handlers with support for path parameters, grouping, and middleware:

```rust
use ruvel::routing::Router;
use crate::app::http::controllers::WelcomeController;

pub fn routes(router: &mut Router) {
    let controller = WelcomeController::new();

    router.get("/", move |req| controller.welcome(req));
    router.get("/welcome", |_| async { "Hello World from Ruvel" });
}
```

### 3. Middleware Pipeline

Middleware wraps request execution using the classic onion model:

```rust
app.with_middleware(|stack| {
    stack.append(LoggingMiddleware);
    stack.alias("auth", AuthMiddleware);
});
```

### 4. Service Container & Dependency Injection

Ruvel includes a type-keyed dependency injection container:

```rust
// Register a singleton
container.singleton(|| Arc::new(DatabaseService::new()));

// Resolve anywhere
let db = container.make::<DatabaseService>();
```

---

## 🗺️ Roadmap & Status

Ruvel is currently in active development (`0.1.x` preview).

- [x] High-performance HTTP/1.1 server engine (Keep-Alive, pipelining)
- [x] Radix-tree router with parameter extraction (`{id}`) and route grouping
- [x] Composable onion-style middleware pipeline (global, alias, group)
- [x] Type-keyed Dependency Injection Container & Service Providers
- [x] Resource controllers & flexible `Responsable` return types
- [ ] Query string map parser & multipart body parser
- [ ] Database layer & Active Record / Eloquent-style ORM
- [ ] Validation & Form Request extractors
- [ ] Session handling, cookies & authentication guards
- [ ] CLI tooling (Artisan-inspired generator commands)
- [ ] Template engine integration & view rendering

---

## 📄 License

Ruvel is open-source software licensed under the MIT License.
