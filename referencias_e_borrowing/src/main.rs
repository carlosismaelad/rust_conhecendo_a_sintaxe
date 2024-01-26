fn main() {
    let s1 = String::from("texto");
    let (s2, tamanho) = calcula_tamanho(s1);
    println!("O tamanho de {} é {}", s2, tamanho)
}

// Fazendo a função devolver o valor de uma variável repassado como
// parâmetro para que possamos usar ele em outro lugar
fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();
    (s, tamanho)
}
