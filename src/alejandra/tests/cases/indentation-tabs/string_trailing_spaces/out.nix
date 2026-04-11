# Test case for issue #409: trailing whitespace preservation
# Content lines have actual trailing spaces that must NOT be stripped
{
	# Case 1: trailing spaces on content lines
	example = ''
    no trailing
    has two trailing spaces  
    another with spaces  
  '';

	# Case 2: Makefile-like content where trailing spaces might be significant
	makefile_like = ''
    target: deps
    	echo "hello"  
  '';

	# Case 3: mixed trailing patterns
	mixed = ''
    clean line
    trailing here  
    another clean
    ends with space 
  '';
}
