const e=`{ lib, stdenv, runtimeShell }:

stdenv.mkDerivation {
  pname = "example-unfree-package";
  version = "1.0";

  dontUnpack = true;

  installPhase = ''
    mkdir -p $out/bin
    cat > $out/bin/hello-unfree << EOF
    #!\${runtimeShell}
    echo "Hello, you are running an unfree system!"
    EOF
    chmod +x $out/bin/hello-unfree
  '';

  meta = {
    description = "An example package with unfree license (for testing)";
    license = lib.licenses.unfree;
    maintainers = [ lib.maintainers.oxij ];
  };
}
`;export{e as default};
//# sourceMappingURL=nixpkgs-f09a5c95-D9kL2RO3.js.map
