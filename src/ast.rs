use super::error::error_exit;
use super::tokenizer::{OperationKind, ParenthesesKind, TokenList};

pub enum PrimaryNodeKind {
    Number(i32),
    Variable(char, i32), // (variablename, offset from bsp)
}

pub enum ASTNodeKind {
    Operation(OperationKind),
    Primary(PrimaryNodeKind),
    Assign,
}

pub struct ASTNode {
    pub node_kind: ASTNodeKind,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

impl ASTNode {
    fn new_primary_node(primary_node: PrimaryNodeKind) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Primary(primary_node),
            left: None,
            right: None,
        }
    }

    fn new_operand_node(kind: OperationKind) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Operation(kind),
            left: None,
            right: None,
        }
    }

    fn new_assign_node() -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Assign,
            left: None,
            right: None,
        }
    }

    // ASTNodeを更新
    fn add_neighbor_node(&mut self, left: Link, right: Link) {
        self.left = left;
        self.right = right;
    }
}

/*
AST 生成規則
program    = stmt*
stmt       = expr ";"
expr       = assign
assign     = equality ("=" assign)?
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num | ident | "(" expr ")"
*/

type Link = Option<Box<ASTNode>>;
// ;で区切られた領域のASTを作成する
pub struct AST {
    pub root: Link,
}

impl AST {
    pub fn new(token_list: &mut TokenList) -> AST {
        let ast_tree = AST {
            root: AST::expr(token_list),
        };
        ast_tree
    }

    // expr  = assign
    fn expr(token_list: &mut TokenList) -> Link {
        AST::assign(token_list)
    }

    // assign = equality ("=" assign)?
    fn assign(token_list: &mut TokenList) -> Link {
        let mut assign_node = AST::equality(token_list);

        if token_list.consume_assign() {
            let mut new_node = ASTNode::new_assign_node();
            new_node.add_neighbor_node(assign_node.take(), AST::assign(token_list));
            assign_node = Some(Box::new(new_node));
        }
        assign_node
    }

    // equality   = relational ("==" relational | "!=" relational)*
    fn equality(token_list: &mut TokenList) -> Link {
        let mut equality_node = AST::relational(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Eq) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Eq);
                new_node.add_neighbor_node(equality_node.take(), AST::relational(token_list));
                equality_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Not) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Not);
                new_node.add_neighbor_node(equality_node.take(), AST::relational(token_list));
                equality_node = Some(Box::new(new_node));
            } else {
                break;
            }
        }
        equality_node
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(token_list: &mut TokenList) -> Link {
        let mut relational_node = AST::add(token_list);

        loop {
            // Gt,Geは左辺と右辺を逆転させてLt, Leで評価する
            if token_list.consume_operation(OperationKind::Gt) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Lt);
                new_node.add_neighbor_node(AST::add(token_list), relational_node.take());
                relational_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Ge) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Le);
                new_node.add_neighbor_node(AST::add(token_list), relational_node.take());
                relational_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Lt) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Lt);
                new_node.add_neighbor_node(relational_node.take(), AST::add(token_list));
                relational_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Le) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Le);
                new_node.add_neighbor_node(relational_node.take(), AST::add(token_list));
                relational_node = Some(Box::new(new_node));
            } else {
                break;
            }
        }
        relational_node
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(token_list: &mut TokenList) -> Link {
        let mut expr_node = AST::mul(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Add) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Add);
                new_node.add_neighbor_node(expr_node.take(), AST::mul(token_list));
                expr_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Sub) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Sub);
                new_node.add_neighbor_node(expr_node.take(), AST::mul(token_list));
                expr_node = Some(Box::new(new_node));
            } else {
                break;
            }
        }
        expr_node
    }

    // mul  = unary ("*" unary | "/" unary)*
    fn mul(token_list: &mut TokenList) -> Link {
        let mut mul_node = AST::urany(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Mul) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Mul);
                new_node.add_neighbor_node(mul_node.take(), AST::urany(token_list));
                mul_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Div) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Div);
                new_node.add_neighbor_node(mul_node.take(), AST::urany(token_list));
                mul_node = Some(Box::new(new_node));
            } else {
                break;
            }
        }
        mul_node
    }

    // unary = ("+" | "-")? primary
    fn urany(token_list: &mut TokenList) -> Link {
        if token_list.consume_operation(OperationKind::Add) {
            return AST::primary(token_list);
        }
        if token_list.consume_operation(OperationKind::Sub) {
            let mut new_node = ASTNode::new_operand_node(OperationKind::Sub);
            let zoro_node = ASTNode::new_primary_node(PrimaryNodeKind::Number(0));
            new_node.add_neighbor_node(Some(Box::new(zoro_node)), AST::primary(token_list));
            return Some(Box::new(new_node));
        }
        return AST::primary(token_list);
    }

    // primary    = num | ident | "(" expr ")"
    fn primary(token_list: &mut TokenList) -> Link {
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            let node = AST::expr(token_list);
            if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                return node;
            } else {
                if let Some(valid_token) = token_list.pop_head() {
                    error_exit(
                        "parenthes is not closed",
                        valid_token.token_pos,
                        &token_list.raw_text,
                    );
                } else {
                    // テキスト終端に要求エラーを立てる
                    let tail_pos = token_list.raw_text.chars().count();
                    error_exit("parenthes is not closed", tail_pos, &token_list.raw_text);
                }
            }
        }
        return Some(Box::new(ASTNode::new_primary_node(
            token_list.expect_primary(),
        )));
    }
}

/*
AST 生成規則
program    = stmt*
stmt       = expr ";"
expr       = assign
assign     = equality ("=" assign)?
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num | ident | "(" expr ")"
*/
// ASTはstmt単位で作成し,
// ASTのリストをASTVecとして保存する
pub struct ASTVec {
    pub vec: Vec<AST>,
}

impl ASTVec {
    fn new() -> ASTVec {
        ASTVec { vec: vec![] }
    }

    pub fn make_ast_vec(token_list: &mut TokenList) -> ASTVec {
        let mut ast_vec = ASTVec::new();
        while !token_list.is_empty() {
            let ast: AST = AST::new(token_list);
            token_list.consume_statement_end();
            ast_vec.vec.push(ast);
        }
        ast_vec
    }
}
