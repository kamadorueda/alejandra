const a=`{deployAndroidPackage, lib, package, os, autoPatchelfHook, pkgs}:

deployAndroidPackage {
  inherit package os;
  nativeBuildInputs = [ autoPatchelfHook ];
  buildInputs = lib.optional (os == "linux") [ pkgs.stdenv.glibc pkgs.stdenv.cc.cc pkgs.ncurses5 ];
  patchInstructions = lib.optionalString (os == "linux") ''
    autoPatchelf $packageBaseDir/bin
  '';
}
`;export{a as default};
//# sourceMappingURL=nixpkgs-71e2b7c0-DnY2o6Xx.js.map
