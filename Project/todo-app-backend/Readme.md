### Backend Axum

1. `nest` -> like express prefix for urls
2. `layer` -> Use for Router, and can be apply globally or for specific router
3. `from_fn` -> Use for Custom Middleware
    - takes `function` that accpet `request` and `next` as parameter and return `response`

#### Flow

`User -> Router -> Middleware -> Handler -> Service -> Repository`
