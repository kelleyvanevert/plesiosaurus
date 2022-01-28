# Experiments in building an API with Rust

Resources used:

- Started with [_Zero To Production In Rust_](https://www.zero2prod.com/index.html?country=Netherlands&discount_code=VAT20)

  - Web server: [actix-web](https://actix.rs/) ([docs.rs](https://docs.rs/actix-web/latest/actix_web/))

  - Typed plain SQL: [sqlx](https://github.com/launchbadge/sqlx) ([docs.rs](https://docs.rs/sqlx/latest/sqlx/))

- Added GraphQL, see also [this blog post](https://romankudryashov.com/blog/2020/12/graphql-rust/)

  - GraphQL server library: [async-graphql](https://github.com/async-graphql/async-graphql) ([docs.rs](https://docs.rs/async-graphql/latest/async_graphql/))

    - [minimal example](https://github.com/danbruder/async-graphql-sqlx-example)

    - [Object macro attributes](https://docs.rs/async-graphql/latest/async_graphql/attr.Object.html) _(Not super sure how to find these in the docs w/ the searchbar...)_
