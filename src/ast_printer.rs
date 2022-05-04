use crate::expr::*;
use crate::token_type::*;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        // We can implement code interpretation for each expression here.
        unimplemented!()
    }
}
