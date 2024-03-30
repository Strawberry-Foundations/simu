use crate::parser::DockerfileConfig;
use std::error::Error;

mod parser;
mod colors;
mod logger;

fn main() -> Result<(), Box<dyn Error>> {
    let receipt = DockerfileConfig::parse_yaml("base.yml")?;
    receipt.pre_hooks().unwrap();
    let dockerfile_content = receipt.generate_dockerfile();
    println!("{}", dockerfile_content);
    Ok(())
}
