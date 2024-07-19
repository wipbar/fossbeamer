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
      GST_PLUGIN_SYSTEM_PATH_1_0 = lib.makeSearchPathOutput "lib" "lib/gstreamer-1.0" (with pkgs.gst_all_1;[
        gstreamer
        gst-plugins-base
        gst-plugins-good
        gst-plugins-bad
        gst-libav
      ]);
      GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";
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
