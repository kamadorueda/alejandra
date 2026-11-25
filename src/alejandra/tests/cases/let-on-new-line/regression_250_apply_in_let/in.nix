let
  r = lib.mapAttrs (appName: app:
    { type = "app"; program = app.program; }
  ) apps;
in
r
