# Ruvel UI

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)

> **This framework is not ready for production.**
>
> This application runs on Ruvel `0.1.x`, an early preview of a Laravel-style framework for Rust. Routing, middleware, the service container, and controllers can be tried locally. Authentication, validation, sessions, a real database layer, TLS, and several builder hooks are missing or panic when called. Do not deploy this application.

Ruvel UI is the application skeleton for [Ruvel](https://github.com/Stephan-MC/ruvel). The layout follows a Laravel project: `bootstrap/`, `app/http/controllers/`, `routes/`, `config/`, and `database/`. The framework crate (`ruvel` `0.1.4`) supplies the HTTP server, router, middleware pipeline, service container, and controller trait. This repository is the application that wires them together.

---

## Installation

Clone the `origin` remote of this repository, then build it with Cargo. You need Rust 1.85 or newer (the 2024 edition) and Cargo.

```bash
git clone git@github.com:Stephan-MC/ruvel-ui.git
cd ruvel-ui
cargo run
```

`origin` is `git@github.com:Stephan-MC/ruvel-ui`. The same repository over HTTPS:

```bash
git clone https://github.com/Stephan-MC/ruvel-ui.git
cd ruvel-ui
cargo run
```

Cargo downloads `ruvel` `0.1.4` and the other dependencies from crates.io. The default branch on `origin` is `main`.

The server listens on `0.0.0.0:3000`. Override either side with the environment:

```bash
HOST=127.0.0.1 PORT=8080 cargo run
```

| Path | Response |
| --- | --- |
| `GET /` | `Welcome to Ruvel Again` |
| `GET /welcome` | `Hello World from Ruvel` |

---

## Application layout

```text
main.rs                         # Tokio entrypoint; calls bootstrap::app()
bootstrap/app.rs                # Application::configure, routes, listen
app/http/controllers/           # WelcomeController
app/providers/                  # Service providers (empty; registration is commented out)
routes/web.rs                   # Web routes
routes/api.rs                   # API routes (empty)
config/                         # Application config (empty)
database/migrations/            # Migrations (empty)
database/seeders/               # Seeders (empty)
resources/                      # Views and other assets (empty)
```

`bootstrap/app.rs` builds the application the way Laravel's `bootstrap/app.php` does:

```rust
let app = Application::configure("")
    .with_routing(web::routes)
    .create();
```

`routes/web.rs` registers the two routes above. `WelcomeController` implements the resource `Controller` trait and also exposes `welcome`, which is the action mounted on `/`.

---

## Framework features in use

These pieces are implemented in the `ruvel` crate this application depends on.

### Routing

`Router` keeps one radix tree per HTTP method.

- Verbs: `get`, `post`, `put`, `delete`, `patch`.
- Path parameters use `{name}` segments. A match fills `Request::params`, and `Request::param("id")` reads one value.
- The query string is stored raw on `Request::query`. Matching uses the path only.
- `router.group(&["web", "auth"], ...)` attaches middleware group or alias names to the routes inside the closure. An unknown name panics when the route is registered.
- A missing route returns `404 Not Found`. Global middleware still runs for that response.
- A handler may return any type that implements `Responsable`, including `&'static str` and `String`.

This application registers its routes through `with_routing` in `bootstrap/app.rs`. Register middleware before routes. Each route copies its pipeline at registration time, so routes added earlier keep an empty pipeline.

### Middleware

The pipeline is the Laravel onion. A layer receives the request and a `Next`. `next.run(request).await` continues inward, and the response walks back out.

`ApplicationBuilder::with_middleware` configures the stacks:

| Method | Effect |
| --- | --- |
| `append` | Add a layer at the end of the global stack. |
| `prepend` | Add a layer at the front. The latest prepend runs first. |
| `remove::<T>()` | Drop every global layer of type `T`. |
| `replace::<T, _>(middleware)` | Swap every global layer of type `T`. |
| `alias(name, middleware)` | Register one layer under a route name. |
| `group(name, layers)` | Register an ordered list under a route name. |

Global order is prepends, then the base stack, then appends. This skeleton does not install middleware yet. Add a `with_middleware` call in `bootstrap/app.rs` before `with_routing` when you add one.

There is no `terminate` hook, and aliases do not take parameters.

### Service container

`ruvel::container::Container` is a type-keyed container. Resolving a service does not take a lock.

| Method | Laravel equivalent | Behavior |
| --- | --- | --- |
| `bind` | `$container->bind()` | Store a factory. `make` calls it on every resolve. |
| `singleton` | `$container->singleton()` | Store one instance and hand out clones of the `Arc`. |
| `alias` | `$container->alias()` | Map a string name to a type. |
| `make::<T>()` | `$container->make()` | Resolve `T`. |
| `make_alias::<T>(name)` | resolve by alias | Resolve the type registered under `name`. |

`ServiceProvider` has the same two phases as Laravel:

1. `register` runs when the provider is added and should only bind services.
2. `boot` runs when the application starts listening. It can resolve services and register routes.

Attach one with `with_provider(...).await` on the builder. The call in `bootstrap/app.rs` is commented out, and `app/providers/` is empty. The container is not injected into route closures or controller methods.

### Controllers

`ruvel::http::Controller` is a resource controller. Each action takes a `Request` and returns `impl Responsable`. Actions you do not override answer `501 Not Implemented`.

| Action | Typical route |
| --- | --- |
| `index` | `GET /resources` |
| `create` | `GET /resources/create` |
| `store` | `POST /resources` |
| `show` | `GET /resources/{id}` |
| `edit` | `GET /resources/{id}/edit` |
| `update` | `PUT` or `PATCH /resources/{id}` |
| `destroy` | `DELETE /resources/{id}` |

`WelcomeController` overrides `index` and adds a `welcome` method. `/` calls `welcome`. You register every action on the router yourself. There is no `resource()` helper and no constructor injection.

### Also available from the framework

- **HTTP/1.1 server.** `listen(host, port)` boots providers, then accepts connections. Keep-alive and pipelined requests share a connection. Bodies are framed with `Content-Length`. A request larger than 64 KiB is dropped. A non-identity `Transfer-Encoding` closes the connection.
- **`Responsable`.** Handlers can return `&'static str`, `String`, `Response`, `()`, `(Status, T)`, or `(u16, T)`. Strings become `200 OK` with `Content-Type: text/html; charset=utf-8`.
- **Responses.** `Response::ok`, `created`, `no_content`, `bad_request`, `not_found`, and `not_implemented`, plus `.text()`, `.json()`, `.html()`, and `.header()`.
- **Headers.** Lookup ignores the case of the name.

---

## Still to be done

In this application:

- `app/providers/` is empty, and provider registration in `bootstrap/app.rs`.
- No middleware is installed.
- `routes/api.rs`, `config/`, `database/migrations/`, `database/seeders/`, and `resources/` are empty for the moment.

In the framework these builder methods panic with `todo!`:

- `with_events`
- `with_commands`
- `with_kernels`
- `with_scheduler`

These framework types are placeholders:

- The Eloquent `Model` is an empty struct and returns an empty `200 OK`. There is no query builder, connection, migration runner, or seeder.
- `Config` holds a name and a debug flag. Nothing loads `.env` files or config files.
- The HTTP `Kernel` contract and the container contract trait are empty. The working container is the concrete `Container`.

Not started in the framework:

- Validation and form requests
- Authentication, authorization, sessions, cookies, and CSRF
- Views and templating
- Named routes, route-model binding, and automatic resource registration
- `HEAD`, `OPTIONS`, and custom fallback routes
- Injecting container services into closures or controller methods
- A parsed query map, multipart bodies, and file uploads
- Redirect and cookie helpers
- Chunked request bodies, TLS, and HTTP/2
- An Artisan-style CLI, queues, mail, cache, and notifications

The framework crate documents the same surface in its own README: [Stephan-MC/ruvel](https://github.com/Stephan-MC/ruvel).
