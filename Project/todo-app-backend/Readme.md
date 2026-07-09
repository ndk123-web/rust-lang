# Axum Backend Notes

## Routing

### `nest()`

* Similar to Express `app.use("/prefix", router)`
* Used to group related routes under a common URL prefix.

Example:

```text
/api/v1/auth/login
/api/v1/auth/signup
```

---

### `layer()`

* Attaches middleware to a `Router`.
* Can be applied:

  * Globally (entire application)
  * To a specific router (only a route group)

> The **last** `.layer(...)` added is the **first** middleware executed because each layer wraps the previous router.

---

### `from_fn()`

Used for creating **custom middleware** from an async function.

It converts a normal async function into a Tower middleware.

Example middleware signature:

```rust
async fn middleware(
    request: Request,
    next: Next,
) -> Response
```

Usually:

1. Read/modify the request.
2. Optionally stop the request early.
3. Otherwise call:

```rust
next.run(request).await
```

---

# Request Flow

```text
Client

↓

Router

↓

Middleware(s)

↓

Handler

↓

Service

↓

Repository

↓

Database
```

---

# Responsibilities

## Handler

Responsible for HTTP.

* Receives request data.
* Uses Axum extractors (`Json`, `Path`, `Query`, `State`, etc.).
* Calls the appropriate service.
* Returns an HTTP response.

Should **not** contain business logic or SQL.

---

## Service

Responsible for business logic.

Examples:

* Validate business rules.
* Generate JWT.
* Hash passwords.
* Check permissions.
* Call repositories.

Should **not** know anything about Axum or HTTP.

---

## Repository

Responsible only for database operations.

Examples:

* SELECT
* INSERT
* UPDATE
* DELETE

Should contain SQLx/database code only.

---

# Environment Variables

We use:

```text
dotenvy
```

Load:

```rust
dotenvy::dotenv()
```

Read values:

```rust
std::env::var("DATABASE_URL")
```

These values are typically loaded once into a `Config` struct.

---

# Config vs AppState

## Config

Contains application configuration loaded from `.env`.

Example:

```rust
Config {
    database_url,
    port,
    jwt_secret,
}
```

Its job is **only** to represent configuration.

---

## AppState

Contains shared application resources used across requests.

Examples:

```rust
AppState {
    config,
    pool,
}
```

Later it may also contain:

* Redis client
* SMTP client
* Cache
* Logger
* etc.

AppState is injected into the router using:

```rust
.with_state(state)
```

---

# Router Generic

Definition:

```rust
Router<S = ()>
```

Default:

```rust
Router<()>
```

means **no application state**.

After:

```rust
.with_state(app_state)
```

the router becomes:

```rust
Router<AppState>
```

All nested routers must use the same state type.

---

# Axum Extractors

Examples:

```rust
State<T>
Json<T>
Path<T>
Query<T>
HeaderMap
CookieJar
```

## When can extractors be used?

Only when **Axum is invoking your function**.

Examples:

* Route handlers ✅
* Some Axum-managed callbacks/extractors ✅
* Axum reads the handler signature first, then it takes the needed data from the HTTP request and fills the parameters.

Not inside:

* Services ❌
* Repositories ❌
* Regular helper functions ❌

Reason:

Axum is responsible for constructing and injecting extractors. If you call a function yourself, you should pass normal Rust values (`&AppState`, `&SqlitePool`, DTOs, etc.) instead.

---

# Important Mental Models

* **Handler = HTTP**
* **Service = Business Logic**
* **Repository = Database**
* **Config = Application Configuration**
* **AppState = Shared Application Resources**
* **Extractors = Data provided by Axum**
* **Middleware = Code executed before and/or after handlers**

## Migration

`sqlx-cli` is used locally to manage database migrations.

Simple flow:

1. Create a new migration file:

    ```text
    sqlx migrate add something
    ```

2. Run migrations:

    ```text
    sqlx migrate run
    ```

It reads `DATABASE_URL` from `.env` and runs the migration on that database.

## Json Response

To send JSON from Axum, use a tuple like this:

```rust
(status_code, Json(value)).into_response()
```

This is useful when you want to return both HTTP status and JSON data.

## Argon and password_hash mechanism

Argon2 hashes the password using a random salt and hashing parameters.

Simple idea:

1. User gives a plain password.
2. Argon2 creates a hash using salt and parameters.
3. Store only the hash in the database.
4. When the user logs in, compare the entered password with the stored hash.
5. If the verification succeeds, the password is correct.

This is safer than storing plain text passwords.
