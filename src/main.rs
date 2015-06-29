#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[derive(Debug, Eq, PartialEq)]
pub enum AST {
    Integer(isize),
    Symbol(String),
    Nil,
    Cons(Box<AST>, Box<AST>),
}

peg! parse(r#"
#[pub]
read -> super::AST
    = cons / integer / nil / symbol

cons -> super::AST
    = "(" space car:read space "." space cdr:read space ")"
    { super::AST::Cons(Box::new(car),
      Box::new(cdr)) }

nil -> super::AST
    = "()" { super::AST::Nil }

integer -> super::AST
    = sign:"-"? n:num
    { super::AST::Integer(if sign.is_some() { - n } else { n }) }

symbol -> super::AST
    = [a-zA-Z][a-zA-Z0-9\-]* { super::AST::Symbol(match_str.to_string()) }

num -> isize
    = [0-9]+ { match_str.parse().unwrap() }

space -> ()
    = [ \n\t]*

"#);

#[cfg(test)]
mod test {
    use super::parse;
    use AST::*;

    #[test]
    fn parse_integers() {
        assert_eq!(parse::read("123"), Ok(Integer(123)));
        assert_eq!(parse::read("-123"), Ok(Integer(-123)));
        assert_eq!(parse::read("0"), Ok(Integer(0)));
        assert_eq!(parse::read("-0"), Ok(Integer(0)));
    }

    #[test]
    fn parse_cons() {
        assert_eq!(parse::read("(123 . 234)"), Ok(Cons(Box::new(Integer(123)),
                                                      Box::new(Integer(234)))));
        assert_eq!(parse::read("(123 . (234 . 42))"),
                   Ok(Cons(Box::new(Integer(123)),
                           Box::new(Cons(Box::new(Integer(234)),
                                         Box::new(Integer(42)))))));
    }

    #[test]
    fn parse_nil() {
        assert_eq!(parse::read("()"), Ok(Nil));
        assert_eq!(parse::read("(1 . ())"), Ok(Cons(Box::new(Integer(1)),
                                                  Box::new(Nil))));
    }

    #[test]
    fn parse_symbol() {
        assert_eq!(parse::read("nil"), Ok(Symbol("nil".to_string())));
        assert_eq!(parse::read("x"), Ok(Symbol("x".to_string())));
        assert_eq!(parse::read("make-something"),
                   Ok(Symbol("make-something".to_string())));
        assert_eq!(parse::read("cdr4-3-"), Ok(Symbol("cdr4-3-".to_string())));
        assert_eq!(parse::read("(cdr4-43- . ())"),
                   Ok(Cons(Box::new(Symbol("cdr4-43-".to_string())),
                           Box::new(Nil))));
    }
}

#[cfg(not(test))]
fn main() {
}
