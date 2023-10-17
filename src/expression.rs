use crate::token::TokenType;

pub enum Expr<'source> {
    Name(&'source str),
    Assign {
        name: &'source str,
        right: Box<Expr<'source>>,
    },
    Call {
        func: Box<Expr<'source>>,
        args: Vec<Box<Expr<'source>>>,
    },
    Cond {
        cond: Box<Expr<'source>>,
        then_arm: Box<Expr<'source>>,
        else_arm: Box<Expr<'source>>,
    },
    Prefix {
        op: TokenType,
        right: Box<Expr<'source>>,
    },
    Postfix {
        left: Box<Expr<'source>>,
        op: TokenType,
    },
    Infix {
        left: Box<Expr<'source>>,
        op: TokenType,
        right: Box<Expr<'source>>,
    },
}


pub trait Print {
    fn print(&self, out: &mut String);
}


impl<'source> Print for Expr<'source> {
    fn print(&self, out: &mut String) {
        match self {
            Expr::Name(name) => out.push_str(name),
            Expr::Assign { name, right } => {
                out.push('(');
                out.push_str(name);
                out.push_str(" = ");
                right.print(out);
                out.push(')');
            }
            Expr::Cond {
                cond,
                then_arm,
                else_arm,
            } => {
                out.push('(');
                cond.print(out);
                out.push_str(" ? ");
                then_arm.print(out);
                out.push_str(" : ");
                else_arm.print(out);
                out.push(')');
            }
            Expr::Call { func, args } => {
                func.print(out);
                out.push('(');
                for (i, e) in args.iter().enumerate() {
                    e.print(out);
                    if i < args.len() - 1 {
                        out.push_str(", ");
                    }
                }
                out.push(')');
            }
            Expr::Infix { left, op, right } => {
                out.push('(');
                left.print(out);
                out.push(' ');
                out.push(op.punctuator().unwrap());
                out.push(' ');
                right.print(out);
                out.push(')');
            }
            Expr::Prefix { op, right } => {
                out.push('(');
                out.push(op.punctuator().unwrap());
                right.print(out);
                out.push(')');
            }
            Expr::Postfix { left, op } => {
                out.push('(');
                left.print(out);
                out.push(op.punctuator().unwrap());
                out.push(')');
            }
        }
    }
}



