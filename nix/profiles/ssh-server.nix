{ ... }:
{

  # Configure ssh
  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    openFirewall = true;
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPTVTXOutUZZjXLB0lUSgeKcSY/8mxKkC0ingGK1whD2 flokli"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIE6a15p9HLSrawsMTd2UQGAiM7r7VdyrfSRyzwRYTgWT flokli@m2air"
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC02nCZ1XKOr7lalsEP4Cy5pfNE34RUOpdvS+UB4ozdpGTk+KybGhh1STs9HcJAmNcOitJ4JRlPtQTQVQK5A8XDnt9qxF5W3umIAkAibe9UIRMJK36/K0BXTtvErLmTUxMuSx/NxRJyTV4/3eJC+Lfzg7hy/M6HoZ41w/7L7WZKotElYgnCZcXMxBPVCX1HXGakcr+SHVZGDAgvQ+QL2kYhRyhAqKWp6U3GJ402selzE1OHOeBylqZP8kRUyt+MjehhLG6Lc3uuc5wiz9UxNc5x7JJXGI2ynta10OyHFOzgKtUzqedyb/5sz53nwdEe4gj/Uyrl84QCpYScLqGPGUOMwHTptBcocucvweOgCFXGwaCid6eAo5FhRLS6JOiCoh0jVvgvFjB+hPAX4cKprlXmpNsJcmn6PvCnIa0IylyT8Ki932KqtSzyEvp+U9Olci+fF0+EjghelSFbY6cEBNdKi/ZYiACtyOnA8hL4oApe7em4nFngkQPPbAh2DpUFxp8= Freya"
  ];

  programs.mosh.enable = true;

  # Allow logging in via serial without password.
  users.users."root".initialPassword = "";
}
