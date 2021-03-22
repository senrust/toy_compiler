use super::tokenizer::OperationKind;
use crate::ast::{ASTNode, ASTNodeKind, ASTVec, PrimaryNodeKind, AST};
use crate::error::error_exit;

struct Instructions{
    vec: Vec<String>,
    end_count: usize,
    else_count: usize,
    begin_count: usize,
}

impl Instructions {
    fn new() -> Self {
        Instructions{
            vec: vec!(),
            end_count: 0,
            else_count: 0,
            begin_count: 0,
        }
    }

    fn push(&mut self, instruction: String) {
        self.vec.push(instruction)
    }

    fn end_count_up(&mut self) {
        self.end_count += 1;
    }

    fn else_count_up(&mut self) {
        self.else_count += 1;
    }

    fn begin_count_up(&mut self) {
        self.begin_count += 1;
    }
}

// ローカル変数のアドレスをスタックにpushする
fn push_local_variable_address(offset: usize, instructions: &mut Instructions) {
    instructions.push(format!("    mov rax, rbp"));
    instructions.push(format!("    sub rax, {}", offset));
    instructions.push(format!("    push rax"));
}

fn compile_node(mut node: ASTNode, instructions: &mut Instructions) {
    if let ASTNodeKind::Primary(PrimaryNodeKind::Number(num)) = node.node_kind {
        let instruction = format!("    push {}", num);
        instructions.push(instruction);
        return;
    } else if let ASTNodeKind::Primary(PrimaryNodeKind::LocalVariable(offset)) = node.node_kind {
        push_local_variable_address(offset, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    mov rax, [rax]"));
        instructions.push(format!("    push rax"));
        return;
    } else if let ASTNodeKind::Assign(text_pos) = node.node_kind {
        //  = の左辺値が変数であること
        // 渡されたastは正しいのでunwrapしても問題ない
        let left_node = node.left.take().unwrap();
        let right_node = node.right.take().unwrap();
        if let ASTNodeKind::Primary(PrimaryNodeKind::LocalVariable(offset)) = left_node.node_kind {
            push_local_variable_address(offset, instructions);
            compile_node(*right_node, instructions);
            instructions.push(format!("    pop rdi"));
            instructions.push(format!("    pop rax"));
            instructions.push(format!("    mov [rax], rdi"));
            instructions.push(format!("    push rdi"));
            return;
        } else {
            error_exit("left value is not correct", text_pos);
        }
    } else if let ASTNodeKind::Return = node.node_kind  {
        let left_node = node.left.take().unwrap();
        compile_node(*left_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    mov rsp, rbp"));
        instructions.push(format!("    pop rbp"));
        instructions.push(format!("    ret"));
        return;
    } else if let ASTNodeKind::If = node.node_kind {
        let condition_node = node.left.take().unwrap();
        let instruction_node = node.left.take().unwrap();
        compile_node(*condition_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    cmp rax, 0"));
        instructions.push(format!("    je .Lend{}", instructions.end_count));
        compile_node(*instruction_node, instructions);
        instructions.push(format!(".Lend{}:", instructions.end_count));
        instructions.end_count_up(); 
        return;
    } else if let ASTNodeKind::IfElse = node.node_kind {
        let condition_node = node.left.take().unwrap();
        let instruction_node = node.left.take().unwrap();
        compile_node(*condition_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    cmp rax, 0"));
        instructions.push(format!("    je .Lelse{}", instructions.else_count));
        compile_node(*instruction_node, instructions);
        instructions.push(format!("    jmp .Lend{}", instructions.end_count));
        instructions.push(format!(".Lelse{}", instructions.end_count));
        // elseが付属するifの場合, if(A) B else C
        // のCは node.vec[0]にある.
        // AST構築の段階でelse文のチェックをしているのでunwrapして良い
        let mut else_vec = node.vec.take().unwrap();
        let else_instruction_node = else_vec[0].take().unwrap();
        compile_node(*else_instruction_node, instructions);
        instructions.push(format!(".Lend{}:", instructions.end_count));
        instructions.end_count_up(); 
        instructions.else_count_up(); 
        return;
    } else if let ASTNodeKind::While = node.node_kind {
        let condition_node = node.left.take().unwrap();
        let instruction_node = node.left.take().unwrap();
        instructions.push(format!(".Lbegin{}", instructions.begin_count));
        compile_node(*condition_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    cmp rax, 0"));
        instructions.push(format!("    je .Lend{}", instructions.else_count));
        compile_node(*instruction_node, instructions);
        instructions.push(format!("    jmp .Lbegin{}", instructions.begin_count));
        instructions.push(format!(".Lend{}:", instructions.end_count));
        instructions.end_count_up(); 
        instructions.begin_count_up();
        return;
    } else if let ASTNodeKind::For = node.node_kind {
        let mut instruction_vec = node.vec.take().unwrap();
        let loop_instruction = node.left.take().unwrap();
        if let Some(initial_instruction) = instruction_vec[0].take() {
            compile_node(*initial_instruction, instructions);
        }
        instructions.push(format!(".Lbegin{}", instructions.begin_count));
        if let Some(judge_instruction) = instruction_vec[1].take() {
            compile_node(*judge_instruction, instructions);
        }
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    cmp rax, 0"));
        instructions.push(format!("    je .Lend{}", instructions.else_count));
        compile_node(*loop_instruction, instructions);
        if let Some(update_instruction) = instruction_vec[2].take() {
            compile_node(*update_instruction, instructions);
        }
        instructions.push(format!("    jmp .Lbegin{}", instructions.begin_count));
        instructions.push(format!(".Lend{}:", instructions.end_count));
        instructions.end_count_up(); 
        instructions.begin_count_up();
        return;
    } else if let ASTNodeKind::MultStmt = node.node_kind {
        // 複文の場合はvecの中に各命令が含まれている
        let node_vec = node.vec.unwrap();
        for node in node_vec {
            let node = node.unwrap();
            compile_node(*node, instructions);
            instructions.push(format!("    pop rax"));
        }
        return;
    }

    // 渡されたastは正しいのでunwrapしても問題ない
    let left_node = node.left.take().unwrap();
    let right_node = node.right.take().unwrap();
    compile_node(*left_node, instructions);
    compile_node(*right_node, instructions);

    instructions.push(format!("    pop rdi"));
    instructions.push(format!("    pop rax"));

    match node.node_kind {
        ASTNodeKind::Operation(OperationKind::Add) => {
            instructions.push(format!("    add rax, rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Sub) => {
            instructions.push(format!("    sub rax, rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Mul) => {
            instructions.push(format!("    imul rax, rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Div) => {
            instructions.push(format!("    cqo"));
            instructions.push(format!("    idiv rdi"));
        }
        ASTNodeKind::Operation(OperationKind::Eq) => {
            instructions.push(format!("    cmp rax, rdi"));
            instructions.push(format!("    sete al"));
            instructions.push(format!("    movzb rax, al"));
        }
        ASTNodeKind::Operation(OperationKind::Not) => {
            instructions.push(format!("    cmp rax, rdi"));
            instructions.push(format!("    setne al"));
            instructions.push(format!("    movzb rax, al"));
        }
        // Gt, GeはASTでは左辺値と右辺値を反転させたLt, Leとして形成される
        ASTNodeKind::Operation(OperationKind::Lt) => {
            instructions.push(format!("    cmp rax, rdi"));
            instructions.push(format!("    setl al"));
            instructions.push(format!("    movzb rax, al"));
        }
        ASTNodeKind::Operation(OperationKind::Le) => {
            instructions.push(format!("    cmp rax, rdi"));
            instructions.push(format!("    setle al"));
            instructions.push(format!("    movzb rax, al"));
        }
        // ASTNodeKind::Numberはここには来ない
        // Gt, GeはASTでは左辺値と右辺値を反転させたLt, Leとして形成される
        _ => {}
    }
    instructions.push(format!("    push rax"));
}

// astからアセンブラを出力する
// 渡されるastはrooがNoneか, 正しいASTである
fn compile_ast(mut ast: AST, instructions: &mut Instructions) {
    match ast.root.take() {
        Some(top_node) => {
            compile_node(*top_node, instructions);
        }
        None => {}
    }
}

// ast_vecからアセンブラを出力する
pub fn compile_astvec(ast_vec: ASTVec) -> Vec<String> {
    let mut instructions = Instructions::new();
    for ast in ast_vec.vec {
        compile_ast(ast, &mut instructions);
        instructions.push(format!("    pop rax"));
    }
    instructions.vec
}
