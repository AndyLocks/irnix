use crate::contract_tokens::ContractTokens;
use logos::{Logos, Source};
use std::collections::HashSet;
use std::process::exit;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Contract {
    stdin: Stdin,
    args: Vec<Arg>,
    flags: HashSet<Flag>,
    stdout: Stdout,
    error_codes: Vec<u32>,
    name: String,
}

impl Contract {
    #[warn(dead_code)]
    pub fn new(
        stdin: impl Into<Stdin>,
        args: Vec<Arg>,
        flags: HashSet<Flag>,
        stdout: impl Into<Stdout>,
        error_codes: Vec<u32>,
        name: String,
    ) -> Contract {
        Contract {
            stdin: stdin.into(),
            args,
            flags,
            stdout: stdout.into(),
            error_codes,
            name,
        }
    }

    pub fn required_args(&self) -> Vec<Arg> {
        self.args.iter().cloned().filter(|x| x.required).collect()
    }

    pub fn args(&self) -> Vec<Arg> {
        self.args.clone()
    }

    pub fn flags(&self) -> HashSet<Flag> {
        self.flags.clone()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn stdin(&self) -> &Stdin {
        &self.stdin
    }

    pub fn stdout(&self) -> &Stdout {
        &self.stdout
    }
}

impl FromStr for Contract {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lex = ContractTokens::lexer(s);

        let mut stdin = Stdin::None;
        let mut stdout = Stdout::None;
        let mut args: Vec<Arg> = Vec::new();
        let mut flags: HashSet<Flag> = HashSet::new();
        let mut error_codes: Vec<u32> = Vec::new();
        let mut name: Option<String> = None;

        while let Some(token) = lex.next() {
            match token? {
                ContractTokens::Stdin => stdin = Stdin::from(lex.slice()),
                ContractTokens::Stdout => stdout = Stdout::from(lex.slice()),
                ContractTokens::Arg => args.push(Arg::from(lex.slice())),
                ContractTokens::Flag => {
                    flags.insert(Flag::from(lex.slice()));
                }
                ContractTokens::Number => {
                    error_codes.push(lex.slice().parse::<u32>().map_err(|_| ())?)
                }
                ContractTokens::Name => name = Some(lex.slice().replace(":", "")),
                _ => continue,
            }
        }

        if name.is_none() {
            eprintln!(
                "Contract must have a name that matches a method name. No name was provided."
            );
            exit(2);
        }

        Ok(Contract {
            stdin,
            args,
            flags,
            stdout,
            error_codes,
            name: name.unwrap(),
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Arg {
    required: bool,
}

impl From<String> for Arg {
    fn from(value: String) -> Self {
        if value.ends_with("!") {
            return Arg { required: true };
        }

        if value.ends_with("?") {
            return Arg { required: false };
        }

        eprintln!("The argument must be explicitly defined as required or optional using ! or ?");
        exit(2)
    }
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        if value.ends_with("!") {
            return Arg { required: true };
        }

        if value.ends_with("?") {
            return Arg { required: false };
        }

        eprintln!("The argument must be explicitly defined as required or optional using ! or ?");
        exit(2)
    }
}

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
pub struct Flag {
    name: String,
    required: bool,
    required_value: bool,
}

impl PartialEq<String> for Flag {
    fn eq(&self, other: &String) -> bool {
        self.name.eq(other)
    }

    fn ne(&self, other: &String) -> bool {
        self.name.ne(other)
    }
}

impl Flag {
    pub fn is_required(&self) -> bool {
        self.required
    }

    pub fn required_value(&self) -> bool {
        self.required_value
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl From<String> for Flag {
    fn from(value: String) -> Self {
        let required: bool;
        let required_value = value.contains("=");

        if value.ends_with("!") {
            required = true;
        } else if value.ends_with("?") {
            required = false;
        } else {
            println!("The flag must be explicitly defined as required or optional using ! or ?");
            exit(2)
        }

        let name: String = if required_value {
            value[..value.len() - 2].to_string()
        } else {
            value[..value.len() - 1].to_string()
        };

        Flag {
            name,
            required,
            required_value,
        }
    }
}

impl From<&str> for Flag {
    fn from(value: &str) -> Self {
        let required: bool;
        let required_value = value.contains("=");

        if value.ends_with("!") {
            required = true;
        } else if value.ends_with("?") {
            required = false;
        } else {
            println!("The flag must be explicitly defined as required or optional using ! or ?");
            exit(2)
        }

        let name: String = if required_value {
            value[..value.len() - 2].to_string()
        } else {
            value[..value.len() - 1].to_string()
        };

        Flag {
            name,
            required,
            required_value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Stdin {
    Required,
    Optional,
    None,
}

impl From<String> for Stdin {
    fn from(value: String) -> Self {
        if value == "stdin!" {
            return Stdin::Required;
        }

        if value == "stdin?" {
            return Stdin::Optional;
        }

        Stdin::None
    }
}

impl From<&str> for Stdin {
    fn from(value: &str) -> Self {
        if value == "stdin!" {
            return Stdin::Required;
        }

        if value == "stdin?" {
            return Stdin::Optional;
        }

        Stdin::None
    }
}

impl From<Option<String>> for Stdin {
    fn from(value: Option<String>) -> Self {
        value.unwrap_or_default().into()
    }
}

impl From<Option<&str>> for Stdin {
    fn from(value: Option<&str>) -> Self {
        value.unwrap_or_default().into()
    }
}

#[derive(PartialEq, Debug)]
pub enum Stdout {
    Required,
    Optional,
    None,
}

impl From<String> for Stdout {
    fn from(value: String) -> Self {
        if value == "stdout!" {
            return Stdout::Required;
        }

        if value == "stdout?" {
            return Stdout::Optional;
        }

        Stdout::None
    }
}

impl From<&str> for Stdout {
    fn from(value: &str) -> Self {
        if value == "stdout!" {
            return Stdout::Required;
        }

        if value == "stdout?" {
            return Stdout::Optional;
        }

        Stdout::None
    }
}

impl From<Option<String>> for Stdout {
    fn from(value: Option<String>) -> Self {
        value.unwrap_or_default().into()
    }
}

impl From<Option<&str>> for Stdout {
    fn from(value: Option<&str>) -> Self {
        value.unwrap_or_default().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contract_eq() {
        let contract = Contract::new(
            "stdin!",
            vec![],
            HashSet::default(),
            "stdout?",
            vec![],
            String::default(),
        );
        let contract2 = Contract::new(
            "stdin!",
            vec![],
            HashSet::default(),
            "stdout?",
            vec![],
            String::default(),
        );

        assert_eq!(contract, contract2);

        let contract = Contract::new(
            "stdin?",
            vec![Arg::from("arg!")],
            HashSet::from([Flag::from("--flag?")]),
            "stdout!",
            vec![2, 3, 8],
            "aboba".into(),
        );

        let contract2 = Contract::new(
            "stdin?",
            vec![Arg::from("arg!")],
            HashSet::from([Flag::from("--flag?")]),
            "stdout!",
            vec![2, 3, 8],
            "aboba".into(),
        );

        assert_eq!(contract, contract2);

        let contract = Contract::new(
            "stdin?".to_string(),
            vec![Arg::from("arg!")],
            HashSet::from([Flag::from("--flag?")]),
            "stdout?",
            vec![2, 3, 8],
            "aga".into(),
        );

        let contract2 = Contract::new(
            "stdin!".to_string(),
            vec![Arg::from("arg!")],
            HashSet::from([Flag::from("--flag?")]),
            "stdout!",
            vec![2, 3, 8],
            "aboba".into(),
        );

        assert_ne!(contract, contract2);
    }

    #[test]
    fn contract_parse() {
        assert_eq!(
            "#>>> aboba: stdin! arg? arg! aboba! --flag! --flag2? stdout?[2, 42, 50]"
                .parse::<Contract>()
                .unwrap(),
            Contract {
                stdin: Stdin::Required,
                args: vec![Arg::from("arg?"), Arg::from("arg!"), Arg::from("aboba!")],
                flags: HashSet::from([Flag::from("--flag!"), Flag::from("--flag2?")]),
                stdout: Stdout::Optional,
                error_codes: vec![2, 42, 50],
                name: "aboba".into()
            }
        );

        assert_eq!(
            "name: stdin? arg! aboba! --flag! --flag2? stdout! 2, 42, 50"
                .parse::<Contract>()
                .unwrap(),
            Contract {
                stdin: Stdin::Optional,
                args: vec![Arg::from("arg!"), Arg::from("aboba!")],
                flags: HashSet::from([Flag::from("--flag!"), Flag::from("--flag2?")]),
                stdout: Stdout::Required,
                error_codes: vec![2, 42, 50],
                name: "name".into()
            }
        );

        assert_eq!(
            "#>>> aga: stdin! -> (arg?, arg!, --flag!, --flag2!) -> stdout?[2, 42, 50]"
                .parse::<Contract>()
                .unwrap(),
            Contract {
                stdin: Stdin::Required,
                args: vec![Arg::from("arg?"), Arg::from("arg!")],
                flags: HashSet::from([Flag::from("--flag!"), Flag::from("--flag2!")]),
                stdout: Stdout::Optional,
                error_codes: vec![2, 42, 50],
                name: "aga".into(),
            }
        );

        assert_eq!(
            "contract: arg? arg! --flag! --flag2?"
                .parse::<Contract>()
                .unwrap(),
            Contract {
                stdin: Stdin::None,
                args: vec![Arg::from("arg?"), Arg::from("arg!")],
                flags: HashSet::from([Flag::from("--flag!"), Flag::from("--flag2?")]),
                stdout: Stdout::None,
                error_codes: Vec::new(),
                name: "contract".into()
            }
        );
    }

    #[test]
    fn arg() {
        assert!(Arg::from("aboba!").required);
        assert!(!Arg::from("aboba?").required);
        assert!(Arg::from("aboba!".to_string()).required);
        assert!(!Arg::from("aboba?".to_string()).required);
    }

    #[test]
    fn flag() {
        let flag = Flag::from("--aboba!");

        assert!(flag.required);
        assert_eq!("--aboba", flag.name);

        let flag = Flag::from("--aboba?");

        assert!(!flag.required);
        assert_eq!("--aboba", flag.name);

        let flag = Flag::from("--aboba=!".to_string());

        assert!(flag.required);
        assert_eq!("--aboba", flag.name);

        let flag = Flag::from("-a=?".to_string());

        assert!(!flag.required);
        assert_eq!("-a", flag.name);

        let flag = Flag::from("-a!");

        assert!(flag.required);
        assert_eq!("-a", flag.name);

        let flag = Flag::from("-a?");

        assert!(!flag.required);
        assert_eq!("-a", flag.name);

        let flag = Flag::from("-a=!".to_string());

        assert!(flag.required);
        assert_eq!("-a", flag.name);

        let flag = Flag::from("-a=?".to_string());

        assert!(!flag.required);
        assert_eq!("-a", flag.name);
    }

    #[test]
    #[should_panic]
    fn flag_from_str() {
        let _ = Flag::from("aboba");
    }

    #[test]
    #[should_panic]
    fn flag_from_string() {
        let _ = Flag::from("aboba".to_string());
    }

    #[test]
    fn stdin() {
        assert_eq!(Stdin::from("stdin!"), Stdin::Required);
        assert_eq!(Stdin::from("stdin?"), Stdin::Optional);
        assert_eq!(Stdin::from(""), Stdin::None);
        assert_eq!(Stdin::from(String::default()), Stdin::None);

        assert_eq!(Stdin::from("stdin!".to_string()), Stdin::Required);
        assert_eq!(Stdin::from("stdin?".to_string()), Stdin::Optional);

        assert_eq!(Stdin::from(Some("stdin!")), Stdin::Required);
        assert_eq!(Stdin::from(Some("stdin?")), Stdin::Optional);
        assert_eq!(Stdin::from(Some("stdin!".to_string())), Stdin::Required);
        assert_eq!(Stdin::from(Some("stdin?".to_string())), Stdin::Optional);
        assert_eq!(Stdin::from(Some("")), Stdin::None);
        assert_eq!(Stdin::from(Some(String::default())), Stdin::None);
    }

    #[test]
    fn stdout() {
        assert_eq!(Stdout::from("stdout!"), Stdout::Required);
        assert_eq!(Stdout::from("stdout?"), Stdout::Optional);

        assert_eq!(Stdout::from(""), Stdout::None);
        assert_eq!(Stdout::from(String::default()), Stdout::None);

        assert_eq!(Stdout::from("stdout!".to_string()), Stdout::Required);
        assert_eq!(Stdout::from("stdout?".to_string()), Stdout::Optional);

        assert_eq!(Stdout::from(Some("stdout!")), Stdout::Required);
        assert_eq!(Stdout::from(Some("stdout?")), Stdout::Optional);
        assert_eq!(Stdout::from(Some("stdout!".to_string())), Stdout::Required);
        assert_eq!(Stdout::from(Some("stdout?".to_string())), Stdout::Optional);
        assert_eq!(Stdout::from(Some("")), Stdout::None);
        assert_eq!(Stdout::from(Some(String::default())), Stdout::None);
    }
}
