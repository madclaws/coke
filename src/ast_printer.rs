use crate::expr::*;
use crate::token_type::*;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary: &Binary) -> String {
        unimplemented!()
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        unimplemented!()
    }

    fn visit_unary(&mut self, unary: &Unary) -> String {
        unimplemented!()
    }

    fn visit_literal(&mut self, literal: &Literals) -> String {
        if literal.value.is_none() {
            String::from("nil")
        } else {
            match literal.value.as_ref().unwrap() {
                Literal::Numbers(num) => num.to_string(),
                Literal::Strings(str) => str.to_string()
            }
        }
    }

    fn paranthesize(&mut self, expr_name: &str, expressions: Vec<Box<dyn Expr>>) -> String {
        let mut builder = String::from("");
        builder.push_str("(");
        builder.push_str(expr_name);
        for expr in expressions {
            builder.push_str(" ");
            // builder.push_str()
            match expr_name {
                "group" => self.visit_grouping(expr)
            }
        }
        builder.push_str(")");
        builder
    }
}
