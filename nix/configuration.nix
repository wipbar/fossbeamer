# Machine config
{ config
, modulesPath
, pkgs
, lib
, ...
}: {
  imports = [
    (modulesPath + "/installer/sd-card/sd-image-aarch64.nix")
    ./profiles/kiosk.nix
    ./profiles/networking.nix
    ./profiles/ssh-server.nix
  ];
  # HACK: sd-image-aarch64.nix sets console=ttyS0,115200n8 (and some others),
  # which breaks the device-tree provided serial config.
  boot.kernelParams = lib.mkForce [
    "systemd.log_target=console"
    "systemd.journald.forward_to_console=1"
    "cma=256MB"
  ];

  # configure journald to not logs to disk
  services.journald.extraConfig = ''
    Storage=volatile
  '';

  boot.kernelPackages = pkgs.linuxPackages_latest;
  # zfs keeps breaking, removing all unneeded filesystems from the image.
  boot.supportedFilesystems =
    lib.mkForce
      [ "vfat" ];


  # Switch to systemd-initrd, which is the future (tm).
  # Also it has proven to be a bit more permissive with some boards where the
  # sdcard takes ages to spin up.
  boot.initrd.systemd.enable = true;

  nix.settings.max-jobs = 8;

  environment.systemPackages = [
    pkgs.kmsxx # kmsprint
    pkgs.i2c-tools
  ];

  system.stateVersion = "24.05";
}
