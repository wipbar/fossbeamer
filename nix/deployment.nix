let
  inherit (import ./. { }) pkgs pkgsAArch64;

  cfgCommon = {
    # https://github.com/DBCDK/morph/issues/106
    nixpkgs.pkgs = pkgsAArch64;
    nixpkgs.localSystem.system = "aarch64-linux";

    deployment.substituteOnDestination = true;
    deployment.targetUser = "root";
  };

  cfgBar = cfgCommon // {
    imports = [ ./configuration-bar.nix ];
  };
  cfgBornfurs = cfgCommon // {
    imports = [ ./configuration-bornfurs.nix ];
  };
in
{
  network = {
    inherit pkgs;
  };

  "screen1" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n1.screens.wip.bar";
    };
  "screen2" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n2.screens.wip.bar";
    };
  "screen3" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n3.screens.wip.bar";
    };
  "screen4" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n4.screens.wip.bar";
    };
  "screen5" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n5.screens.wip.bar";
    };
  "screen6" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n6.screens.wip.bar";
    };
  "screen7" =
    { config, pkgs, ... }:
    cfgBar
    // {
      deployment.targetHost = "root@n7.screens.wip.bar";
    };

  "bornfurs1" =
    { config, pkgs, ... }:
    cfgBornfurs
    // {
      deployment.targetHost = "root@bornfurs1.screens.wip.bar";
    };
  "bornfurs2" =
    { config, pkgs, ... }:
    cfgBornfurs
    // {
      deployment.targetHost = "root@bornfurs2.screens.wip.bar";
    };
  "bornfurs3" =
    { config, pkgs, ... }:
    cfgBornfurs
    // {
      deployment.targetHost = "root@bornfurs3.screens.wip.bar";
    };
  "bornfurs4" =
    { config, pkgs, ... }:
    cfgBornfurs
    // {
      deployment.targetHost = "root@bornfurs4.screens.wip.bar";
    };
  "flokli" =
    { config, pkgs, ... }:
    {
      # https://github.com/DBCDK/morph/issues/106
      nixpkgs.pkgs = pkgsAArch64;
      nixpkgs.localSystem.system = "aarch64-linux";

      imports = [ ./configuration.nix ];
      deployment.substituteOnDestination = true;
      deployment.targetUser = "root";
      deployment.targetHost = "fossbeamer.flokli.io";
    };
}
