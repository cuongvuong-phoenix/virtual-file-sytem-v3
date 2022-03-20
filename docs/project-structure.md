# Project Structure

This project consists of 2 parts:

- [*Client*](../client) using [*TypeScript*](https://www.typescriptlang.org/) programming language and [*Vue 3*](https://vuejs.org/) as the front-end web framework.
- [*Server*](../server) using [*Rust*](https://rust-lang.com/) programming language and [*Axum*](https://github.com/tokio-rs/axum) as the back-end web framework.

## Client (TypeScript + Vue 3)

- *Vite + Vue 3 + TypeScript* base structure + simple *SPA* since we only have a single page to render and we also do not need *SEO*:
  - [`/index.html`](../client/index.html) as the entry point, which imports the `main.js` module compiled from [`/src/main.ts`](../client/src/main.ts).
  - [`/public`](../client/public): consists of any asset that will be statically bundled into production and will be served at `/<file>` endpoint.
  - [`/src`](../client/src) as the main source code folder:
    - [`/src/components`](../client/src/components): contains Vue components to be imported.
    - [`/src/composables`](../client/src/composables): contains Vue 3 composable functions.
    - [`/src/constants`](../client/src/constants): contains constants, enums that can be used accross the application.
    - [`/src/helpers`](../client/src/helpers): contains helper functions.
    - [`/src/types`](../client/src/types): contains additional shim, types and models.
- *Core Libraries:*
  - [TailwindCSS](https://tailwindcss.com/) - utility-first CSS framework for rapidly building custom user interfaces.
  - [PostCSS](https://postcss.org/) - a tool for transforming CSS with JavaScript: combined with *TailwindCSS*.
  - [`unplugin-icons`](https://github.com/antfu/unplugin-icons) - access thousands of icons as components on-demand universally.
  - [`unplugin-vue-components`](https://github.com/antfu/unplugin-vue-components) - On-demand components auto importing for Vue.
  - [date-fns](https://date-fns.org/) - modern JavaScript date utility library.
  - [Axios](https://axios-http.com/) - promise-based HTTP client for the browser and Node.JS.
  - [yargs](https://yargs.js.org/) - helps building interactive command line tools, by parsing arguments and generating an elegant user interface.
- *Development Tools:*
  - [ESLint](https://eslint.org/) - find and fix problems in JavaScript code.
  - [Prettier](https://prettier.io/) - an opinionated code formatter.
  - [Volar](https://github.com/johnsoncodehk/volar) - fast Vue language support extension for VSCode

## Server (Rust + Axum)

- Typical *Cargo* binary package structure:
  - [`/src`](../server/src) as the main source code folder:
    - [`/src/main.rs`](../server/src/main.rs): entry point of the binary package.
    - [`/src/config`](../server/src/config): contains shared configurations such as environments.
    - [`/src/api`](../server/src/api): contains all API implementations including handlers, models, errors, responses.
  - [`/migrations`](../server/migrations): contains *SQLx* migration definitions.
  - [`/db`](../server/db): contains development-only SQL scripts for testing in local environment via *DBeaver*.
- *Core Libraries:*
  - [Tokio](https://tokio.rs/) - an asynchronous runtime for the Rust programming language.
  - [Axum](https://github.com/tokio-rs/axum) - a web application framework that focuses on ergonomics and modularity.
  - [Tower](https://github.com/tower-rs/tower) - modular components for building reliable clients and servers, including retry, load-balancing, filtering, request-limiting facilities, and more.
  - [SQLx](https://github.com/launchbadge/sqlx) - an async, pure Rust SQL crate featuring compile-time checked queries without a DSL.
  - [Chrono](https://github.com/chronotope/chrono) - date and time crate for Rust.
  - [Serde](https://serde.rs/) - a framework for serializing and deserializing Rust data structures efficiently and generically.
- *Development Tools:*
  - [Rust Analyzer](https://rust-analyzer.github.io/) - a modular compiler frontend to create excellent IDE support for the Rust language.
  - [DBeaver](https://dbeaver.io/) - free universal database tool.
  - [Postman](https://www.postman.com/) - an API platform for building and using APIs.
