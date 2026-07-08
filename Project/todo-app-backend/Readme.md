### Backend Axum

1. `nest` -> like express prefix for urls
2. `layer` -> Use for Router, and can be apply globally or for specific router
3. `from_fn` -> Use for Custom Middleware
    - takes `function` that accpet `request` and `next` as parameter and return `response`

#### Flow

- `User -> Router -> Middleware -> Handler -> Service -> Repository`

- Handler -> Takes Arguments/Data from request and send it to related service
- Service -> Actual Buissness Logic
- Repository -> Only DB Related work

#### Enviroment

- we used `dotenvy` crate
- and we get using `std::env::var(key)` the value of enviromenet variable
