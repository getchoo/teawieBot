_: {
  imports = [
    ./deployment.nix
    ./dev.nix
    ./packages.nix
    ./toolchain.nix
  ];

  systems = [
    "x86_64-linux"
    "x86_64-darwin"
    "aarch64-linux"
    "aarch64-darwin"
  ];

  perSystem = _: {
    _module.args.src = builtins.path {
      name = "teawiebot-src";
      path = ../.;
    };
  };
}
