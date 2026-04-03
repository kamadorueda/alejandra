const r=`{ callPackage, fetchurl, ... } @ args:

callPackage ./generic.nix (args // rec {
  version = "1.60.0";

  src = fetchurl {
    url = "mirror://sourceforge/boost/boost_\${builtins.replaceStrings ["."] ["_"] version}.tar.bz2";
    sha256 = "0fzx6dwqbrkd4bcd8pjv0fpapwmrxxwr8yx9g67lihlsk3zzysk8";
  };

})
`;export{r as default};
//# sourceMappingURL=nixpkgs-963a5d20-Bu18p7PL.js.map
