# The intention of this project

nunomaduro.com is the personal website of Nuno Maduro: the posts, the talks, the newsletter and the sponsorships. A Rust server on `hyper` renders the pages from `askama` templates and the posts from Markdown files. It is an application, not a package.

Install the dependencies with `npm ci`. Build the CSS and the JavaScript with `npm run build`. Run the tests with `cargo test`. Run the linter with `cargo clippy`. Start the server with `cargo run`, or with `make dev` to restart it on each change. The server listens on the port in `PORT`, or on port `8000`.

- `src/` — the binary `server`, which binds the port and serves each request
- `application/` — the crate of the HTTP router and the routes
- `domain/` — the crate of the value objects and the contracts of the repositories
- `infrastructure/` — the crate of the repositories: the Markdown posts, the static pages and the Mailcoach newsletter
- `presentation/` — the crate of the `askama` templates, with the CSS and the JavaScript that Vite builds
- `content/` — the posts, in Markdown
- `public/` — the static files, the images and the build output of Vite in `public/dist/`
- `scripts/` — the Node scripts that generate the Open Graph images and update the social numbers
