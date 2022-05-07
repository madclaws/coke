use crate::expr::*;
use crate::token_type::*;

pub struct AstPrinter;

impl AstPrinter {
    fn paranthesize(&mut self, expr_name: &str, expressions: Vec<&Expr>) -> String {
        let mut builder = String::from("(");
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
            Expr::Binary(left_expr, token, right_expr) => {
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

#[cfg(test)]
mod tests {
    use super::AstPrinter;
    use crate::expr::*;
    use crate::token_type::*;
    use crate::token::*;

    #[test]
    fn creating_binary_expressions() {
        let add_token = Token::new(TokenType::Plus, String::from("+"), None, 0);
        let num_1 = Box::new(Expr::Lit(Some(Literal::Numbers(2.3))));  
        let num_2 = Box::new(Expr::Lit(Some(Literal::Numbers(3.0)))); 

        let add_expr = Expr::Binary(num_1, add_token , num_2);
        
        let mut ast_printer = AstPrinter{};
        let result = ast_printer.visit_expr(&add_expr);
        assert_eq!(result, "(+ 2.3 3)");
    }

    #[test]
    fn creating_group_expressions() {
        let add_token = Token::new(TokenType::Plus, String::from("+"), None, 0);
        let num_1 = Box::new(Expr::Lit(Some(Literal::Numbers(2.0))));  
        let num_2 = Box::new(Expr::Lit(Some(Literal::Numbers(3.0)))); 

        let add_expr = Box::new(Expr::Binary(num_1, add_token, num_2)); 
        let grouping = Expr::Grouping(add_expr);

        let mut ast_printer = AstPrinter{};
        let result = ast_printer.visit_expr(&grouping);
        assert_eq!(result, "(group (+ 2 3))");
    }

    #[test]
    fn creating_unary_expressions() {
        let add_token = Token::new(TokenType::Plus, String::from("+"), None, 0);
        let num_1 = Box::new(Expr::Lit(Some(Literal::Numbers(2.0))));  

        let unary = Expr::Unary(add_token, num_1);

        let mut ast_printer = AstPrinter{};
        let result = ast_printer.visit_expr(&unary);
        assert_eq!(result, "(+ 2)");
    }

    #[test]
    fn creating_composed_expressions() {
        // (-123) * (45.67)
        //  -> (* (- 123) (group 45.67))
        let minus_token = Token::new(TokenType::Minus, String::from("-"), None, 0);
        let num_1 = Box::new(Expr::Lit(Some(Literal::Numbers(123.0))));  

        let unary = Box::new(Expr::Unary(minus_token, num_1)); 
        let grouped_number = Box::new(Expr::Grouping(Box::new(Expr::Lit(Some(Literal::Numbers(45.67)))))); 

        let star_token = Token::new(TokenType::Star, String::from("*"), None, 0);

        let final_expression = Expr::Binary(
            unary,
            star_token,
            grouped_number
        );
        let mut ast_printer = AstPrinter{};
        let result = ast_printer.visit_expr(&final_expression);
        println!("{result:?}");
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}


