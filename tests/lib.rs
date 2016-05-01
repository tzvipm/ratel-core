extern crate badger;

use badger::grammar::*;
use badger::parser::parse;
use badger::grammar::Statement::*;
use badger::grammar::Expression::*;
use badger::grammar::ClassMember::*;
use badger::grammar::OperatorType::*;

macro_rules! assert_parse {
    ($string:expr, $body:expr) => {
        assert_eq!(parse($string.to_string()).body, $body);
    }
}

macro_rules! assert_expression {
    ($string:expr, $ex:expr) => {
        match parse($string.to_string()).body[0] {
            ExpressionStatement(ref expression) => assert_eq!(*expression, $ex),
            _                                   => panic!("No expression found"),
        }
    }
}

macro_rules! assert_statement {
    ($string:expr, $ex:expr) => (assert_parse!($string, vec![$ex]))
}

macro_rules! num {
    ($num:expr) => (LiteralExpression(LiteralFloat($num)))
}

macro_rules! boxnum {
    ($num:expr) => (Box::new(num!($num)))
}

macro_rules! ident {
    ($name:expr) => (IdentifierExpression($name.to_string()))
}

macro_rules! param {
    ($name:expr) => (Parameter {
        name: $name.to_string()
    })
}


#[test]
fn var_declare() {
    assert_statement!("var foo = 100;", VariableDeclarationStatement {
        kind: VariableDeclarationKind::Var,
        declarations: vec![(
            "foo".to_string(),
            num!(100.0)
        )]
    });
}

#[test]
fn let_declare() {
    assert_statement!("let foo = 100;", VariableDeclarationStatement {
        kind: VariableDeclarationKind::Let,
        declarations: vec![(
            "foo".to_string(),
            num!(100.0)
        )]
    });
}


#[test]
fn const_declare() {
    assert_statement!("const foo = 100;", VariableDeclarationStatement {
        kind: VariableDeclarationKind::Const,
        declarations: vec![(
            "foo".to_string(),
            num!(100.0)
        )]
    });
}

#[test]
fn var_muliple_declare() {
    assert_statement!("var foo = 100, bar = 200;", VariableDeclarationStatement {
        kind: VariableDeclarationKind::Var,
        declarations: vec![(
            "foo".to_string(),
            num!(100.0)
        ), (
            "bar".to_string(),
            num!(200.0)
        )]
    });
}

#[test]
fn identifier_expression() {
    assert_expression!("foobar", ident!("foobar"))
}

#[test]
fn null_expression() {
    assert_expression!("null", LiteralExpression(LiteralNull));
}

#[test]
fn undefined_expression() {
    assert_expression!("undefined", LiteralExpression(LiteralUndefined));
}

#[test]
fn true_expression() {
    assert_expression!("true", LiteralExpression(LiteralTrue));
}

#[test]
fn false_expression() {
    assert_expression!("false", LiteralExpression(LiteralFalse));
}

#[test]
fn number_expression() {
    assert_expression!("100", num!(100.0));
}

#[test]
fn binary_number_expression() {
    assert_expression!("0b1100100", LiteralExpression(LiteralInteger(100)));
}

#[test]
fn octal_number_expression() {
    assert_expression!("0o144", LiteralExpression(LiteralInteger(100)));
}

#[test]
fn hexdec_number_expression() {
    assert_expression!("0x64", LiteralExpression(LiteralInteger(100)));
}

#[test]
fn floating_number_expression() {
    assert_expression!("3.14", num!(3.14));
}

#[test]
fn binary_expression() {
    assert_expression!("true == 1", BinaryExpression {
        left: Box::new(LiteralExpression(LiteralTrue)),
        operator: Equality,
        right: boxnum!(1.0)
    });
}

#[test]
fn op_precedence_left() {
    assert_expression!("1 + 2 * 3", BinaryExpression {
        left: boxnum!(1.0),
        operator: Addition,
        right: Box::new(BinaryExpression {
            left: boxnum!(2.0),
            operator: Multiplication,
            right: boxnum!(3.0),
        }),
    });
}

#[test]
fn op_precedence_right() {
    assert_expression!("1 * 2 + 3", BinaryExpression {
        left: Box::new(BinaryExpression {
            left: boxnum!(1.0),
            operator: Multiplication,
            right: boxnum!(2.0),
        }),
        operator: Addition,
        right: boxnum!(3.0),
    });
}

#[test]
fn function_statement() {
    assert_statement!("

    function foo() {
        return bar;
    }

    ", FunctionStatement {
        name: "foo".to_string(),
        params: vec![],
        body: vec![
            ReturnStatement(ident!("bar"))
        ]
    });
}

#[test]
fn function_with_params_statement() {
    assert_statement!("

    function foo(a, b, c) {
        return bar;
    }

    ", FunctionStatement {
        name: "foo".to_string(),
        params: vec![
            param!("a"),
            param!("b"),
            param!("c"),
        ],
        body: vec![
            ReturnStatement(ident!("bar"))
        ]
    });
}

#[test]
fn if_statement() {
    assert_statement!("

    if (true) {
        foo;
    }

    ", IfStatement {
        test: LiteralExpression(LiteralTrue),
        consequent: Box::new(BlockStatement {
            body: vec![ExpressionStatement(
                ident!("foo")
            )]
        }),
        alternate: None,
    });
}

#[test]
fn if_else_statement() {
    assert_statement!("

    if (true) {
        foo;
    } else {
        bar;
    }

    ", IfStatement {
        test: LiteralExpression(LiteralTrue),
        consequent: Box::new(BlockStatement {
            body: vec![ExpressionStatement(
                ident!("foo")
            )]
        }),
        alternate: Some(Box::new(BlockStatement {
            body: vec![ExpressionStatement(
                ident!("bar")
            )]
        })),
    })
}

#[test]
fn if_else_if_else_statement() {
    assert_statement!("

    if (true) {
        foo;
    } else if(false) {
        bar;
    } else {
        baz;
    }

    ", IfStatement {
        test: LiteralExpression(LiteralTrue),
        consequent: Box::new(BlockStatement {
            body: vec![ExpressionStatement(
                ident!("foo")
            )]
        }),
        alternate: Some(Box::new(IfStatement {
            test: LiteralExpression(LiteralFalse),
            consequent: Box::new(BlockStatement {
                body: vec![ExpressionStatement(
                    ident!("bar")
                )]
            }),
            alternate: Some(Box::new(BlockStatement {
                body: vec![ExpressionStatement(
                    ident!("baz")
                )]
            })),
        })),
    });
}

#[test]
fn if_no_block_statement() {
    assert_statement!("if (true) foo;", IfStatement {
        test: LiteralExpression(LiteralTrue),
        consequent: Box::new(ExpressionStatement(
            ident!("foo")
        )),
        alternate: None,
    });
}

#[test]
fn if_else_no_block_statement() {
    assert_statement!("if (true) foo; else bar;", IfStatement {
        test: LiteralExpression(LiteralTrue),
        consequent: Box::new(ExpressionStatement(
            ident!("foo")
        )),
        alternate: Some(Box::new(ExpressionStatement(
            ident!("bar")
        ))),
    })
}

#[test]
fn while_statement() {
    assert_statement!("

    while (true) {
        foo;
    }

    ", WhileStatement {
        test: LiteralExpression(LiteralTrue),
        body: Box::new(BlockStatement {
            body: vec![ExpressionStatement(
                ident!("foo")
            )]
        }),
    });
}

#[test]
fn while_no_block_statement() {
    assert_statement!("while (true) foo;", WhileStatement {
        test: LiteralExpression(LiteralTrue),
        body: Box::new(ExpressionStatement(
            ident!("foo")
        )),
    });
}

#[test]
fn arrow_function() {
    assert_expression!("

    () => {
        bar;
    }

    ", ArrowFunctionExpression {
        params: vec![],
        body: Box::new(BlockStatement {
            body: vec![
                ExpressionStatement(ident!("bar"))
            ]
        })
    });
}

#[test]
fn arrow_function_shorthand() {
    assert_expression!("n => n * n", ArrowFunctionExpression {
        params: vec![
            param!("n")
        ],
        body: Box::new(ExpressionStatement(
            BinaryExpression {
                left: Box::new(ident!("n")),
                operator: Multiplication,
                right: Box::new(ident!("n")),
            }
        )),
    });
}

#[test]
fn arrow_function_with_params() {
    assert_expression!("

    (a, b, c) => {
        bar;
    }

    ", ArrowFunctionExpression {
        params: vec![
            param!("a"),
            param!("b"),
            param!("c"),
        ],
        body: Box::new(BlockStatement {
            body: vec![
                ExpressionStatement(ident!("bar"))
            ]
        })
    });
}

#[test]
fn function_expression() {
    assert_expression!("

    foo = function () {
        return bar;
    }

    ", BinaryExpression {
        left: Box::new(ident!("foo")),
        operator: Assign,
        right: Box::new(FunctionExpression {
            name: None,
            params: vec![],
            body: vec![
                ReturnStatement(ident!("bar"))
            ]
        })
    });
}

#[test]
fn named_function_expression() {
    assert_expression!("

    foo = function foo() {
        return bar;
    }

    ", BinaryExpression {
        left: Box::new(ident!("foo")),
        operator: Assign,
        right: Box::new(FunctionExpression {
            name: Some("foo".to_string()),
            params: vec![],
            body: vec![
                ReturnStatement(ident!("bar"))
            ]
        })
    });
}

