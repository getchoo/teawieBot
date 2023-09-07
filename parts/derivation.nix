{
  lib,
  stdenv,
  craneLib,
  self,
  ...
}:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource self;
  inherit (self.packages.${stdenv.hostPlatform.system}) cargoArtifacts;

  meta = with lib; {
    mainProgram = "teawiebot";
    description = "funni bot";
    homepage = "https://github.com/getchoo/teawiebot";
    license = licenses.mit;
    platforms = with platforms; unix;
    maintainers = with maintainers; [getchoo];
  };
}
