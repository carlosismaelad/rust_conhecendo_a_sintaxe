fn main() {
    let minha_string = &String::from("texto longo");

    // primera_palavra funciona com slices de String's
    let palavra = primeira_palavra(&minha_string[..]);

    let minha_string_literal = "texto longo";

    // primeira_palavra funciona com strings literais
    let palavra = primeira_palavra(&minha_string_literal[..]);

    // uma vez que strings literais são slicesde strings
    // isso também funciona, sem nem usar sintaxe de slices
    let palavra = primeira_palavra(minha_string_literal);
}

fn primeira_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
