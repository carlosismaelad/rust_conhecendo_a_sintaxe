// importando a biblioteca de input/output
use std::io;

// definindo a nossa função de conversão para int de 32bits
fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number1 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Erro ao ler numero 1.");

    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Erro ao ler numero 2.");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("O número {} é maior que o número {}", number1, number2);
    } else {
        println!("O número {} é menor ou igual a {}", number1, number2);
    }
}
