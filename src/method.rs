use crate::contract::Contract;
use regex::Regex;
use std::collections::HashMap;
use std::io::{IsTerminal, Stdin};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};

pub(crate) struct Object {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) contracts: Option<HashMap<String, Contract>>,
}

pub(crate) struct Method {
    pub(crate) stdin: Option<Stdin>,
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) args: Vec<String>,
    pub(crate) object: Object,
}

impl Method {
    pub fn new(
        name: String,
        args: Vec<String>,
        stdin: Stdin,
        namespace: PathBuf,
    ) -> Result<Method, String> {
        let name = name.trim();

        if !Regex::new(r"^[\w-]+\.([\w-]+\.?)+[\w-]$").unwrap().is_match(name) {
            return Err("The method call does not match the pattern".into());
        }

        let stdin_result: Option<Stdin>;
        if stdin.is_terminal() {
            stdin_result = None;
        } else {
            stdin_result = Some(stdin);
        }

        let entities = name.split(".").collect::<Vec<&str>>();

        let object = Object {
            path: namespace.join(entities[..=entities.len() - 2].join("/")),
            name: entities[entities.len() - 2].into(),
            contracts: None,
        };

        Ok(Method {
            stdin: stdin_result,
            name: entities[entities.len() - 1].into(),
            path: namespace.join(entities.join("/")),
            args,
            object,
        })
    }

    pub fn execute(self) {
        let err = Command::new(self.path)
            .args(self.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .exec();

        eprintln!("Exec failed: {err}");
        exit(1);
    }
}
