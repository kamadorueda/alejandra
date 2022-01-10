pub fn string(
    config: &crate::config::Config,
    path: &str,
    string: String,
) -> String {
    let tokens = rnix::tokenizer::Tokenizer::new(&string);
    let ast = rnix::parser::parse(tokens);

    for error in ast.errors() {
        eprintln!("Warning: parsing error: {}, at: {}", error, path);
    }

    let green_node =
        crate::builder::build(config, ast.node().into(), false, path).unwrap();

    if config.debug() {
        crate::debug::display(&(&green_node).into());
    }

    green_node.to_string()
}

pub fn file(config: &crate::config::Config, path: &str) -> std::io::Result<()> {
    use std::io::Write;

    let input = std::fs::read_to_string(path)?;
    let output = crate::format::string(config, path, input);

    std::fs::File::create(path)?.write_all(output.as_bytes())?;

    Ok(())
}
