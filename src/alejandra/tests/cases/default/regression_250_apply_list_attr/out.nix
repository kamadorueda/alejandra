{
  list = [(f (x: {a = x;})) (g (y: [y]))];
  with_ = with pkgs; mapAttrs (n: v: {inherit n v;}) set;
}
