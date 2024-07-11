# Fossbeamer

WIP

This provides an application, as well as the tooling around a single full-screen
application running on displays at [wip.bar](https://wip.bar/).

## Development setup
[Nix] is used to pin dependencies, and [Direnv] to enter a development
environment automatically, from which you can use `cargo` to build the project,
taking care of all necessary system dependencies.

[Install Nix](https://nixos.org/download/) and
[hook direnv into your shell][hook-direnv] to get started.

## Nix+Rust
For "release builds" we use `crate2nix` to build Rust crates incrementally and
in isolation.

You can build fossbeamer for your current system using `nix-build nix -A
fossbeamer`, and then invoke it via `result/bin/fossbeamer`.

Whenever there's a change in the crate dependencies, run
`crate2nix generate --all-features` to re-generate `Cargo.nix`.


--
[wip.bar]: https://wip.bar
[Nix]: https://nixos.org
[Direnv]: https://direnv.net/docs/hook.html
[hook-direnv]: https://direnv.net/docs/hook.html
