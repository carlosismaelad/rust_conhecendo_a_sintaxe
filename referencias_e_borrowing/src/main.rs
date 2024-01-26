fn main() {
    let s1 = String::from("texto");
    let tamanho = calcula_tamanho(&s1);
    println!("O tamanho de {} é {}", s1, tamanho);

    let mut exemplo = String::from("exemplo de");
    modifica(&mut exemplo);
    println!("{}", exemplo);
}
// Uma forma de como você poderia definir e usar uma função
// calcula_tamanho que recebe uma referência para um objeto como
// parâmetro, em vez de pegar este valor para si:
// O "&" são referencias e ele permite que você se refira a algum
// valor sem tomar ele para si.

// determinamos que a nossa fn deve receber uma referencia
// Damos a isso o nome de "borrowing" (empréstimo)
fn calcula_tamanho(s: &String) -> usize {
    s.len()
}

// alterando um elemento emprestado
// com referências mutáveis
// repassamos um elemento mutável como parâmetro da nossa fn
fn modifica(uma_string: &mut String) {
    uma_string.push_str(" mais um texto")
}
