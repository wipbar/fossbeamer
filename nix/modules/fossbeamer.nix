{
  lib,
  pkgs,
  config,
  ...
}:

let
  cfg = config.fossbeamer;
in
{
  options.fossbeamer = {
    enable = lib.mkEnableOption "fossbeamer";
    configFile = lib.mkOption {
      type = lib.types.path;
      default = ../../default-config.json;
      description = "The config file to use.";
    };
    preStart = lib.mkOption {
      type = lib.types.lines;
      default = "";
      description = "Shell commands executed before `fossbeamer` is started.";
    };
  };

  config = lib.mkIf cfg.enable {
    # Create a kiosk user
    users.users.fossbeamer = {
      isNormalUser = true;
      uid = 1000;
    };

    # And configure cage to show fossbeamer on it.
    services.cage = {
      enable = true;
      user = "fossbeamer";
      program = pkgs.writers.writeBash "run-cage-program" ''
        ${cfg.preStart}
        ${pkgs.fossbeamer}/bin/fossbeamer --default-config=${cfg.configFile} https://wip.bar
      '';
      environment = {
        GDK_GL = "gles";
        GDK_DEBUG = "vulkan,opengl,dmabuf";
        LIBGL_DEBUG = "verbose";
        RUST_LOG = "debug";
      };
      extraArguments = [
        "-d" # don't draw client decorations when possible
      ];
    };
    systemd.services."cage-tty1" = {
      restartIfChanged = lib.mkForce true;
      reloadIfChanged = lib.mkForce false;
      serviceConfig.Restart = "always";
    };

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
  };
}
