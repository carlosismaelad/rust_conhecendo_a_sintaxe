fn main() {
    let s1 = String::from("texto");
    let tamanho = calcula_tamanho(&s1);
    println!("O tamanho de {} é {}", s1, tamanho)
}

// Uma forma de como você poderia definir e usar uma função
// calcula_tamanho que recebe uma referência para um objeto como
// parâmetro, em vez de pegar este valor para si:
// O "&" são referencias e ele permite que você se refira a algum
// valor sem tomar ele para si.
fn calcula_tamanho(s: &String) -> usize {
    s.len()
}
