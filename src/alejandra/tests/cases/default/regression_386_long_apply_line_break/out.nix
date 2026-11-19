{
  buildersBySubsystem.${cfg.subsystem}.default =
    import (../../../../.. + "/src/subsystems/${cfg.subsystem}/builders/${cfg.builder}/default.nix") {
      inherit lib pkgs utils;
      dlib = {};
    };
  oneLine = import (../../../../.. + "/src/subsystems/${cfg.subsystem}/builders/${cfg.builder}/default.nix") {
    inherit lib pkgs utils;
    dlib = {};
  };
  short = import ./x {
    inherit lib;
  };
  a = f "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" {
    inherit lib;
  };
  b =
    f "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" {
      inherit lib;
    };
  nested = {
    deeper = {
      c = f "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" {
        inherit lib;
      };
      d =
        f "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" {
          inherit lib;
        };
    };
  };
  multi = f {
    inherit lib;
  };
}
