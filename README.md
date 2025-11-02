# 🦀 Async Backend Rust Mastery Plan

This is a **6-week roadmap** designed to take you from knowing only the Rust and Async books → to being **proficient in building scalable, production-grade backend systems in Rust**.

You’ll build projects, solve practical challenges, and gradually learn tools used in real-world production environments like **Tokio, Axum, SQLx, Redis, and tracing**.

---

## 🗓️ Overview

| Week | Focus Area | Mini Project | Core Libraries |
|------|-------------|---------------|----------------|
| 1 | Async Core Foundations | Async Playground (toy server) | Tokio, async/await |
| 2 | Networking Fundamentals | Echo Server + Chat Server | Tokio, TCP/UDP, Futures |
| 3 | Web Framework Mastery | REST API with Axum | Axum, Tower, Serde, SQLx |
| 4 | Database + Auth Systems | Auth + CRUD App | SQLx, JWT, bcrypt, Axum extractors |
| 5 | Distributed Systems | Job Queue + Caching Layer | Redis, Tokio tasks, Channels |
| 6 | Observability & Production | Full Microservice | Tracing, Metrics, Docker, CI/CD |

---

## 🧭 Week 1 — Async Foundations & Tokio Basics

**Goal:** Understand async/await deeply and how the executor works.

### Learn
- How async functions work under the hood (state machines)
- Difference between blocking vs non-blocking
- How `Tokio` runs tasks and spawns them
- Futures, `.await`, `Pin`, and lifetimes in async
- Async channels (`mpsc`, `broadcast`, `oneshot`)

### Project
🧩 **Async Playground**
- Build a CLI app that:
  - Runs async tasks concurrently (like downloading multiple files or simulating delayed operations)
  - Uses `join!`, `select!`, and `tokio::spawn`

### Challenges
- Write your own `MiniExecutor` (custom runtime)
- Implement a fake async "timer" using `tokio::time::sleep`
- Benchmark sequential vs concurrent execution

---

## ⚙️ Week 2 — Networking & Concurrency Models

**Goal:** Learn how to handle low-level async IO and concurrency safely.

### Learn
- TCP/UDP sockets with Tokio
- Streams & sinks (`AsyncRead`, `AsyncWrite`)
- Task cancellation and graceful shutdown
- Handling shared state across async tasks (Arc + Mutex)
- Error propagation and `Result` handling in async contexts

### Project
💬 **Async Chat Server**
- Multi-client chat app using TCP sockets
- Broadcast messages to all connected clients
- Use `tokio::sync::broadcast` or `mpsc`

### Challenges
- Add message history persistence
- Implement server shutdown with Ctrl+C signal
- Add authentication to chat server

---

## 🕸️ Week 3 — Building REST APIs with Axum

**Goal:** Become fluent with **Axum**, the async web framework.

### Learn
- Routing, extractors, middleware, response types
- JSON (serde), form, query parameters
- Layers (Tower) and error handling
- Using shared app state
- Structuring a multi-module backend project

### Project
🧱 **Book Library API**
- CRUD endpoints for books (`GET /books`, `POST /books`)
- In-memory storage (HashMap)
- Proper error handling + middlewares (logging, CORS)

### Challenges
- Add pagination and filtering
- Add simple rate limiting middleware
- Handle graceful shutdown with tracing logs

---

## 🧩 Week 4 — Database Integration + Authentication

**Goal:** Build production-style APIs with database + JWT auth.

### Learn
- SQLx for async DB access (Postgres or SQLite)
- Migrations and connection pooling
- Password hashing (`bcrypt`)
- JWT authentication and middleware
- Error handling patterns (`thiserror`, `anyhow`)

### Project
🔐 **Auth + CRUD App**
- Users can sign up, log in, and manage their profiles
- Protected routes with JWT
- SQLx + Postgres setup

### Challenges
- Implement refresh tokens
- Add async transactional operations
- Integrate Docker for DB + app

---

## ⚡ Week 5 — Distributed Systems Concepts

**Goal:** Build async services that communicate & scale.

### Learn
- Async job queues with Redis
- Task scheduling and background workers
- Using `tokio::sync::mpsc` for message passing
- Handling backpressure and retries

### Project
🚀 **Job Queue + Caching Service**
- REST API that enqueues jobs (e.g. email sending simulation)
- Worker service consumes jobs and processes them
- Add caching layer (Redis)

### Challenges
- Add exponential backoff retries
- Add metrics dashboard (processed jobs count)
- Introduce inter-service communication (HTTP or channel)

---

## 🧠 Week 6 — Observability, Testing & Production

**Goal:** Make your backend production-grade.

### Learn
- Tracing and structured logs (`tracing`, `tracing-subscriber`)
- Metrics and Prometheus integration
- Integration + load testing (`reqwest`, `tokio::test`)
- Dockerizing async apps
- CI/CD basics (GitHub Actions)

### Project
🌐 **Final Microservice**
- Combine everything:
  - Auth, REST API, background workers, caching
  - Observability with tracing + metrics
  - Docker-compose for DB + Redis + app

### Challenges
- Add distributed tracing (Jaeger or OpenTelemetry)
- Deploy to fly.io or Railway
- Write load tests with `wrk` or `k6`

---

## 🏁 After Week 6 — Mastery Path

Once you finish, go deeper into:
- **Advanced async patterns:** `Pin`, `Future`, custom executors
- **Performance tuning:** `tracing-flame`, `criterion`
- **Microservice architecture:** gRPC with `tonic`
- **Real production stacks:** Axum + SQLx + Redis + Kafka + OpenTelemetry

---

## 📚 Recommended References

- [Tokio Docs](https://tokio.rs/tokio/tutorial)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Axum Guide](https://docs.rs/axum/latest/axum/)
- [SQLx Docs](https://docs.rs/sqlx/)
- [Zero To Production in Rust (Book)](https://www.zero2prod.com/)
