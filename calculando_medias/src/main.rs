use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    println!("Informe a quantidade de médias a serem inseridas: ");
    let mut qtde_medias = String::new();
    io::stdin()
        .read_line(&mut qtde_medias)
        .expect("Erro ao ler dados de entrada.");

    let mut soma_rec = 0;
    let mut i = 0;

    while convert_to_int(&qtde_medias) > i {
        println!("Informe a média do {}º aluno:", i + 1);
        let mut media_aluno = String::new();
        io::stdin()
            .read_line(&mut media_aluno)
            .expect("Erro ao ler média do aluno.");
        i += 1;

        // Qtde de alunos de recuperação
        if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6 {
            soma_rec += 1;
        }
    }
    println!("O total de alunos de recuperação é: {}", soma_rec)
}
