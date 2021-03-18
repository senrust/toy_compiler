use crate::ast::{ASTNode, ASTNodeKind, PrimaryNodeKind, AST};
use crate::error::error_exit;
use super::tokenizer::OperationKind;

// ローカル変数のアドレスをスタックにpushする
fn push_local_variable_address(offset:i32, instruction_vec: &mut Vec<String>) {
    instruction_vec.push(format!("    mov rax [rbp]"));
    instruction_vec.push(format!("    sub rax {}",offset));
    instruction_vec.push(format!("    push rax"));
}

fn compile_node(mut node: ASTNode, instruction_vec: &mut Vec<String>, input_text: &String) {
    if let ASTNodeKind::Primary(PrimaryNodeKind::Number(num)) = node.node_kind {
        let instruction = format!("    push {}", num);
        instruction_vec.push(instruction);
        return;
    } else if let ASTNodeKind::Primary(PrimaryNodeKind::LocalVariable(offset)) = node.node_kind {
        push_local_variable_address(offset, instruction_vec);
        instruction_vec.push(format!("    pop rax"));
        instruction_vec.push(format!("    mov rax, [rax]"));
        instruction_vec.push(format!("    push rax"));
        return;
    } else if let ASTNodeKind::Assign = node.node_kind {
        //  = の左辺値が変数であること
        // 渡されたastは正しいのでunwrapしても問題ない
        let left_node = node.left.take().unwrap();
        let right_node = node.right.take().unwrap();
        if let ASTNodeKind::Primary(PrimaryNodeKind::LocalVariable(offset)) = left_node.node_kind {
            push_local_variable_address(offset, instruction_vec);
            compile_node(*right_node, instruction_vec, input_text);
            instruction_vec.push(format!("    pop rdi"));
            instruction_vec.push(format!("    pop rax"));
            instruction_vec.push(format!("    mov [rax], rdi"));
            instruction_vec.push(format!("    push rdi"));
            return;
        } else {
            error_exit("left value is not correct", node.node_pos, input_text);
        }
    }

    // 渡されたastは正しいのでunwrapしても問題ない
    let left_node = node.left.take().unwrap();
    let right_node = node.right.take().unwrap();
    compile_node(*left_node, instruction_vec, input_text);
    compile_node(*right_node, instruction_vec, input_text);

    instruction_vec.push(format!("    pop rdi"));
    instruction_vec.push(format!("    pop rax"));

    match node.node_kind {
        ASTNodeKind::Operation(OperationKind::Add) => {
            instruction_vec.push(format!("    add rax, rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Sub) => {
            instruction_vec.push(format!("    sub rax, rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Mul) => {
            instruction_vec.push(format!("    imul rax, rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Div) => {
            instruction_vec.push(format!("    cqo"));
            instruction_vec.push(format!("    idiv rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Eq) => {
            instruction_vec.push(format!("    cmp rax, rdi"));
            instruction_vec.push(format!("    sete al"));
            instruction_vec.push(format!("    movzb rax, al"));
        }
        ASTNodeKind::Operation(OperationKind::Not) => {
            instruction_vec.push(format!("    cmp rax, rdi"));
            instruction_vec.push(format!("    setne al"));
            instruction_vec.push(format!("    movzb rax, al"));
        }
        // Gt, GeはASTでは左辺値と右辺値を反転させたLt, Leとして形成される
        ASTNodeKind::Operation(OperationKind::Lt) => {
            instruction_vec.push(format!("    cmp rax, rdi"));
            instruction_vec.push(format!("    setl al"));
            instruction_vec.push(format!("    movzb rax, al"));
        }
        ASTNodeKind::Operation(OperationKind::Le) => {
            instruction_vec.push(format!("    cmp rax, rdi"));
            instruction_vec.push(format!("    setle al"));
            instruction_vec.push(format!("    movzb rax, al"));
        }
        // ASTNodeKind::Numberはここには来ない
        // Gt, GeはASTでは左辺値と右辺値を反転させたLt, Leとして形成される
        _ => {}
    }
    instruction_vec.push(format!("    push rax"));
}

// astからアセンブラを出力する
// 渡されるastはrooがNoneか, 正しいASTである
pub fn compile_ast(ast: &mut AST) -> Vec<String> {
    let mut instruction_vec: Vec<String> = vec![];
    match ast.root.take() {
        Some(top_node) => {
            compile_node(*top_node, &mut instruction_vec, &mut ast.raw_text);
        }
        None => {}
    }
    instruction_vec
}
