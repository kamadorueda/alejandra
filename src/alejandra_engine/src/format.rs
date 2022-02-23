pub fn string(path: String, string: String) -> std::io::Result<String> {
    let tokens = rnix::tokenizer::Tokenizer::new(&string);
    let ast = rnix::parser::parse(tokens);

    let errors = ast.errors();
    if !errors.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            errors[0].to_string(),
        ));
    }

    let green_node =
        crate::builder::build(ast.node().into(), false, path, true).unwrap();

    Ok(green_node.to_string())
}

pub fn string_or_passthrough(path: String, before: String) -> String {
    match crate::format::string(path, before.clone()) {
        Ok(after) => after,
        Err(_) => before,
    }
}

pub fn file(path: String) -> std::io::Result<bool> {
    use std::io::Write;

    let input = std::fs::read_to_string(&path)?;
    let input_clone = input.clone();
    let input_bytes = input_clone.as_bytes();

    let output = crate::format::string(path.clone(), input)?;
    let output_bytes = output.as_bytes();

    let changed = input_bytes != output_bytes;
    if changed {
        std::fs::File::create(path)?.write_all(output_bytes)?;
    }

    Ok(changed)
}
