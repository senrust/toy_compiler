use super::tokenizer::{TokenList, TokenKind, OperationKind, ParenthesesKind};
use super::compiler::{expect_operation, expect_number};
use super::error::error_exit;

pub struct Node {
    node_kind: TokenKind,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new_num_node(num: i32) -> Node {
        Node {
            node_kind: TokenKind::Number(num),
            left: None,
            right: None,
        }
    }

    fn new_operand_node(kind: OperationKind) -> Node {
        Node {
            node_kind: TokenKind::Operation(kind),
            left: None,
            right: None,
        }
    }
}

// AST 生成規則
// expr    = mul ("+" mul | "-" mul)*
// mul     = primary ("*" primary | "/" primary)*
// primary = num | "(" expr ")"
type Link = Option<Box<Node>>;

pub struct AST {
    pub root: Link,
}


impl AST {
    pub fn new(token_list: &mut TokenList) -> AST {
        let ast_tree = AST {
            root: AST::expr(token_list),
        };
        if !token_list.is_empty() {
            let rem_token = token_list.head.take().unwrap();
            error_exit("this token is invalid", rem_token.token_pos, &token_list.raw_text);
        }
        ast_tree
    }

    fn upgrade_node(mut node: Node, right: Link, left: Link) -> Node {
        node.right = right;
        node.left = left;
        node
    }

    fn expr(token_list: &mut TokenList) ->  Link {
        let mut expr_node = AST::mul(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Add) {
                let new_node = Node::new_operand_node(OperationKind::Add);
                let new_node = AST::upgrade_node(new_node, expr_node.take(), AST::mul(token_list));
                expr_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Sub) {
                let new_node = Node::new_operand_node(OperationKind::Sub);
                let new_node = AST::upgrade_node(new_node, expr_node.take(), AST::mul(token_list));
                expr_node = Some(Box::new(new_node));
            } else {
                break;
            }
        }
        expr_node
    }

    fn mul(token_list: &mut TokenList) -> Link {
        let mut mul_node = AST::primary(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Mul) {
                let new_node = Node::new_operand_node(OperationKind::Mul);
                let new_node = AST::upgrade_node(new_node, mul_node.take(), AST::primary(token_list));
                mul_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Div) {
                let new_node = Node::new_operand_node(OperationKind::Div);
                let new_node = AST::upgrade_node(new_node, mul_node.take(), AST::primary(token_list));
                mul_node = Some(Box::new(new_node));
            } else {
                break;
            }
        }
        mul_node
    }

    fn primary(token_list: &mut TokenList) ->  Link {
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            let node = AST::expr(token_list);
            if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                return node;
            } else {
                if let Some(valid_token) = token_list.pop_head() {
                    error_exit("parenthes is not closed", valid_token.token_pos, &token_list.raw_text);
                } else {
                    // テキスト終端に要求エラーを立てる
                    let tail_pos = token_list.raw_text.chars().count();
                    error_exit("parenthes is not closed", tail_pos, &token_list.raw_text);
                }
            }
        }

        return Some(Box::new(Node::new_num_node(token_list.expect_number())));
    }

}