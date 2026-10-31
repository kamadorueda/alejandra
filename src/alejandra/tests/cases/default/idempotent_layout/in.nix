rec {
  flakeApps =
    lib.mapAttrs (appName: app:
      {
        type = "app";
        program = b.toString app.program;
      }
    ) apps;

  lambdaInElse =
    if a then 1
    else
      content:
      let
        l = stringLength content;
      in
      l;
}
