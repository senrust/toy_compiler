use super::error::{error_exit, invalid_token_exit};
use super::tokenizer::{BracesKind, OperationKind, ParenthesesKind, TokenList, PROGRAM_TEXT};

#[derive(PartialEq, Eq)]
pub enum PrimaryNodeKind {
    Number(i32),
    LocalVariable(usize), // (offset from bsp)
}

#[derive(PartialEq, Eq)]
pub enum ASTNodeKind {
    Operation(OperationKind),
    Primary(PrimaryNodeKind),
    Assign(usize), // =の文字列中の位置 左辺値に誤りがある場合に渡せるようにする
    Return,
    If,
    IfElse,
    While,
    For,
    MultStmt, // Mult statement, 複文
    FunctionCall(String),
    Reference(usize), // &の文字列中の位置 右辺が変数でない場合にエラーにする
    Dereference(usize),
}

pub struct ASTNode {
    pub node_kind: ASTNodeKind,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
    pub vec: Option<Vec<Option<Box<ASTNode>>>>,
}

impl ASTNode {
    fn new_primary_node(primary_node: PrimaryNodeKind) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Primary(primary_node),
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_reference_node(node_pos: usize) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Reference(node_pos),
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_deference_node(node_pos: usize) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Dereference(node_pos),
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_operand_node(kind: OperationKind) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Operation(kind),
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_assign_node(node_pos: usize) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Assign(node_pos),
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_return_node() -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::Return,
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_if_node() -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::If,
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_while_node() -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::While,
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_for_node() -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::For,
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_multstmt_node() -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::MultStmt,
            left: None,
            right: None,
            vec: None,
        }
    }

    fn new_funtioncall_node(function_name: String) -> ASTNode {
        ASTNode {
            node_kind: ASTNodeKind::FunctionCall(function_name),
            left: None,
            right: None,
            vec: None,
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
            | stmt    = expr ";"
            | "if" "(" expr ")" stmt ("else" stmt)?
            | "while" "(" expr ")" stmt
            | "for" "(" expr? ";" expr? ";" expr? ")" stmt
            |"return" expr ";"
            | "{" stmt* "}"
expr       = assign
assign     = equality ("=" assign)?
equality   = relational ("==" relational | "!=" relational)*
relational = add ("<" add | "<=" add | ">" add | ">=" add)*
add        = mul ("+" mul | "-" mul)*
mul        = unary ("*" unary | "/" unary)*
unary      = ("+" | "-")? primary
primary    = num | | ident ("(" ")")? | "(" expr ")"
*/

type Link = Option<Box<ASTNode>>;
// ;で区切られた領域のASTを作成する
pub struct AST {
    pub root: Link,
}

impl AST {
    pub fn new(token_list: &mut TokenList) -> AST {
        let ast_tree = AST {
            root: AST::stmt(token_list),
        };
        ast_tree
    }

    //stmt = expr ";"
    //      | stmt    = expr ";"
    //      | "if" "(" expr ")" stmt ("else" stmt)?
    //      | "while" "(" expr ")" stmt
    //      | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    //      |"return" expr ";"
    //      | "{" stmt* "}"
    fn stmt(token_list: &mut TokenList) -> Link {
        let stmt_link;
        if token_list.consume_return() {
            let mut return_node = ASTNode::new_return_node();
            // 左辺値だけで良い
            return_node.add_neighbor_node(AST::expr(token_list), None);
            stmt_link = Some(Box::new(return_node));
            token_list.consume_statement_end();
        } else if token_list.consume_if() {
            stmt_link = AST::stmt_if(token_list);
        } else if token_list.consume_while() {
            stmt_link = AST::stmt_while(token_list);
        } else if token_list.consume_for() {
            stmt_link = AST::stmt_for(token_list);
        } else if token_list.comsume_braces(BracesKind::LeftBraces) {
            // 複文の場合
            let mut stmt_node = ASTNode::new_multstmt_node();
            let mut stmt_vec: Vec<Link> = vec![];
            while !token_list.comsume_braces(BracesKind::RightBraces) {
                stmt_vec.push(AST::stmt(token_list));
            }
            stmt_node.vec = Some(stmt_vec);
            stmt_link = Some(Box::new(stmt_node));
        } else {
            stmt_link = AST::expr(token_list);
            token_list.consume_statement_end();
        }

        stmt_link
    }

    fn stmt_if(token_list: &mut TokenList) -> Link {
        // "if" を取り出し
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            // まずはelseのないif文としてASTnodeを作る
            let mut if_node = ASTNode::new_if_node();
            if_node.add_neighbor_node(AST::expr(token_list), None);
            // ")"でクローズされているかチェック
            if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                if_node.right = AST::stmt(token_list);
                // else文が続く場合
                if token_list.consume_else() {
                    if_node.node_kind = ASTNodeKind::IfElse;
                    if_node.vec = Some(vec![AST::stmt_else(token_list)]);
                }
                return Some(Box::new(if_node));
            } else {
                if let Some(valid_token) = token_list.pop_head() {
                    error_exit("if contition must be expression", valid_token.token_pos);
                } else {
                    // テキスト終端に要求エラーを立てる
                    let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                    error_exit("parenthes is not closed", tail_pos);
                }
            }
        } else {
            if let Some(valid_token) = token_list.pop_head() {
                error_exit("if condition must start '(' ", valid_token.token_pos);
            } else {
                // テキスト終端に要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit("if condition is not given", tail_pos);
            }
        }
    }

    fn stmt_else(token_list: &mut TokenList) -> Link {
        AST::stmt(token_list)
    }

    fn stmt_while(token_list: &mut TokenList) -> Link {
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            let mut while_node = ASTNode::new_while_node();
            while_node.add_neighbor_node(AST::expr(token_list), None);
            // ")"でクローズされているかチェック
            if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                while_node.right = AST::stmt(token_list);
                return Some(Box::new(while_node));
            } else {
                if let Some(valid_token) = token_list.pop_head() {
                    error_exit("while contition must be expression", valid_token.token_pos);
                } else {
                    // テキスト終端に要求エラーを立てる
                    let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                    error_exit("parenthes is not closed", tail_pos);
                }
            }
        } else {
            if let Some(valid_token) = token_list.pop_head() {
                error_exit("while condition must start '(' ", valid_token.token_pos);
            } else {
                // テキスト終端に要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit("while condition is not given", tail_pos);
            }
        }
    }

    fn stmt_for(token_list: &mut TokenList) -> Link {
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            let mut for_node = ASTNode::new_for_node();
            let mut for_vec: Vec<Link> = vec![]; // for文用のvecを作成
                                                 // 初期化式
            if token_list.is_statement_end() {
                token_list.pop_head();
                for_vec.push(None);
            } else {
                for_vec.push(AST::expr(token_list));
                if !token_list.consume_statement_end() {
                    invalid_token_exit("for initialzer must be expression", token_list);
                }
            }
            // 判定式
            if token_list.is_statement_end() {
                token_list.pop_head();
                for_vec.push(None);
            } else {
                for_vec.push(AST::expr(token_list));
                if !token_list.consume_statement_end() {
                    invalid_token_exit("for judge must be expression", token_list);
                }
            }
            // 更新式
            if token_list.is_parentheses(ParenthesesKind::RightParentheses) {
                token_list.pop_head();
                for_vec.push(None);
            } else {
                for_vec.push(AST::expr(token_list));
                if !token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                    invalid_token_exit("for updater must be expression", token_list);
                }
            }
            for_node.left = AST::stmt(token_list);
            for_node.vec = Some(for_vec);
            return Some(Box::new(for_node));
        } else {
            if let Some(valid_token) = token_list.pop_head() {
                error_exit("for condition must start '(' ", valid_token.token_pos);
            } else {
                // テキスト終端に要求エラーを立てる
                let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                error_exit("for condition is not given", tail_pos);
            }
        }
    }

    // expr  = assign
    fn expr(token_list: &mut TokenList) -> Link {
        AST::assign(token_list)
    }

    // assign = equality ("=" assign)?
    fn assign(token_list: &mut TokenList) -> Link {
        let mut assign_link = AST::equality(token_list);
        // assignは左辺値が変数でないかのチェックをASTのコンパイル時に行うので,
        // tokenの位置を取得する必要がある
        if token_list.is_assign() {
            let assign_token = token_list.pop_head().unwrap();
            let mut assign_node = ASTNode::new_assign_node(assign_token.token_pos);
            assign_node.add_neighbor_node(assign_link.take(), AST::assign(token_list));
            assign_link = Some(Box::new(assign_node));
        }
        assign_link
    }

    // equality   = relational ("==" relational | "!=" relational)*
    fn equality(token_list: &mut TokenList) -> Link {
        let mut equality_link = AST::relational(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Eq) {
                let mut equality_node = ASTNode::new_operand_node(OperationKind::Eq);
                equality_node.add_neighbor_node(equality_link.take(), AST::relational(token_list));
                equality_link = Some(Box::new(equality_node));
            } else if token_list.consume_operation(OperationKind::Not) {
                let mut equality_node = ASTNode::new_operand_node(OperationKind::Not);
                equality_node.add_neighbor_node(equality_link.take(), AST::relational(token_list));
                equality_link = Some(Box::new(equality_node));
            } else {
                break;
            }
        }
        equality_link
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(token_list: &mut TokenList) -> Link {
        let mut relational_link = AST::add(token_list);

        loop {
            // Gt,Geは左辺と右辺を逆転させてLt, Leで評価する
            if token_list.consume_operation(OperationKind::Gt) {
                let mut relational_node = ASTNode::new_operand_node(OperationKind::Lt);
                relational_node.add_neighbor_node(AST::add(token_list), relational_link.take());
                relational_link = Some(Box::new(relational_node));
            } else if token_list.consume_operation(OperationKind::Ge) {
                let mut relational_node = ASTNode::new_operand_node(OperationKind::Le);
                relational_node.add_neighbor_node(AST::add(token_list), relational_link.take());
                relational_link = Some(Box::new(relational_node));
            } else if token_list.consume_operation(OperationKind::Lt) {
                let mut relational_node = ASTNode::new_operand_node(OperationKind::Lt);
                relational_node.add_neighbor_node(relational_link.take(), AST::add(token_list));
                relational_link = Some(Box::new(relational_node));
            } else if token_list.consume_operation(OperationKind::Le) {
                let mut relational_node = ASTNode::new_operand_node(OperationKind::Le);
                relational_node.add_neighbor_node(relational_link.take(), AST::add(token_list));
                relational_link = Some(Box::new(relational_node));
            } else {
                break;
            }
        }
        relational_link
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(token_list: &mut TokenList) -> Link {
        let mut add_link = AST::mul(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Add) {
                let mut add_node = ASTNode::new_operand_node(OperationKind::Add);
                add_node.add_neighbor_node(add_link.take(), AST::mul(token_list));
                add_link = Some(Box::new(add_node));
            } else if token_list.consume_operation(OperationKind::Sub) {
                let mut add_node = ASTNode::new_operand_node(OperationKind::Sub);
                add_node.add_neighbor_node(add_link.take(), AST::mul(token_list));
                add_link = Some(Box::new(add_node));
            } else {
                break;
            }
        }
        add_link
    }

    // mul  = unary ("*" unary | "/" unary)*
    fn mul(token_list: &mut TokenList) -> Link {
        let mut mul_link = AST::urany(token_list);

        loop {
            if token_list.consume_operation(OperationKind::Mul) {
                let mut mul_node = ASTNode::new_operand_node(OperationKind::Mul);
                mul_node.add_neighbor_node(mul_link.take(), AST::urany(token_list));
                mul_link = Some(Box::new(mul_node));
            } else if token_list.consume_operation(OperationKind::Div) {
                let mut mul_node = ASTNode::new_operand_node(OperationKind::Div);
                mul_node.add_neighbor_node(mul_link.take(), AST::urany(token_list));
                mul_link = Some(Box::new(mul_node));
            } else {
                break;
            }
        }
        mul_link
    }

    // unary = ("+" | "-")? primary
    fn urany(token_list: &mut TokenList) -> Link {
        if token_list.consume_operation(OperationKind::Add) {
            return AST::primary(token_list);
        } else if token_list.consume_operation(OperationKind::Sub) {
            let mut unary_node = ASTNode::new_operand_node(OperationKind::Sub);
            let zoro_node = ASTNode::new_primary_node(PrimaryNodeKind::Number(0));
            unary_node.add_neighbor_node(Some(Box::new(zoro_node)), AST::primary(token_list));
            return Some(Box::new(unary_node));
        } else if token_list.is_operation(OperationKind::Mul) {
            // アドレスは対象が変数でないかのチェックをASTのコンパイル時に行うので,
            // tokenの位置を取得する必要がある
            let dereference_token = token_list.pop_head().unwrap();
            let mut dereference_node = ASTNode::new_deference_node(dereference_token.token_pos);
            dereference_node.add_neighbor_node(AST::urany(token_list), None);
            return Some(Box::new(dereference_node));
        } else if token_list.is_reference() {
            // アドレスは対象が変数でないかのチェックをASTのコンパイル時に行うので,
            // tokenの位置を取得する必要がある
            let reference_token = token_list.pop_head().unwrap();
            let mut reference_node = ASTNode::new_reference_node(reference_token.token_pos);
            reference_node.add_neighbor_node(AST::urany(token_list), None);
            return Some(Box::new(reference_node));
        }
        return AST::primary(token_list);
    }

    // primary    = num | ident ("(" ")")? | "(" expr ")"
    fn primary(token_list: &mut TokenList) -> Link {
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            let node = AST::expr(token_list);
            if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                return node;
            } else {
                invalid_token_exit("parenthes is not closed", token_list);
            }
        }

        // 関数呼び出しの場合
        // 識別子の次が(の場合には関数呼び出しトークンとしている
        if let Some(function_name) = token_list.consume_functioncall() {
            // "(" token取り出し
            token_list.pop_head();
            let mut function_call_node = ASTNode::new_funtioncall_node(function_name);
            let mut args_vec: Vec<Link> = vec![];
            while !token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                if !token_list.is_empty() {
                    if args_vec.len() == 6 {
                        if let Some(valid_token) = token_list.pop_head() {
                            error_exit("canoot take to much argument", valid_token.token_pos);
                        } else {
                            // テキスト終端に要求エラーを立てる
                            let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                            error_exit("function call is not closed", tail_pos);
                        }
                    }

                    let primarykind = token_list.expect_primary();
                    let primary_node = ASTNode::new_primary_node(primarykind);
                    args_vec.push(Some(Box::new(primary_node)));

                    if !token_list.consume_commma() {
                        if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                            break;
                        }

                        if let Some(valid_token) = token_list.pop_head() {
                            error_exit(
                                "function args must be separated by comma",
                                valid_token.token_pos,
                            );
                        } else {
                            // テキスト終端に要求エラーを立てる
                            let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                            error_exit("function call is not closed", tail_pos);
                        }
                    }
                } else {
                    // テキスト終端に要求エラーを立てる
                    let tail_pos = PROGRAM_TEXT.get().unwrap().get_tail_pos();
                    error_exit("function call is not closed", tail_pos);
                }
            }
            function_call_node.vec = Some(args_vec);
            return Some(Box::new(function_call_node));
        }

        let primarykind = token_list.expect_primary();
        let primary_node = ASTNode::new_primary_node(primarykind);
        return Some(Box::new(primary_node));
    }
}

pub struct FuntionInfo {
    pub function_name: String,
    pub args_count: usize,
    pub local_stack_size: usize,
}

// ASTはstmt単位で作成し,
// ASTのリストをASTVecとして保存する
pub struct FunctionAST {
    pub function_ast: AST,
    pub function_info: FuntionInfo,
}

fn pop_function_info(token_list: &mut TokenList) -> FuntionInfo {
    if let Some(function_name) = token_list.consume_function_definition() {
        if token_list.comsume_parentheses(ParenthesesKind::LeftParentheses) {
            let mut args_count = 0;
            loop {
                if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                    break;
                }
                token_list.expect_variable();
                args_count += 1;
                if args_count == 7 {
                    invalid_token_exit("too many argument", token_list);
                }
                if token_list.consume_commma() {
                    continue;
                } else {
                    if token_list.comsume_parentheses(ParenthesesKind::RightParentheses) {
                        break;
                    } else {
                        invalid_token_exit("function argument is not coorect", token_list);
                    }
                }
            }
            let function_info = FuntionInfo {
                function_name,
                args_count,
                local_stack_size: token_list.local_stack_size,
            };
            function_info
        } else {
            invalid_token_exit("function  definition requires '('", token_list);
        }
    } else {
        invalid_token_exit("invalid function defition", token_list);
    }
}

impl FunctionAST {
    fn new(token_list: &mut TokenList) -> FunctionAST {
        let function_info: FuntionInfo = pop_function_info(token_list);
        let function_ast: AST = AST::new(token_list);

        let fucntion_ast = FunctionAST {
            function_ast,
            function_info,
        };

        // 関数は複文{}なのでASTは1つなのでここには来ないはず
        if !token_list.is_empty() {
            error_exit(
                "some error happened",
                token_list.pop_head().unwrap().token_pos,
            );
        }
        fucntion_ast
    }

    pub fn make_function_ast(token_list: &mut TokenList) -> FunctionAST {
        let fucntion_ast = FunctionAST::new(token_list);
        fucntion_ast
    }
}
