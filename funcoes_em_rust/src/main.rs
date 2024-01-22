fn dobro(num: i32) -> i32 {
    return 2 * num;
}

fn maior_menor_que(a: i32, b: i32) -> i32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

fn alguma_fn(a: f32, b: f32) -> f32 {
    println!("Essa função devolve um float. O Resultado é: ");
    a / b
    // usamos uma "Expressão de retorno implícito" onde,
    // a última expressão repassada sem ";" é entendida
    // como o retorno da função
}

fn main() {
    println!("O dobro do número 5 é {}.", dobro(5));
    println!("O maior número entre 5 e 9 é {}.", maior_menor_que(5, 9));
    println!("{:.2}", alguma_fn(12.2, 2.2));
}
