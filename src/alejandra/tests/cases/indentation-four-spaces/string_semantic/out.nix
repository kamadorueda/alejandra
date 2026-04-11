{
    # Issue #409: Trailing whitespace in multiline strings
    # These lines have actual trailing spaces that must be preserved
    trailing_spaces = ''
        line without trailing
        line with two spaces  
        another line
    '';

    # Issue #442: String escape sequences must be preserved
    # Escapes like ''${...} prevent interpolation and should not be modified
    escapes = ''
        ''${1+x}
        ''${variable}
        ''${foo.bar}
        '''
        ''''
    '';

    # Indentation significance
    # When indentation is part of the content (like Makefiles)
    makefile = ''
        target: deps
        	echo "built"
    '';

    # Mixed: escapes with interpolation and trailing spaces
    complex = ''
        prefix ''${expr} suffix  
        ''${func "arg"}
        escaped: '''and then '''${real_interp}
    '';
}
