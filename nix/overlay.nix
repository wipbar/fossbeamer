self: pkgs: rec {
  fossbeamer_crates = import ../Cargo.nix {
    inherit pkgs;
    nixpkgs = pkgs.path;

    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      fossbeamer = prev: {
        # NOTE: this is a result of us adding a dependency to zlib through
        # `system_deps`, to get an impure `cargo build` to pick up zlib runpath.
        # Once rustc stops linking to zlib explicitly, or also does do RUNPATH
        # propagation, this could be removed.
        # See https://github.com/NixOS/nixpkgs/pull/325876
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = prev.buildInputs or [ ] ++ [
          pkgs.zlib
        ];
      };
    };
  };
  fossbeamer = fossbeamer_crates.rootCrate.build.override {
    runTests = true;
  };
}
