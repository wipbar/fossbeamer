# Machine config for the Bornfurs displays
{
  ...
}:

{
  imports = [
    ./configuration.nix
    ./profiles/cm3.nix
  ];

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPTVTXOutUZZjXLB0lUSgeKcSY/8mxKkC0ingGK1whD2 flokli"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIE6a15p9HLSrawsMTd2UQGAiM7r7VdyrfSRyzwRYTgWT flokli@m2air"
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCeWntgv5K7p1eZ3wiSBXJO3ENStnnaWVJh3pA8asZWFcP65VXZ+6/6/lRJtI5TtG1OGCN2kvy/p8YXnqzOEHns37MIIlbr3gMy7+AWeGeO5ywep09vL7/FEbUa50gP1z/EJGPenG3kU1MFVV3+1aS2d5taO2GCsOcgKJMMAy0fwvXwLkXlsotckMMrPflD6V4DViXKhMz4J9+O497PPEEq810Y6hAB3O6IGt/FSyNqUvlfNp91I+lNdiFWVqBjX0p2saMUEFKOKQXqv6GeFTIrdVMsoEgwWzI6T7r/CtIwdOysV5cd8Yw2fePWnhLFfsI/r8YvrAPg4fJnejYwdWqlmDrq9DplXPVA/dXYjt4RUhMzzxmYR+TXQJUn7Q3CZkWYLkpkxol4Q5Ds7BgssAn0saFdTVm/cOAr7hUxWsC90dw6ApJR2Fh6VOfWtRa2u1D3QuFXEoDHgYeTID4SWG61gWy8+ZDMye1fmkC5tQXoxSQuRZM9w79T7zHor5PBpGU= richard@richard-ThinkPad-T480s"
  ];
}
