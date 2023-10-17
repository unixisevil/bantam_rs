mod expression;
mod lexer;
mod parselets;
mod parser;
mod token;

use crate::{expression::Print, parser::BantamParser};

#[repr(u8)]
enum Precedence {
    Assignment = 1,
    Conditional,
    Sum,
    Product,
    Exponent,
    Prefix,
    Postfix,
    Call,
}

fn main() {
    let mut parser = BantamParser::new("a = b + c * d ^ e - f / g");
    let expr = parser.parse_expression();
    let mut out = String::new();
    expr.print(&mut out);

    println!("ast string: {out}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper(source: &'static str, expected: &'static str) -> bool {
        let mut parser = BantamParser::new(source);
        let expr = parser.parse_expression();
        let mut out = String::new();
        expr.print(&mut out);
        out == expected
    }

    #[test]
    fn unary() {
        assert!(test_helper("~ ! - + a", "(~(!(-(+a))))"));
        assert!(test_helper("a ! ! !", "(((a!)!)!)"));
    }

    #[test]
    fn unary_mix_binary() {
        assert!(test_helper("- a * b", "((-a) * b)"));
        assert!(test_helper("! a + b", "((!a) + b)"));
        assert!(test_helper("~ a ^ b", "((~a) ^ b)"));
        assert!(test_helper("- a !", "(-(a!))"));
        assert!(test_helper("! a !", "(!(a!))"));
    }

    #[test]
    fn binary() {
        assert!(test_helper(
            "a = b + c * d ^ e - f / g",
            "(a = ((b + (c * (d ^ e))) - (f / g)))"
        ));
    }

    #[test]
    fn binary_associativity() {
        assert!(test_helper("a = b = c", "(a = (b = c))"));
        assert!(test_helper("a + b - c", "((a + b) - c)"));
        assert!(test_helper("a * b / c", "((a * b) / c)"));
        assert!(test_helper("a ^ b ^ c", "(a ^ (b ^ c))"));
    }

    #[test]
    fn conditional() {
        assert!(test_helper("a ? b : c ? d : e", "(a ? b : (c ? d : e))"));
        assert!(test_helper("a ? b ? c : d : e", "(a ? (b ? c : d) : e)"));
        assert!(test_helper(
            "a + b ? c * d : e / f",
            "((a + b) ? (c * d) : (e / f))"
        ));
    }

    #[test]
    fn func_call() {
        assert!(test_helper("a()", "a()"));
        assert!(test_helper("a(b)", "a(b)"));
        assert!(test_helper("a(b,c)", "a(b, c)"));
        assert!(test_helper("a(b)(c)", "a(b)(c)"));
        assert!(test_helper("a(b)+c(d)", "(a(b) + c(d))"));
        assert!(test_helper("a(b?c:d,e+f)", "a((b ? c : d), (e + f))"));
    }

    #[test]
    fn grouping() {
        assert!(test_helper("a + (b + c) + d", "((a + (b + c)) + d)"));
        assert!(test_helper("a ^ (b + c)", "(a ^ (b + c))"));
        assert!(test_helper("( !a ) !", "((!a)!)"));
    }
}
