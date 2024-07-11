self: pkgs: rec {
  fossbeamer_crates = import ../Cargo.nix {
    inherit pkgs;
    nixpkgs = pkgs.path;
  };

  fossbeamer = fossbeamer_crates.rootCrate.build.override {
    runTests = true;
  };
}
