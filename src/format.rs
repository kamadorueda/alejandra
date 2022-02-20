pub fn string(
    config: &crate::config::Config,
    path: String,
    string: String,
) -> String {
    let tokens = rnix::tokenizer::Tokenizer::new(&string);
    let ast = rnix::parser::parse(tokens);

    for error in ast.errors() {
        eprintln!("Error: {}, at: {}", error, path);
        return ast.node().to_string();
    }

    let green_node =
        crate::builder::build(config, ast.node().into(), false, path).unwrap();

    if config.debug() {
        crate::debug::display(&(&green_node).into());
    }

    green_node.to_string()
}

pub fn file(
    config: &crate::config::Config,
    path: String,
) -> std::io::Result<bool> {
    use std::io::Write;

    let input = std::fs::read_to_string(&path)?;
    let input_clone = input.clone();
    let input_bytes = input_clone.as_bytes();

    let output = crate::format::string(config, path.clone(), input);
    let output_bytes = output.as_bytes();

    let changed = input_bytes != output_bytes;
    if changed {
        std::fs::File::create(path)?.write_all(output_bytes)?;
    }

    Ok(changed)
}
