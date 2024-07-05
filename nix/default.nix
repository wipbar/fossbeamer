{ system ? builtins.currentSystem
, nixpkgs ? (import ./sources.nix { inherit system; }).nixpkgs
}:

let
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ (import ./overlay.nix) ];
  };
in
rec {
  inherit pkgs;

  profileEnv = pkgs.writeTextFile {
    name = "profile-env";
    destination = "/.profile";
    # This gets sourced by direnv. Set NIX_PATH, so `nix-shell` uses the same nixpkgs as here.
    text = ''
      export NIX_PATH=nixpkgs=${toString pkgs.path}
    '';
  };

  env = pkgs.buildEnv {
    name = "dev-env";
    paths = [
      profileEnv

      pkgs.niv
    ];
  };
}

