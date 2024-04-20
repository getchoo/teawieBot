{
  description = "teawie moment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
  in {
    nixosModules.default = import ./nix/module.nix self;

    packages = forAllSystems ({
      pkgs,
      system,
      ...
    }: {
      teawiebot = pkgs.callPackage ./nix/derivation.nix {inherit self;};
      default = self.packages.${system}.teawiebot;
    });

    overlays.default = _: prev: {
      teawiebot = prev.callPackage ./nix/derivation.nix {inherit self;};
    };
  };
}
