{ ... }: {

  # Configure ssh
  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    openFirewall = true;
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPTVTXOutUZZjXLB0lUSgeKcSY/8mxKkC0ingGK1whD2 flokli"
    "sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIA34k0FVKDGNdJ8uk0Ytbvh6J8v+H86F4t6BXAIoW/7xAAAABHNzaDo= flokli 20240704 14321691"
    "sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIP7rdJ1klzK8nx74QQA8jYdFwznM1klLS0C7M5lHiu+IAAAABHNzaDo= flokli 20240617 28772765"
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC02nCZ1XKOr7lalsEP4Cy5pfNE34RUOpdvS+UB4ozdpGTk+KybGhh1STs9HcJAmNcOitJ4JRlPtQTQVQK5A8XDnt9qxF5W3umIAkAibe9UIRMJK36/K0BXTtvErLmTUxMuSx/NxRJyTV4/3eJC+Lfzg7hy/M6HoZ41w/7L7WZKotElYgnCZcXMxBPVCX1HXGakcr+SHVZGDAgvQ+QL2kYhRyhAqKWp6U3GJ402selzE1OHOeBylqZP8kRUyt+MjehhLG6Lc3uuc5wiz9UxNc5x7JJXGI2ynta10OyHFOzgKtUzqedyb/5sz53nwdEe4gj/Uyrl84QCpYScLqGPGUOMwHTptBcocucvweOgCFXGwaCid6eAo5FhRLS6JOiCoh0jVvgvFjB+hPAX4cKprlXmpNsJcmn6PvCnIa0IylyT8Ki932KqtSzyEvp+U9Olci+fF0+EjghelSFbY6cEBNdKi/ZYiACtyOnA8hL4oApe7em4nFngkQPPbAh2DpUFxp8= Freya"
  ];

  programs.mosh.enable = true;

  # Allow logging in via serial without password.
  users.users."root".initialPassword = "";
}
