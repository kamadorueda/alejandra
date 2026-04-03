const n=`{ buildDunePackage
, gluten
, lwt
}:

buildDunePackage rec {
  pname = "gluten-lwt";
  inherit (gluten) doCheck meta src useDune2 version;

  propagatedBuildInputs = [
    gluten
    lwt
  ];
}
`;export{n as default};
//# sourceMappingURL=nixpkgs-1c105fd4-9ca68bnx.js.map
