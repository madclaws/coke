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
}
