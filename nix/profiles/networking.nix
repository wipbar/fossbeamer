{ ... }:
{
  # Configure networking
  networking.hostName = "";
  networking.useDHCP = true;
  networking.useNetworkd = true;
  networking.wireless.iwd.enable = true;
  networking.networkmanager.enable = false;
  services.resolved.enable = true;
  services.resolved.dnssec = "false";
  networking.nftables.enable = true;
}
