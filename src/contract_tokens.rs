use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f\r]+")]
pub enum ContractTokens {
    #[token("#>>>")]
    Start,

    #[regex(r"stdin[!?]")]
    Stdin,

    #[regex(r"stdout[!?]")]
    Stdout,

    #[regex(r"\w+[?!]")]
    Arg,

    #[regex(r"--?[a-zA-Z]\w*=?[?!]")]
    Flag,

    #[regex(r"\d+")]
    Number,

    #[token("->")]
    Arrow,

    #[token("(")]
    LP,

    #[token(")")]
    RP,

    #[token("[")]
    LSB,

    #[token("]")]
    RSB,

    #[token(",")]
    Comma,

    #[regex(r"\w+:")]
    Name,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokeniser() {
        let mut lex = ContractTokens::lexer("#>>> method_name: stdin! -> (arg?, arg!, aboba!, --flag!, --flag2?) -> stdout?[2, 42, 50]");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Start)));
        assert_eq!(lex.slice(), "#>>>");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Name)));
        assert_eq!(lex.slice(), "method_name:");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Stdin)));
        assert_eq!(lex.slice(), "stdin!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arrow)));
        assert_eq!(lex.slice(), "->");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::LP)));
        assert_eq!(lex.slice(), "(");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "aboba!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag2?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::RP)));
        assert_eq!(lex.slice(), ")");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arrow)));
        assert_eq!(lex.slice(), "->");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Stdout)));
        assert_eq!(lex.slice(), "stdout?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::LSB)));
        assert_eq!(lex.slice(), "[");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "42");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "50");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::RSB)));
        assert_eq!(lex.slice(), "]");

        let mut lex = ContractTokens::lexer("#>>> stdin! arg? arg! aboba! --flag! --flag2? stdout?[2, 42, 50]");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Start)));
        assert_eq!(lex.slice(), "#>>>");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Stdin)));
        assert_eq!(lex.slice(), "stdin!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "aboba!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag2?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Stdout)));
        assert_eq!(lex.slice(), "stdout?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::LSB)));
        assert_eq!(lex.slice(), "[");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "42");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "50");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::RSB)));
        assert_eq!(lex.slice(), "]");

        let mut lex = ContractTokens::lexer("#>>> stdin! -> arg? arg! aboba! --flag! --flag2? -> stdout? 2, 42, 50");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Start)));
        assert_eq!(lex.slice(), "#>>>");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Stdin)));
        assert_eq!(lex.slice(), "stdin!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arrow)));
        assert_eq!(lex.slice(), "->");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "aboba!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag2?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arrow)));
        assert_eq!(lex.slice(), "->");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Stdout)));
        assert_eq!(lex.slice(), "stdout?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "42");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Comma)));
        assert_eq!(lex.slice(), ",");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Number)));
        assert_eq!(lex.slice(), "50");

        let mut lex = ContractTokens::lexer("#>>> arg? arg! aboba! -s! -b?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Start)));
        assert_eq!(lex.slice(), "#>>>");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "arg!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Arg)));
        assert_eq!(lex.slice(), "aboba!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "-s!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "-b?");

        let mut lex = ContractTokens::lexer("-s=! -b=? --flag=! --flag=?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "-s=!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "-b=?");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag=!");

        assert_eq!(lex.next(), Some(Ok(ContractTokens::Flag)));
        assert_eq!(lex.slice(), "--flag=?");
    }
}
