use crate::expr::*;
use crate::token_type::*;

pub struct AstPrinter;

impl AstPrinter {
    fn paranthesize(&mut self, expr_name: &str, expressions: Vec<&Expr>) -> String {
        let mut builder = String::from("");
        builder.push_str(&expr_name);
        for expr in expressions {
            builder.push_str(" ");
            builder.push_str(&self.visit_expr(expr));
        }
        builder.push_str(")");
        builder
    }
}
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
            Expr::Binary(token, left_expr, right_expr) => {
                self.paranthesize(&token.lexeme, vec![left_expr, right_expr])
            },
            Expr::Grouping(expr) => {
                self.paranthesize("group", vec![expr])
            },
            Expr::Unary(token, left_expr) => {
                self.paranthesize(&token.lexeme, vec![left_expr])
            },
        }
    }
}
