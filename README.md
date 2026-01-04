# Envoy Config

Envoy Config is a small Rust backend service that models how **feature flags and runtime configuration** are managed across different environments (dev, staging, prod) in real-world systems.

The project focuses on **backend design, clean domain logic, and Rust fundamentals**, with HTTP treated as a thin adapter layer rather than the core of the system.

---

## Why this project

In production systems, teams often need to:

* Enable or disable features without redeploying
* Control feature rollout per environment
* Quickly turn off features if something breaks

Envoy Config models this backend responsibility in a minimal and explicit way, similar to internal configuration or feature-flag services used in real companies.

---

## What the service does

* Create or update feature flags scoped to an environment
* Fetch all feature flags for a given environment
* Enforce simple domain rules (for example valid keys and uniqueness per environment)

The service intentionally avoids persistence and authentication to keep the focus on **correctness, clarity, and architecture**.

---

## Architecture

The project is structured to keep business logic independent of transport and storage:

```
src/
├── domain/        # Core domain models and domain errors
├── repository/    # Repository abstraction and in-memory implementation
├── service/       # Business logic and validation
├── http/          # Thin HTTP adapter (Axum)
└── main.rs        # Application wiring
```

### Design principles

* Domain logic does not depend on HTTP
* HTTP handlers contain no business logic
* Storage is in-memory for simplicity
* Clear separation of concerns between layers

---

## API Endpoints

### Create or update a feature flag

```
POST /flags
```

Request body:

```json
{
  "key": "new_checkout",
  "enabled": true,
  "environment": "prod"
}
```

cURL example:

```bash
curl -X POST http://localhost:3000/flags \
  -H "Content-Type: application/json" \
  -d '{"key":"new_checkout","enabled":true,"environment":"prod"}'
```

---

### Fetch flags for an environment

```
GET /flags/{environment}
```

Example:

```
GET /flags/prod
```

cURL example:

```bash
curl http://localhost:3000/flags/prod
```

---

## Running the project

### Prerequisites

* Rust ([https://rustup.rs](https://rustup.rs))
* On Windows: Visual Studio C++ Build Tools

### Run locally

```bash
cargo run
```

The server will start on:

```
http://localhost:3000
```

---

## Notes

* The architecture allows persistence, caching, or authentication to be added later without changing core domain logic.

---

## Motivation

Envoy Config was built to reflect how real backend systems are shaped:

* small, focused services
* explicit domain rules
* transport treated as an adapter

The emphasis is on **engineering judgment and learning**, rather than framework usage.
