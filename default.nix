let
  lockData = builtins.readFile ./flake.lock;
  lock = builtins.fromJSON lockData;
  flakeCompat = lock.nodes.flakeCompat.locked;
  flakeCompatSrc = builtins.fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/${flakeCompat.rev}.tar.gz";
    sha256 = flakeCompat.narHash;
  };
  flake = import flakeCompatSrc {src = ./.;};
in
  {system ? builtins.currentSystem, ...}:
    if builtins.hasAttr system flake.defaultNix.defaultPackage
    then flake.defaultNix.defaultPackage.${system}
    else
      builtins.throw ''

        Alejandra does not support the system: ${system}

        Please consider creating an issue requesting
        support for such system:
        https://github.com/kamadorueda/alejandra

        Thank you!

      ''
