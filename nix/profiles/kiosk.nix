{ lib, pkgs, ... }: {
  # Create a kiosk user
  users.users.kiosk = {
    isNormalUser = true;
    uid = 1000;
    packages = [ pkgs.dconf ];
  };

  # And configure cage to show fossbeamer on it.
  services.cage = {
    enable = true;
    user = "kiosk";
    program = pkgs.writers.writeBash "run-cage-program" ''
      ${pkgs.wlr-randr}/bin/wlr-randr --output HDMI-A-1 --transform 180 --scale 2
      ${pkgs.fossbeamer}/bin/fossbeamer --default-config=${../../default-config.json} https://wip.bar
    '';
    environment = {
      LIBGL_DEBUG = "verbose";
      RUST_LOG = "debug";
    };
    extraArguments = [
      "-d" # don't draw client decorations when possible
    ];
  };
  systemd.services."cage-tty1".restartIfChanged = lib.mkForce true;

  fonts.enableDefaultPackages = true;
  fonts.packages = with pkgs; [
    noto-fonts-color-emoji
    roboto
    source-code-pro
    gentium
  ];

  environment.systemPackages = [
    pkgs.htop
  ];

  documentation.enable = false; # Reduces closure size and build time.
}
