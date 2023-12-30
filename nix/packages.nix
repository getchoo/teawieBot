{
  self,
  inputs,
  ...
}: {
  perSystem = {
    pkgs,
    system,
    self',
    ...
  }: {
    packages = {
      teawiebot = pkgs.callPackage ./derivation.nix {
        inherit self;
        inherit
          (pkgs.darwin.apple_sdk.frameworks)
          CoreFoundation
          Security
          SystemConfiguration
          ;

        naersk = inputs.naersk.lib.${system};
      };

      default = self'.packages.teawiebot;
    };
  };
}
