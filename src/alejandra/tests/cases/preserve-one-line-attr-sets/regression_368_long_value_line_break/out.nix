{
  # #368: a long string keeps the line break after `=`
  xdgUserDirsProjects =
    "${config.home.homeDirectory}/${config.lib.local.xdgUserDirsProjectsRel}";
  # A short string is pulled up
  short = "abc";
  # A long string on one line stays on one line
  oneline = "${config.home.homeDirectory}/${config.lib.local.xdgUserDirsProjectsRel}";
  # #271: a long function keeps the line break after `=`
  foo =
    f: barbarbarbarbarbarbarbarbarbarbarbarbarbarbar == packageNamecdsacascdacd;
  # A short function is pulled up
  bar = f: f == 1;
  # A function on more than one line is pulled up
  baz = x: {
    a = 1;
  };
  # The indentation counts: 81 characters break, 80 do not
  nested = {
    deeper = {
      value =
        "${config.home.homeDirectory}/${config.lib.local.xdgUserDirsProj}";
      fits = "${config.home.homeDirectory}/${config.lib.local.xdgUserDirsProj}";
    };
  };
  # An indented string on one line
  script =
    ''echo "${config.home.homeDirectory}/${config.lib.local.xdgUserDirsProjects}"'';
}
