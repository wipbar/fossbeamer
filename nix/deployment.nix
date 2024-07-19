let
  nixpkgs = (import ./sources.nix { inherit system; }).nixpkgs;
  system = builtins.currentSystem;
  pkgs = import nixpkgs {
    inherit system;
    overlays = [ (import ./overlay.nix) ];
  };
  pkgsAArch64 = import nixpkgs {
    localSystem = system;
    crossSystem = "aarch64-linux";
    overlays = [ (import ./overlay.nix) ];
  };

  common = {
    # https://github.com/DBCDK/morph/issues/106
    nixpkgs.pkgs = pkgsAArch64;
    nixpkgs.localSystem.system = "aarch64-linux";

    imports = [ ./configuration-cm3.nix ];
    deployment.substituteOnDestination = true;
    deployment.targetUser = "root";
  };
in
{
  network = {
    inherit pkgs;
  };

  "screen1" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n1.screens.wip.bar";
  };
  "screen2" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n2.screens.wip.bar";
  };
  "screen3" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n3.screens.wip.bar";
  };
  "screen4" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n4.screens.wip.bar";
  };
  "screen5" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n5.screens.wip.bar";
  };
  "screen6" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n6.screens.wip.bar";
  };
  "screen7" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@n7.screens.wip.bar";
  };

  "bornfurs1" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@bornfurs1.screens.wip.bar";
  };
  "bornfurs2" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@bornfurs2.screens.wip.bar";
  };
  "bornfurs3" = { config, pkgs, ... }: common // {
    deployment.targetHost = "root@bornfurs3.screens.wip.bar";
  };
}
