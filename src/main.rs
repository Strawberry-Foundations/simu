use crate::parser::DockerfileConfig;
use std::error::Error;

mod parser;
mod colors;
mod logger;

fn main() -> Result<(), Box<dyn Error>> {
    let receipt = DockerfileConfig::parse_yaml("base.yml")?;
    match receipt.pre_hooks() { 
        Ok(..) => (), 
        Err(err) => eprintln!("Error")
    };
    let dockerfile_content = receipt.generate_dockerfile();
    println!("{}", dockerfile_content);
    
    std::fs::write("Dockerfile", dockerfile_content).unwrap();
    Ok(())
}
