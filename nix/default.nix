{ system ? builtins.currentSystem
, nixpkgs ? (import ./sources.nix { inherit system; }).nixpkgs
}:

let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ (import ./overlay.nix) ];
  };

  # All libraries we need to link against during an imperative `cargo build`.
  # Used to assemble `PKG_CONFIG_PATH`.
  rust_sys_dep_libs = with pkgs; [
    atk
    gtk3
    gdk-pixbuf
    glib
    webkitgtk_4_1
    libsoup_3
    pango
    cairo
    zlib
  ]
  ++ webkitgtk_4_1.buildInputs
  ++ gdk-pixbuf.buildInputs
  ++ gtk3.buildInputs
  ++ pango.buildInputs;

in
rec {
  inherit pkgs;

  profileEnv = pkgs.writeTextFile {
    name = "profile-env";
    destination = "/.profile";
    # This gets sourced by direnv. Set NIX_PATH, so `nix-shell` uses the same nixpkgs as here.
    text = ''
      export NIX_PATH=nixpkgs=${toString pkgs.path}
      export PKG_CONFIG_PATH=${pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" rust_sys_dep_libs}
    '';
  };

  env = pkgs.buildEnv {
    name = "dev-env";
    paths = [
      profileEnv
    ] ++ (with pkgs;[
      niv
      treefmt

      # Nix
      nixpkgs-fmt

      # Rust
      cargo
      clippy
      rust-analyzer
      rustc
      pkg-config
    ]);
  };
}

