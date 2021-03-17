use super::tokenizer::{TokenList, OperationKind, ParenthesesKind};
use super::error::error_exit;

pub enum ASTNodeKind {
    Operation(OperationKind),
    Number(i32),
}

pub struct ASTNode {
    pub node_kind: ASTNodeKind,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

impl ASTNode {
    fn new_num_node(num: i32) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Number(num),
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

    // ASTNodeを更新
    fn add_neighbor_node(&mut self, left: Link, right: Link) {
        self.left = left;
        self.right = right;
    }
}

// AST 生成規則
// expr    = mul ("+" mul | "-" mul)*
// mul     = primary ("*" primary | "/" primary)*
// primary = num | "(" expr ")"
type Link = Option<Box<ASTNode>>;

pub struct AST {
    pub root: Link,
}


impl AST {
    pub fn new(token_list: &mut TokenList) -> AST {
        let ast_tree = AST {
            root: AST::expr(token_list),
        };
        // トークン連想配列が空でない場合、exprでNode化できていないトークンがあるのでエラーにする
        if !token_list.is_empty() {
            let rem_token = token_list.head.take().unwrap();
            error_exit("this token is invalid", rem_token.token_pos, &token_list.raw_text);
        }
        ast_tree
    }

    fn expr(token_list: &mut TokenList) ->  Link {
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

    fn mul(token_list: &mut TokenList) -> Link {
        let mut mul_node = AST::primary(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Mul) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Mul);
                new_node.add_neighbor_node(mul_node.take(), AST::primary(token_list));
                mul_node = Some(Box::new(new_node));
            } else if token_list.consume_operation(OperationKind::Div) {
                let mut new_node = ASTNode::new_operand_node(OperationKind::Div);
                new_node.add_neighbor_node(mul_node.take(), AST::primary(token_list));
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
        return Some(Box::new(ASTNode::new_num_node(token_list.expect_number())));
    }

}