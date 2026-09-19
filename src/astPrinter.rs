use crate::expression::Expr;
use crate::token::Literal; // Needed ot convert Literal to its different enum types

fn ast_printer(expr: &Expr) -> String {
    // 
    match expr { // instead of 4 different visit... methods, I can use a match{} in rust
        Expr::Binary { left, operator, right } => {
            parenthesize(&operator.lexeme, &[left, right])
        }
        Expr::Grouping { expression } => {
            parenthesize("group", &[expression])
        }
        Expr::Literal { value_type } => {
            match value_type {
                None => "nil".to_string(),
                Some(Literal::Number(n)) => n.to_string(),
                Some(Literal::String(s)) => s.clone(),
                Some(Literal::Bool(b)) => b.to_string(),
            }
        }
        Expr::Unary { prefix, expression } => {
            parenthesize(&prefix.lexeme, &[expression])
        }
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut result = String::from("");
    result += "(";
    result += name;
    for expr in exprs {
        result += " ";
        result += &ast_printer(&expr);
    }
    result += ")";
    result

}

// UNIT TEST      

#[cfg(test)]
mod tests {
    use super::ast_printer;
    use crate::expression::Expr;
    use crate::token::{Literal, Token};
    use crate::tokenType::TokenType;

    fn minus_token() -> Token {
        Token::new(TokenType::MINUS, "-".to_string(), None, 1)
    }

    fn star_token() -> Token {
        Token::new(TokenType::STAR, "*".to_string(), None, 1)
    }

    #[test]
    fn prints_nil() {
        let expr = Expr::Literal { value_type: None };
        assert_eq!(ast_printer(&expr), "nil");
    }

    #[test]
    fn prints_number_literal() {
        let expr = Expr::Literal {
            value_type: Some(Literal::Number(123.0)),
        };
        assert_eq!(ast_printer(&expr), "123");
    }

    #[test]
    fn prints_string_literal() {
        let expr = Expr::Literal {
            value_type: Some(Literal::String("lox".to_string())),
        };
        assert_eq!(ast_printer(&expr), "lox");
    }

    #[test]
    fn prints_unary() {
        let expr = Expr::Unary {
            prefix: minus_token(),
            expression: Box::new(Expr::Literal {
                value_type: Some(Literal::Number(123.0)),
            }),
        };
        assert_eq!(ast_printer(&expr), "(- 123)");
    }

    #[test]
    fn prints_grouping() {
        let expr = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value_type: Some(Literal::Number(45.0)),
            }),
        };
        assert_eq!(ast_printer(&expr), "(group 45)");
    }

    #[test]
    fn prints_book_example_tree() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                prefix: minus_token(),
                expression: Box::new(Expr::Literal {
                    value_type: Some(Literal::Number(123.0)),
                }),
            }),
            operator: star_token(),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value_type: Some(Literal::Number(45.0)),
                }),
            }),
        };

        assert_eq!(ast_printer(&expr), "(* (- 123) (group 45))");
    }
}
