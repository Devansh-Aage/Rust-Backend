# 🦀 Backend Rust Pro Roadmap (6 Weeks)

### 🎯 Goal
Be able to **design, build, debug, and deploy scalable production-grade backends** using  
**Rust + Tokio + Axum + SQLx + Tracing.**

Each week includes:

- **Focus:** what you’ll master  
- **Project:** the core thing you’ll build  
- **Challenges:** smaller exercises that push you  
- **Resources:** to dive deeper

---

## 🗓️ Week 1 — Async & Axum Foundations

### 🎯 Focus
Understand async behavior enough to build your first Axum server confidently.

### 🧠 Concepts
- Futures, `.await`, and task scheduling  
- Tokio basics: `spawn`, `join!`, `select!`  
- Channels (`mpsc`, `broadcast`)  
- Axum basics: routes, extractors, JSON responses  
- Shared state with `Arc<RwLock<T>>`

### 💻 Mini Challenges
- Write an async function that runs 3 concurrent tasks with `tokio::join!`.  
- Build a small `mpsc` channel demo: producer + 2 consumers.  
- Use `tokio::select!` to race between a timer and a long task.

### 🧩 Project — Mini “Hello API” Server
- Routes: `/health`, `/greet/:name`, `/time`  
- Global state (request counter)  
- Logging with `tracing`  
- Graceful shutdown  

### 📚 Resources
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum Getting Started](https://docs.rs/axum/latest/axum/)
- [Jon Gjengset – Async in Rust (YouTube)](https://www.youtube.com/watch?v=9_3krAQtD2k)

✅ **By end of Week 1:** You can spin up and reason about an async Axum app with concurrency and logging.

---

## 🗓️ Week 2 — REST APIs & Middleware

### 🎯 Focus
Build full REST APIs, handle errors cleanly, and understand middleware flow.

### 🧠 Concepts
- Axum extractors (`Path`, `Query`, `Json`, `State`)  
- Custom error types and `IntoResponse`  
- Tower layers: request logging, timing  
- JSON request/response validation with `serde`

### 💻 Mini Challenges
- Implement your own logging middleware with `tower::Layer`.  
- Create a unified `AppError` type and map it to `StatusCode`.  
- Add input validation using `serde::Deserialize`.

### 🧩 Project — Task Manager API
- CRUD routes: `/tasks`, `/tasks/:id`  
- Shared `RwLock<HashMap>` store  
- Custom error responses  
- Request timing middleware

### 📚 Resources
- [Axum Error Handling Guide](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)
- [Tower HTTP Middleware Docs](https://docs.rs/tower-http/latest/tower_http/)

✅ **By end of Week 2:** You can design clean, modular REST APIs with proper middleware & error layers.

---

## 🗓️ Week 3 — Databases & SQLx

### 🎯 Focus
Connect your backend to a real database with async persistence.

### 🧠 Concepts
- SQLx: `query!`, `query_as!`, and `Pool`  
- Connection pooling & migrations  
- Mapping `sqlx::Error` to your API errors  
- Transactions & prepared statements

### 💻 Mini Challenges
- Connect to SQLite or Postgres using SQLx.  
- Write migrations for `users` and `tasks` tables.  
- Implement a repository layer to abstract DB calls.

### 🧩 Project — Persistent Task API
- Replace in-memory store with a database  
- Implement CRUD via SQLx  
- Add `migrate!()` macro + connection pool  
- Use `.env` for DB URL

### 📚 Resources
- [SQLx Book](https://docs.rs/sqlx/latest/sqlx/)
- [Axum + SQLx Example](https://github.com/tokio-rs/axum/tree/main/examples)
- [Shuttle.rs Postgres Example](https://www.shuttle.rs/)

✅ **By end of Week 3:** You can build a full async API backed by a database.

---

## 🗓️ Week 4 — Auth, Sessions & Security

### 🎯 Focus
Add authentication, sessions, and security patterns.

### 🧠 Concepts
- JWT authentication (`jsonwebtoken` crate)  
- Password hashing (`argon2` crate)  
- Session state (cookies or tokens)  
- Auth middleware and role-based access

### 💻 Mini Challenges
- Implement password hashing and verify login credentials.  
- Write a middleware that extracts and validates JWT.  
- Protect routes based on user roles (e.g., `/admin`).

### 🧩 Project — Auth-Enabled Notes API
- Routes: `/register`, `/login`, `/notes`  
- JWT-based auth middleware  
- Role-based authorization  
- Store hashed passwords in DB

### 📚 Resources
- [jsonwebtoken crate](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)
- [argon2 crate](https://docs.rs/argon2/latest/argon2/)
- [Axum Auth Example](https://github.com/tokio-rs/axum/blob/main/examples/jwt.rs)

✅ **By end of Week 4:** You can secure your APIs with authentication and session management.

---

## 🗓️ Week 5 — Background Jobs & Concurrency

### 🎯 Focus
Design async systems that run jobs concurrently and cleanly.

### 🧠 Concepts
- Worker pools and message queues (`mpsc`, `broadcast`)  
- Structured concurrency (`JoinSet`, cancellation)  
- Scheduling periodic tasks (`tokio::time::interval`)  
- Graceful shutdown of async workers

### 💻 Mini Challenges
- Create a job producer that spawns worker tasks.  
- Implement periodic cleanup using `tokio::time::interval`.  
- Handle `Ctrl+C` signal to cancel tasks cleanly.

### 🧩 Project — Async Job Queue System
- REST endpoint `/jobs` to enqueue tasks  
- Worker pool that processes jobs asynchronously  
- Job logs + retry logic  
- Graceful shutdown with cancellation signals

### 📚 Resources
- [Structured Concurrency in Tokio](https://tokio.rs/tokio/topics/structured-concurrency)
- [Tokio JoinSet Docs](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)
- [Graceful Shutdown Example](https://tokio.rs/tokio/topics/shutdown)

✅ **By end of Week 5:** You can orchestrate concurrent background work safely.

---

## 🗓️ Week 6 — Observability, Optimization & Deployment

### 🎯 Focus
Bring everything together into a production-ready service.

### 🧠 Concepts
- `tracing` + `tracing-subscriber` for structured logs  
- Request metrics (Prometheus / Tower-Metrics)  
- Performance benchmarking (Criterion)  
- Dockerization & deployment (Fly.io / Shuttle)  
- Config management with `.env`

### 💻 Mini Challenges
- Add structured request logs to your existing API.  
- Expose a `/metrics` endpoint (Prometheus).  
- Run `wrk` or `bombardier` load tests to profile bottlenecks.

### 🧩 Capstone Project — Production-Ready Mini SaaS Backend
- Axum + SQLx + JWT + Job Workers + Tracing  
- Configurable via `.env`  
- Metrics + logs  
- Deploy on Fly.io or Docker

### 📚 Resources
- [tracing crate docs](https://docs.rs/tracing/latest/tracing/)
- [Tokio Performance Tips](https://tokio.rs/tokio/topics/performance)
- [Deploy Rust to Fly.io Guide](https://fly.io/docs/languages-and-frameworks/rust/)

✅ **By end of Week 6:** You’ll have a production-grade backend deployed and observable — your **portfolio-ready project**.

---

### 🏁 Final Outcome
By completing this roadmap, you’ll confidently:

- Build scalable, concurrent backends using **Axum + Tokio**  
- Design modular APIs with strong error handling  
- Manage persistence, auth, background jobs, and observability  
- Optimize and deploy Rust services for real production use
