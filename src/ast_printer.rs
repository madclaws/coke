use crate::expr::*;
use crate::token_type::*;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        // We can implement code interpretation for each expression here.
        match expr {
            Expr::Lit(literal) => {
                if literal.is_none() {
                    String::from("nil")
                } else {
                    match literal.as_ref().unwrap() {
                        Literal::Numbers(num) => num.to_string(),
                        Literal::Strings(string) => string.to_string() 
                    }
                }
            },
            Expr::Binary(_token, _left_expr, _right_expr) => String::from(""),
            Expr::Grouping(_expr) => String::from(""),
            Expr::Unary(_token, _left_expr) => String::from(""),
        }
    }
}
