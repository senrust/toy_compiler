use super::tokenizer::OperationKind;
use crate::ast::{ASTNode, ASTNodeKind, FunctionAST, FuntionInfo, PrimaryNodeKind, AST};
use crate::error::{error_exit};

struct Instructions {
    vec: Vec<String>,
    end_count: usize,
    else_count: usize,
    begin_count: usize,
}

impl Instructions {
    fn new() -> Self {
        Instructions {
            vec: vec![],
            end_count: 0,
            else_count: 0,
            begin_count: 0,
        }
    }

    fn push(&mut self, instruction: String) {
        self.vec.push(instruction)
    }

    fn pop(&mut self) {
        self.vec.pop();
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

fn push_left_value_adress(mut node: ASTNode, instructions: &mut Instructions, dereference_pos: usize){
    let pointer_to_node = node.left.take().unwrap();
    if let  ASTNodeKind::Primary(PrimaryNodeKind::LocalVariable(offset)) = pointer_to_node.node_kind  {
        // 変数値
        push_local_variable_address(offset, instructions);
            
    } else if let ASTNodeKind::Dereference(text_pos) = pointer_to_node.node_kind {
        push_left_value_adress(*pointer_to_node, instructions, text_pos);
    } else {
        error_exit("left value cannot do operation", dereference_pos);
    }
    instructions.push(format!("    pop rax"));
    instructions.push(format!("    mov rax, [rax]"));
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
        } else if let ASTNodeKind::Dereference(text_pos) = left_node.node_kind {
            push_left_value_adress(*left_node, instructions, text_pos);
            compile_node(*right_node, instructions);
            instructions.push(format!("    pop rdi"));
            instructions.push(format!("    pop rax"));
            instructions.push(format!("    mov [rax], rdi"));
            instructions.push(format!("    push rdi"));
            return;
        } else {
            error_exit("left value is not correct", text_pos);
        }
    } else if let ASTNodeKind::Return = node.node_kind {
        let left_node = node.left.take().unwrap();
        compile_node(*left_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    mov rsp, rbp"));
        instructions.push(format!("    pop rbp"));
        instructions.push(format!("    ret"));
        return;
    } else if let ASTNodeKind::If = node.node_kind {
        let condition_node = node.left.take().unwrap();
        let instruction_node = node.right.take().unwrap();
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
        let instruction_node = node.right.take().unwrap();
        compile_node(*condition_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    cmp rax, 0"));
        instructions.push(format!("    je .Lelse{}", instructions.else_count));
        compile_node(*instruction_node, instructions);
        instructions.push(format!("    jmp .Lend{}", instructions.end_count));
        instructions.push(format!(".Lelse{}:", instructions.end_count));
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
        let instruction_node = node.right.take().unwrap();
        instructions.push(format!(".Lbegin{}:", instructions.begin_count));
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
        instructions.push(format!(".Lbegin{}:", instructions.begin_count));
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
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    jmp .Lbegin{}", instructions.begin_count));
        instructions.push(format!(".Lend{}:", instructions.end_count));
        instructions.end_count_up();
        instructions.begin_count_up();
        return;
    } else if let ASTNodeKind::MultStmt = node.node_kind {
        // 複文の場合はvecの中に各命令が含まれている
        let node_vec = node.vec.unwrap();
        if node_vec.len() != 0 {
            for node in node_vec {
                let node = node.unwrap();
                compile_node(*node, instructions);
                instructions.push(format!("    pop rax"));
            }
            // 文終了時にpop raxを実行するので,
            // 複文の最後のpop raxは取り除く
            instructions.pop();
        }
        return;
    } else if let ASTNodeKind::FunctionCall(function_name) = node.node_kind {
        let args_vec = node.vec.unwrap();
        for (arg_counnt, arg) in args_vec.into_iter().enumerate() {
            compile_node(*arg.unwrap(), instructions);
            if arg_counnt == 0 {
                instructions.push(format!("    pop rdi"));
            } else if arg_counnt == 1 {
                instructions.push(format!("    pop rsi"));
            } else if arg_counnt == 2 {
                instructions.push(format!("    pop rdx"));
            } else if arg_counnt == 3 {
                instructions.push(format!("    pop rcx"));
            } else if arg_counnt == 4 {
                instructions.push(format!("    pop r8"));
            } else if arg_counnt == 5 {
                instructions.push(format!("    pop r9"));
            }
        }
        instructions.push(format!("    call {}", function_name));
        instructions.push(format!("    push rax"));
        return;
    } else if let ASTNodeKind::Reference(text_pos) = node.node_kind {
        //  &の対象が変数であること
        // 渡されたastは正しいのでunwrapしても問題ない
        let variable_node = node.left.take().unwrap();
        if let ASTNodeKind::Primary(PrimaryNodeKind::LocalVariable(offset)) = variable_node.node_kind {
            push_local_variable_address(offset, instructions);
            return;
        } else {
            error_exit("& operarand must be for variable", text_pos);
        }
    }  else if let ASTNodeKind::Dereference(_text_pos) = node.node_kind {
        // ここの*は値であれば良い
        let variable_node = node.left.take().unwrap();
        compile_node(*variable_node, instructions);
        instructions.push(format!("    pop rax"));
        instructions.push(format!("    mov rax, [rax]"));
        instructions.push(format!("    push rax"));
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
// 渡されるastはrootがNoneか, 正しいASTである
fn compile_ast(mut ast: AST, instructions: &mut Instructions) {
    match ast.root.take() {
        Some(top_node) => {
            compile_node(*top_node, instructions);
        }
        None => {}
    }
}

fn compile_function_prologue(function_info: FuntionInfo, instructions: &mut Instructions) {
    instructions.push(format!(""));
    instructions.push(format!("{}:", function_info.function_name));
    instructions.push(format!("    push rbp"));
    instructions.push(format!("    mov rbp, rsp"));
    // AST側で7個以上の引数は拒否している
    for arg_index in 0..function_info.args_count {
        if arg_index == 0 {
            instructions.push(format!("    mov [rbp - 8], rdi"));
        } else if arg_index == 1 {
            instructions.push(format!("    mov [rbp - 16], rsi"));
        } else if arg_index == 2 {
            instructions.push(format!("    mov [rbp - 24], rdx"));
        } else if arg_index == 3 {
            instructions.push(format!("    mov [rbp - 32], rcx"));
        } else if arg_index == 4 {
            instructions.push(format!("    mov [rbp - 40], r8"));
        } else if arg_index == 5 {
            instructions.push(format!("    mov [rbp - 48], r9"));
        }
    }

    // スタックサイズは引数なので, 引数分引いた分スタックを下げる
    let local_variable_size = function_info.local_stack_size;

    if local_variable_size % 16 == 0 && local_variable_size != 0 {
        instructions.push(format!("    sub rsp, {}", local_variable_size));
    } else if local_variable_size % 16 != 0 && local_variable_size != 0 {
        instructions.push(format!("    sub rsp, {}", local_variable_size + 8));
    }
}

fn compile_function_epilogue(instructions: &mut Instructions) {
    instructions.push(format!("    pop rax"));
    instructions.push(format!("    mov rsp, rbp"));
    instructions.push(format!("    pop rbp"));
    instructions.push(format!("    ret"));
}

// function_astからアセンブラを出力する
pub fn compile_function_ast(function_ast: FunctionAST) -> Vec<String> {
    let mut instructions = Instructions::new();
    compile_function_prologue(function_ast.function_info, &mut instructions);
    compile_ast(function_ast.function_ast, &mut instructions);
    compile_function_epilogue(&mut instructions);
    instructions.vec
}
