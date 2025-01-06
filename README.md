# Route-rs

Route-rs is multiple things:
- Http webserver.
- Http path router.
- Http html renderer which consists of multiple parts:
  - css-in-rust functionality. with deduplication (almost like tailwind but css-in-rust).
  - Some magic with links (a tags) with the help of javascript.

All these things are wrapped in one final package, which becomes a web framework. You can mix and match which libraries you would like to use. Everything is composable.

All these libraries have as little libraries as possible.
