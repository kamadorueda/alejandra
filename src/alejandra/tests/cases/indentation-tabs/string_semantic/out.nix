{
	# Issue #409: Trailing whitespace in multiline strings
	# These lines have actual trailing spaces that must be preserved
	trailing_spaces = ''
    line without trailing
    line with two spaces  
    another line
  '';

	# Issue #442: String escape preservation
	# These escape sequences must not be modified
	escapes = ''
    ''${variable}
    '''
    ''''
  '';

	# Indentation significance
	# When indentation is part of the content (like Makefiles)
	makefile = ''
    target: deps
    	echo "built"
  '';

	# Mixed: escapes with interpolation
	complex = ''
    prefix ''${expr} suffix
    ''${func "arg"}
  '';
}
