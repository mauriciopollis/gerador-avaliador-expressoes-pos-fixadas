use std::io;

struct Stack<T> {
    size: i32,
    data: Vec<T>,
}

fn new_stack<T>() -> Stack<T> {
    Stack {
        size: 0,
        data: Vec::new()
    }
}

fn push<T>(s: &mut Stack<T>, d: T) {
    s.data.push(d);
    s.size += 1;
}

fn pop<T>(s: &mut Stack<T>) {
    if s.size > 0 {
        s.size -= 1;
        s.data.pop();
    }
}

fn len<T>(s: &Stack<T>) -> i32 {
    return s.size;
}

fn top<T>(s: &Stack<T>) -> &T {
    return s.data.last().expect("Tentou acessar topo de pilha vazia");
}

fn gera_expressao_pos_fixada(v: &Vec<String>) -> String {
    let mut exp_pf: String = String::new();
    let mut pilha: Stack<String> = new_stack();

    for elem in v {
        if eh_operando(elem) {
            exp_pf.push_str(elem);
            exp_pf.push(' ');
        } else if elem == "(" {
            push(&mut pilha, elem.to_string());
        } else if elem == ")" {
            while len(&pilha) > 0 && top(&pilha) != "(" {
                // adicionar elemento a exp_pf
                exp_pf.push_str(&top(&pilha));
                exp_pf.push(' ');
                // dar pop desse elemento da pilha
                pop(&mut pilha);
            }
            // dar pop no parêntesis (
            pop(&mut pilha);
        } else { // elem é operador
            if len(&pilha) == 0 {
                push(&mut pilha, elem.to_string());
            } else {
                while len(&mut pilha) > 0 && prioridade(elem) <= prioridade(top(&pilha)) {
                    exp_pf.push_str(&top(&pilha));
                    exp_pf.push(' ');
                    pop(&mut pilha);
                }
                push(&mut pilha, elem.to_string());
            }
        }
    }

    while len(&pilha) > 0 {
        exp_pf.push_str(&top(&pilha));
        exp_pf.push(' ');
        pop(&mut pilha);
    }
    return exp_pf;
}

fn gera_vetor(s: &String) -> Vec<String> {
    let mut resp: Vec<String> = Vec::new();
    let mut token: String = String::new();

    for c in s.chars() {
        if c == ' ' {
            if !token.is_empty() {
                // adicionar o conteúdo do token no vetor resp
                resp.push(token.clone());
                // resetar o valor do token
                token.clear();
            }
        } else {
            if eh_operador(&c.to_string()) {
                if token.is_empty() {
                    resp.push(c.to_string());
                } else {
                    // colocar o conteúdo do token e depois o operador (c)
                    resp.push(token.clone());
                    token.clear();
                    resp.push(c.to_string());
                }
            } else {
                // não é operador -> é parêntesis ou letra
                // se for parentesis, adiciona a resp
                if c == '(' || c == ')' {
                    if !token.is_empty() {
                        resp.push(token.clone());
                        token.clear();
                        resp.push(c.to_string());
                    } else {
                        resp.push(c.to_string());
                    }
                } else {
                    token.push(c);
                }
                // se for letra, adiciona ao token
            }
        }
    }
    
    if !token.is_empty() {
        // adicionar o token restante a resp
        resp.push(token);
    }
    return resp;
}

fn prioridade(op: &String) -> i32 {
    if op == "+" || op == "-" {
        return 1;
    } else if op == "*" || op == "/" {
        return 2;
    } else if op == "^" {
        return 3;
    }
    return -1;
}

fn eh_operador(c: &String) -> bool {
    return c == "+" || c == "-" || c == "*" || c == "/" || c == "^";
}

fn eh_operando(c: &String) -> bool {
    return !eh_operador(c) && c != "(" && c != ")";
}

fn calcula_operacao(op1: i32, op2: i32, operacao: &String) -> i32 {
    if operacao == "+" {
        return op1 + op2;
    } else if operacao == "-" {
        return op1 - op2;
    } else if operacao == "*" {
        return op1 * op2;
    } else { // divisão
        return op1 / op2;
    } 
}

fn avalia_expressao_pf(exp_pf: &String) -> i32 {
    let vetor_tokens: Vec<String> = gera_vetor(&exp_pf);
    let mut pilha: Stack<String> = new_stack();

    for token in vetor_tokens {
        if eh_operador(&token) {
            let op2: i32 = top(&pilha).parse::<i32>().unwrap();
            pop(&mut pilha);
            let op1: i32 = top(&pilha).parse::<i32>().unwrap();
            pop(&mut pilha);
            let resultado: i32 = calcula_operacao(op1, op2, &token);
            push(&mut pilha, resultado.to_string());
        } else {
            push(&mut pilha, token);
        }
    }

    return top(&pilha).parse::<i32>().unwrap();
}

fn main() {

    let mut expressao: String = String::new();

    io::stdin().read_line(&mut expressao).expect("Falha na leitura");

    let expressao = expressao.trim().to_string();

    let vetor_caracteres : Vec<String> = gera_vetor(&expressao);

    let expressao_pf : String = gera_expressao_pos_fixada(&vetor_caracteres);

    let resultado: i32 = avalia_expressao_pf(&expressao_pf);

    println!("{expressao} -> {expressao_pf} = {resultado}");
}
