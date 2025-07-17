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


## Functionality
Fossbeamer is a fullscreen Rust application, using webkitgtk to render a website.
It can be remote-controlled via MQTT.

It picks the broker specified in the config, and a topic using a (hopefully
unique) serial number. In the case of the CM3 displays, this is the serial
present in the EDID data (so surviving re-flashes).

In other cases, it might be the machine-id.

On bootup, publishes a message to `screens/$id/info`, containing data about
the connected display, as well as the IP addresses present on the network
interface(s).

It also listens to `screens/$id/set`, to allow being configured to show another
URL.
Check `src/display/mod.rs`, `enum Scenario` for the exact definition, but the
following message payload configures it to show the bornhack website:

```
{"name":"url", "args": { "url": "https://bornhack.dk"}}
```

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

### Deployment
Initially, you can flash the resulting SD card image using `dd` or similar
tools. Depending on the flash chip and image size, it can take 10-15mins.

Subsequent redeploys can be done via
[morph](https://github.com/DBCDK/morph), if you have ssh access.
They only copy the paths that are changed (so it's much faster), but might be
not possible if the disk is too small. If that's the case, you can run
`nix-collect-garbage -d` and try again, or resort to flashing the SD image.

#### Initial flashing to CM3
The displays with the integrated CM3 modules can be flashed without opening the
case, performing the following steps:

 - Unplug the AC cable from the monitor
 - Unplug ALL USB peripherals from the USB hub
 - Connect the USB-B side of a USB-A to USB-B cable to the monitor
 - Connect the USB-A side to your computer
 - Plug the AC cable into the monitor
 - Run `sudo rpiboot` on your computer, wait for the block device to appear.
   You can run `nix-shell -p rpiboot` to get a functioning version.
   Once it's mounted, take note of the name, e.g. `/dev/sda` by looking at the
   `dmesg -w` output.
 - Ensure the block devices are unmounted if they got automounted.
   Use `sudo umount /dev/sda*` or similar.
 - Flash the image.
   Assuming you used above `nix-build` command,
   it sits in a `result/sd-image-nixos-image-*.img.zst` path (zstd-compressed).
   You can use the following command (please double-check the output block device):
   `zstd -d < result/sd-image-nixos-image-*.img.zst | sudo dd of=/dev/sda bs=4M status=progress`
   `dd` calls `sync()`, so it might sit around seemingly doing nothing at the
   end. Wait for the command to fully return!
 - Disconnect the USB cable
 - Disconnect the AC cable from the monitor. Wait at least 5 seconds for
   everything to power down.
 - Reconnect the USB peripherals and AC power.
 - The monitor should show U-Boot, a kernel log and eventually boot into
   fossbeamer.

#### Incremental updates via morph
There is a `nix/deployment.nix` file keeping track of all deployments.

You can use the `morph deploy --on=$pattern nix/deployment.nix switch` command to
deploy the latest version to that host. Check the `morph` documentation for
other options.

As explained above, this might work for small updates only, and major updates
might need a reflash.

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
