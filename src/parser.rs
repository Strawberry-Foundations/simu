use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use crate::{command, error, exception, info, success};


#[derive(Debug, Serialize, Deserialize)]
pub struct DockerfileConfig {
    name: String,
    id: String,
    #[serde(rename = "pre-init")]
    pre_init: Option<InitConfig>,
    init: InitConfig,
    #[serde(rename = "post-init")]
    post_init: Option<InitConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitConfig {
    base: Option<String>,
    files: Option<Vec<FileConfig>>,
    commands: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    source: String,
    destination: String,
}

impl DockerfileConfig {
    pub fn parse_yaml(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let receipt: DockerfileConfig = serde_yaml::from_reader(reader).unwrap_or_else(|err| {
            exception!(err);
        });
        Ok(receipt)
    }

    pub fn pre_hooks(&self) -> eyre::Result<()> {
        info!("Starting pre-init build process ...");
        if let Some(pre_init) = &self.pre_init {
            if let Some(commands) = &pre_init.commands {
                for command in commands {
                    command!(format!("Executing command {command}"));
                    let status = Command::new("sh").arg("-c").arg(&command).status()?;

                    if status.success() {
                        success!(format!("Command '{command}' executed successfully"));
                    } else {
                        error!(format!("Error executing command: {command}"));
                        exception!("Cannot continue due to previous error");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn generate_dockerfile(&self) -> String {
        let mut dockerfile = String::new();
        /* if let Some(pre_init) = &self.pre_init {
            if let Some(commands) = &pre_init.commands {
                for command in commands {
                    dockerfile.push_str(&format!("RUN {}\n", command));
                }
            }
        } */
        if let Some(init) = &self.init.base {
            dockerfile.push_str(&format!("FROM {}\n", init));
        }
        if let Some(files) = &self.init.files {
            for file in files {
                dockerfile.push_str(&format!("ADD {} {}\n", file.source, file.destination));
            }
        }
        if let Some(init) = &self.init.commands {
            for command in init {
                dockerfile.push_str(&format!("RUN {}\n", command));
            }
        }
        if let Some(post_init) = &self.post_init {
            if let Some(commands) = &post_init.commands {
                for command in commands {
                    dockerfile.push_str(&format!("RUN {}\n", command));
                }
            }
        }

        success!("Created Dockerfile");
        dockerfile
    }
}