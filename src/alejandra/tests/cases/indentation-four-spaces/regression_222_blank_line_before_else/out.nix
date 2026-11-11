{
    # #222: an empty line before `else` stays
    a =
        if some condition
        then do something

        else if some other condition
        then do something_else

        else if
            some more
            && complex condition
        then do something_different

        else if another condition
        then
            do something
            more
            complex

        else something_else;
    # Many empty lines become one
    b =
        if x
        then y

        else z;
    # With an inline comment
    c =
        if x
        then y # why

        else z;
    # With a comment before `else`
    d =
        if x
        then y

        # other case
        else z;
    # No empty line: as before
    e =
        if x
        then y
        else z;
    f =
        if x
        then y
        else z;
}
