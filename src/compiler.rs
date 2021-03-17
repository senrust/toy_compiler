use crate::ast::{AST, ASTNode, ASTNodeKind};
use super::tokenizer::OperationKind;

fn compile_node(mut node: ASTNode, instruction_vec: &mut Vec<String>) {
    if let ASTNodeKind::Number(num) = node.node_kind {
        let instruction = format!("    push {}", num);
        instruction_vec.push(instruction);
        return;
    }

    // 渡されたastは正しいのでunwrapしても問題ない
    let left_node = node.left.take().unwrap();
    let right_node = node.right.take().unwrap();
    compile_node(*left_node, instruction_vec);
    compile_node(*right_node, instruction_vec);

    instruction_vec.push("    pop rdi".to_string());
    instruction_vec.push("    pop rax".to_string());

    match node.node_kind {
        ASTNodeKind::Operation(OperationKind::Add) => {
            instruction_vec.push("    add rax, rdi".to_string());
        },
        ASTNodeKind::Operation(OperationKind::Sub) => {
            instruction_vec.push("    sub rax, rdi".to_string());
        },
        ASTNodeKind::Operation(OperationKind::Mul) => {
            instruction_vec.push("    imul rax, rdi".to_string());
        },
        ASTNodeKind::Operation(OperationKind::Div) => {
            instruction_vec.push("    cqo".to_string());
            instruction_vec.push("    idiv rdi".to_string());
        },
        ASTNodeKind::Operation(OperationKind::Eq) => {
            instruction_vec.push("    cmp rax, rdi".to_string());
            instruction_vec.push("    sete al".to_string());
            instruction_vec.push("    movzb rax, al".to_string());
        },
        ASTNodeKind::Operation(OperationKind::Not) => {
            instruction_vec.push("    cmp rax, rdi".to_string());
            instruction_vec.push("    setne al".to_string());
            instruction_vec.push("    movzb rax, al".to_string());
        },
        // Gt, GeはASTでは左辺値と右辺値を反転させたLt, Leとして形成される
        ASTNodeKind::Operation(OperationKind::Lt) => {
            instruction_vec.push("    cmp rax, rdi".to_string());
            instruction_vec.push("    setl al".to_string());
            instruction_vec.push("    movzb rax, al".to_string());
        },
        ASTNodeKind::Operation(OperationKind::Le) => {
            instruction_vec.push("    cmp rax, rdi".to_string());
            instruction_vec.push("    setle al".to_string());
            instruction_vec.push("    movzb rax, al".to_string());
        },
        // ASTNodeKind::Numberはここには来ない
        // Gt, GeはASTでは左辺値と右辺値を反転させたLt, Leとして形成される 
        _ => {},
    }
    instruction_vec.push("    push rax".to_string());
}

// astからアセンブラを出力する
// 渡されるastはrooがNoneか, 正しいASTである
pub fn compile_ast(ast: &mut AST) -> Vec<String> {
    let mut instruction_vec: Vec<String> = vec![];
    match ast.root.take() {
        Some(top_node) => {
            compile_node(*top_node, &mut instruction_vec);
        },
        None => {},
    }   
    instruction_vec
}
