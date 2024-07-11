fn main() {
    // HACK: steer the impure nix build to zlib via pkg-config,
    // but don't do that for the pure one.
    // NOTE: this is a result of us adding a dependency to zlib through
    // `system_deps`, to get an impure `cargo build` to pick up zlib runpath.
    // Once rustc stops linking to zlib explicitly, or also does do RUNPATH
    // propagation, this could be removed.
    // See https://github.com/NixOS/nixpkgs/pull/325876
    for (k, v) in std::env::vars() {
        if &k == "NIX_BUILD_TOP" && v == "/build" {
            return;
        }
    }

    system_deps::Config::new().probe().unwrap();
}
