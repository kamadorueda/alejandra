{
    a = pkgs.writeShellScript "a" /* sh */ ''
        echo ${b}
    '';
    c = {
        d = f /* lua */ "x";
        e = [(f /* sh */ "y")];
    };
    g =
        f
        /* not a string */
        x;
}
