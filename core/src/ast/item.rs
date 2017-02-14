use ast::{Node, Index, Slice, Ident, OperatorKind, VariableDeclarationKind};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Undefined,
    Null,
    True,
    False,
    Number(Slice),
    Binary(u64),
    String(Slice),
    RawQuasi(Slice),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item<'src> {
    // Identifiers
    Identifier(Ident<'src>),
    This,

    // Expressions
    ValueExpr(Value),
    ArrayExpr(Index),
    SequenceExpr(Index),
    MemberExpr {
        object: Index,
        property: Index,
    },
    CallExpr {
        callee: Index,
        arguments: Option<Index>,
    },
    BinaryExpr {
        parenthesized: bool,
        operator: OperatorKind,
        left: Index,
        right: Index,
    },
    Prefix {
        operator: OperatorKind,
        operand: Index,
    },
    PostfixExpr {
        operator: OperatorKind,
        operand: Index,
    },
    ConditionalExpr {
        test: Index,
        consequent: Index,
        alternate: Index,
    },
    ArrowExpr {
        params: Option<Index>,
        body: Option<Index>,
    },
    FunctionExpr {
        name: Option<Ident<'src>>,
        params: Option<Index>,
        body: Option<Index>,
    },
    ClassExpr {
        name: Option<Ident<'src>>,
        extends: Option<Ident<'src>>,
        body: Option<Index>,
    },


    // Declaration
    VariableDeclarator {
        name: Index,
        value: Option<Index>,
    },

    // Statements
    EmptyStatement,
    ExpressionStatement(Index),
    DeclarationStatemenet {
        kind: VariableDeclarationKind,
        declarators: Index,
    },
    FunctionStatement {
        name: Ident<'src>,
        params: Option<Index>,
        body: Option<Index>,
    },
    ReturnStatement {
        value: Option<Index>,
    }

}

impl<'src> Item<'src> {
    #[inline]
    pub fn at(self, start: usize, end: usize) -> Node<'src> {
        Node::new(start, end, self)
    }
}
