# Fossbeamer

WIP

This provides an application, as well as the tooling around a single full-screen
application running on displays at [wip.bar](https://wip.bar/), a Bar opened
during [Bornhack](https://bornhack.dk).

## Development setup
[Nix] is used to pin dependencies, and [Direnv] to enter a development
environment automatically, from which you can use `cargo` to build the project,
taking care of all necessary system dependencies.

[Install Nix](https://nixos.org/download/) and
[hook direnv into your shell][hook-direnv] to get started.

## Nix+Rust
For "release builds" we use `crate2nix` to build Rust crates incrementally and
in isolation.

You can build fossbeamer for your current system using
`nix-build nix -A fossbeamer`, and then invoke it via `result/bin/fossbeamer`.

Whenever there's a change in the crate dependencies, run
`crate2nix generate --all-features; treefmt` to re-generate `Cargo.nix`.

### Machine configuration
Various NixOS machine configs are provided in `nix/configuration.nix`.
It describes running fossbeamer in a wayland compositor, cage.

We have multiple flavours, depending on where they're used:

 - `machine-bar`
 - `machine-bornfurs`

The system toplevel can be reached via:
`nix-build nix -A machine-$flavour.toplevel`

An SD-card image can be reached via:
`nix-build nix -A machine-$flavour.sdImage`

If you invoke the build from `x86_64-linux`, it'll cross-compile.
If you build on an `aarch64-linux` box, it'll natively compile. Both should
work.

### VM Test
For a smaller feedback loop, it's possible to build a script running the
software stack in a local qemu (KVM). It might have another virtual screen
resolution, and obviously doesn't test the hardware bits.

To run it, invoke:
```
$(nix-build nix -A vm)/bin/run-fossbeamer-vm
```

--
[wip.bar]: https://wip.bar
[Nix]: https://nixos.org
[Direnv]: https://direnv.net/docs/hook.html
[hook-direnv]: https://direnv.net/docs/hook.html
