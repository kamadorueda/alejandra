let
  lockData = builtins.readFile ./flake.lock;
  lock = builtins.fromJSON lockData;
  flakeCompat = lock.nodes.flakeCompat.locked;
  flakeCompatSrc =
    builtins.fetchTarball
      {
        url = "https://github.com/edolstra/flake-compat/archive/${ flakeCompat.rev }.tar.gz";
        sha256 = flakeCompat.narHash;
      };
  flake = import flakeCompatSrc { src = ./.; };
in
  flake.defaultNix.defaultPackage
