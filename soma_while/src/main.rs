use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut sum = 0;
    println!("Insira os números a serem somados: ");
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Erro ao ler valores de entrada");
    let mut int_input_value = convert_to_int(&input_value);

    while int_input_value != 0 {
        let r = int_input_value % 10;
        sum = sum + r;
        int_input_value = int_input_value / 10;
    }
    println!("O valor da soma dos dígitos é {}", sum);
}
