{
  a = [
    ''(
            "${chromium}/Chromium.app/Contents/MacOS/Chromium"
            )''
  ];
  b = ''[name] The niv type "builtin-url" will soon be deprecated.
        $ niv modify name -a type=file -a builtin=true'';
  c = ''${x}
      y
    '';
  d = ''one line'';
  e = ''
    kept as before
  '';
}
