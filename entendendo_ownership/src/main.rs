fn main() {
    let mut s = String::from("Olá");
    s.push_str(", mundo!");

    // deep copy usando método .clone()
    // Quando vemos uma chamada para clone, sabemos que algum código
    // arbitrário está sendo executado, e que este código talvez seja custoso.
    // É um indicador visual de que algo diferente está acontecendo.
    let s2 = s.clone();
    println!("s = {}, s2 = {}", s, s2);

    // o exemplo abaixo contradiz o que acabamos de ver
    let x = 5;
    let y = x;
    print!("x = {}, y = {}", x, y);
    // O motivo é que tipos como números inteiros têm um tamanho conhecido em
    // tempo de compilação e são armazenados inteiramente na pilha, e por isso,
    // cópias desses valores são rápidas de se fazer. Isso significa que não há
    // razão para impedir x de ser válido após criarmos a variável y.
}
