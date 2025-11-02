use crate::contract::{Contract, Flag, Stdin, Stdout};
use crate::method::Method;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

pub trait Handle {
    fn handle(method: &mut Method);
}

pub struct MainHandler;
struct ContractFillerHandler;
struct InterfaceHandler;
struct InterfaceMethodPathReplaceHandler;
struct InterfaceContractValidator;
struct ContractValidator;
struct StdoutHandler;
struct StdinHandler;

impl Handle for MainHandler {
    fn handle(method: &mut Method) {
        ContractFillerHandler::handle(method);
        InterfaceHandler::handle(method);
        ContractValidator::handle(method);
        StdinHandler::handle(method);
        StdoutHandler::handle(method);
    }
}

impl Handle for ContractFillerHandler {
    fn handle(method: &mut Method) {
        if !method.object.path.is_dir() {
            eprintln!("Object must be a directory");

            exit(1);
        }

        method.object.contracts = object_contracts(&method.object.path);
    }
}

impl Handle for InterfaceHandler {
    fn handle(method: &mut Method) {
        if !method.object.name.starts_with("__") || !method.object.name.ends_with("__") {
            return;
        }

        if method.object.contracts.is_none() {
            eprintln!("Interface must have a contract");
            exit(1)
        }

        match fs::read_dir(&method.object.path) {
            Ok(files) => {
                let mut files_count = 0;
                for file in files {
                    match file {
                        Ok(_file) => {
                            files_count += 1;
                        }
                        Err(err) => {
                            eprintln!("IO Error: {:?} : {err}", &method.object.path);
                            exit(1)
                        }
                    }
                }

                if files_count != 2 {
                    eprintln!(
                        "Interfaces must only contain two files: a self file with contracts and a link to an object"
                    );
                    exit(1)
                }
            }
            Err(err) => {
                eprintln!("IO Error: {:?} {err}", &method.object.path);
                exit(1)
            }
        }

        InterfaceMethodPathReplaceHandler::handle(method);
        InterfaceContractValidator::handle(method);
    }
}

impl Handle for InterfaceMethodPathReplaceHandler {
    fn handle(method: &mut Method) {
        let object_link = fs::read_dir(&method.object.path)
            .unwrap()
            .map(|x| x.unwrap())
            .filter(|x| x.file_name().ne(".self"))
            .map(|x| x.path())
            .collect::<Vec<_>>()
            .get(0)
            .map(|x| x.clone())
            .unwrap();

        let object_path = fs::canonicalize(object_link).unwrap();
        method.path = object_path.join(&method.name);
        method.object.path = object_path;
    }
}

impl Handle for InterfaceContractValidator {
    fn handle(method: &mut Method) {
        let interface_contracts = method.object.contracts.as_ref().unwrap();
        let object_contracts = object_contracts(&method.object.path);
        if object_contracts.is_none() {
            eprintln!(
                "The object \"{}\" referenced by the interface must contain a contract.",
                method.object.name
            );
            exit(1)
        }

        for (method, contract) in interface_contracts {
            match object_contracts.as_ref().unwrap().get(method) {
                Some(object_contract) if contract.eq(object_contract) => {}
                _ => {
                    eprintln!(
                        "The enumeration of interface contracts must be a subset of the enumeration of contracts of the object referenced by the interface"
                    );
                    exit(1)
                }
            }
        }

        if !interface_contracts.contains_key(&method.name) {
            if object_contracts.unwrap().contains_key(&method.name) {
                eprintln!(
                    "The called method \"{}\" is not specified in the interface contract. However, it is specified in the objects contract: {:?}",
                    method.name, method.object.path
                );
                exit(1)
            }

            eprintln!(
                "The called method \"{}\" is not specified in the interface contract.",
                method.name
            );
            exit(1)
        }

        method.object.contracts = object_contracts;
    }
}

impl Handle for ContractValidator {
    fn handle(method: &mut Method) {
        if let Some(contracts) = &method.object.contracts {
            if let Some(contract) = contracts.get(&method.name) {
                let contract_flags: HashMap<String, Flag> = contract
                    .flags()
                    .iter()
                    .cloned()
                    .map(|x| (x.name().clone(), x))
                    .collect();

                let mut flags = Vec::new();
                let mut args_count = 0;

                let mut requires_value: (String, bool) = (String::default(), false);
                for arg in method.args.iter().cloned() {
                    if arg.starts_with("-") {
                        if requires_value.1 {
                            break
                        }
                        let flag_has_value: bool;
                        let flag_name: String;

                        if let Some(i) = arg.find("=") {
                            flag_name = arg[..i].to_string();
                            flag_has_value = true;
                        } else {
                            flag_name = arg;
                            flag_has_value = false;
                        }

                        match contract_flags.get(&flag_name) {
                            Some(flag) => {
                                requires_value =
                                    (flag_name.clone(), flag.required_value() && !flag_has_value);
                            }
                            None => {
                                eprintln!(
                                    "A flag was provided that is not in the contract: {flag_name}"
                                );
                                exit(1);
                            }
                        }

                        flags.push(flag_name);
                        continue
                    }

                    if !requires_value.1 {
                        args_count += 1;
                    }
                    requires_value = (String::default(), false);
                }

                if requires_value.1 {
                    eprintln!(
                        "The flag \"{}\" must have a value, which is not provided",
                        requires_value.0
                    );
                    exit(1);
                }

                if contract.required_args().len() > args_count {
                    eprintln!(
                        "The arguments provided are fewer than required by the contract. The contract requires {} arguments and {} optional ones.",
                        contract.required_args().len(),
                        contract.args().len() - contract.required_args().iter().len()
                    );
                    exit(2)
                }

                if contract.args().len() < args_count {
                    eprintln!(
                        "Too many arguments. The contract requires {} arguments and {} optional ones.",
                        contract.required_args().len(),
                        contract.args().len() - contract.required_args().iter().len()
                    );
                    exit(2)
                }

                let contract_flags = contract_flags.values().collect::<Vec<_>>();
                for flag in &contract_flags {
                    if flag.is_required() && !flags.contains(flag.name()) {
                        eprintln!("One required flag is missing: {}", flag.name());
                        exit(2)
                    }
                }

                let contract_flags = contract_flags.iter().map(|x| x.name()).collect::<Vec<_>>();
                for flag in flags {
                    if !contract_flags.contains(&&flag) {
                        eprintln!("The flag {flag} is not mentioned in the contract");
                        exit(2)
                    }
                }
            }
        }
    }
}

impl Handle for StdoutHandler {
    fn handle(method: &mut Method) {
        if let Some(contracts) = &method.object.contracts {
            if let Some(contract) = contracts.get(&method.name) {
                match contract.stdout() {
                    Stdout::None => {
                        if atty::isnt(atty::Stream::Stdout) {
                            eprintln!(
                                "The contract {} does not imply functionality for stdout, but stdout is used in pipeline.",
                                contract.name()
                            );
                            exit(1);
                        }
                    }
                    _ => return,
                }
            }
        }
    }
}

impl Handle for StdinHandler {
    fn handle(method: &mut Method) {
        if let Some(contracts) = &method.object.contracts {
            if let Some(contract) = contracts.get(&method.name) {
                match contract.stdin() {
                    Stdin::Required => {
                        if method.stdin.is_none() {
                            eprintln!(
                                "The contract \"{}\" requires stdin, which is not provided",
                                contract.name()
                            );
                            exit(2);
                        }
                    }
                    Stdin::None => {
                        if method.stdin.is_some() {
                            eprintln!(
                                "The contract \"{}\" does not imply functionality for stdin, but stdin was passed.",
                                contract.name()
                            );
                            exit(2);
                        }
                    }
                    _ => return,
                }
            }
        }
    }
}

fn object_contracts(object_path: &PathBuf) -> Option<HashMap<String, Contract>> {
    match fs::read_to_string(object_path.join(".self")) {
        Ok(string) => Some(
            string
                .lines()
                .map(|x| match Contract::from_str(x) {
                    Ok(contract) => contract,
                    Err(_err) => {
                        eprintln!("Error while parsing contract: {x}");
                        exit(2);
                    }
                })
                .map(|x| (x.name().clone(), x))
                .collect::<HashMap<String, Contract>>(),
        ),
        Err(err) if err.kind() == ErrorKind::NotFound => None,
        Err(err) => {
            eprintln!("IO Error: {:?} : {err}", object_path);
            exit(1);
        }
    }
}
