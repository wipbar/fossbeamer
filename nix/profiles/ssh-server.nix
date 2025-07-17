{ ... }:
{

  # Configure ssh
  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    openFirewall = true;
  };

  programs.mosh.enable = true;

  # Allow logging in via serial without password.
  users.users."root".initialPassword = "";
}
