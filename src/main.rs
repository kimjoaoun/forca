use rand::Rng;
use std::{io, thread, time};

// mod possibilities;

fn main() {
    welcome_message(your_name());

    let resposta = sorted();

    game_f(resposta.0);

    game_playing(resposta.0);
}

fn welcome_message(name: String) {
    // primeiras mensagens a serem exibidas ao usuario
    // também pede que este apresente-se
    // as mensagens tem "sleeps" para dar maior naturalidade

    thread::sleep(time::Duration::from_secs(1));
    println!("Ah, seu nome é {}? Belo nome.", name);
    println!("Então, {}, vamos jogar forca? Já deve conhecer essa brincadeira, é a mesma que jogava quando estava na escola.", name);

    thread::sleep(time::Duration::from_secs(3));
    println!("As instruções são simples, você tem 6 chances de acertar a palavra. Escreva a letra (ou a palavra inteira) e a resposta irá se completando");

    thread::sleep(time::Duration::from_secs(2));
    println!("A qualquer momento você pode pedir ajuda com o comando: !ajuda.");
}

fn your_name() -> String {
    println!("Olá, vejo que é sua primeira vez por aqui, qual seu nome?");

    let mut nome = String::new();

    io::stdin()
        .read_line(&mut nome)
        .expect("Erm... Acho que não entendi direito.");

    nome.trim().to_string()
}

//fn start() {}

fn sorted() -> (&'static str, usize) {
    // sorteia um valor da tuple POSS, e retorna um array com a palavra sorteada e o tamanho da palavra.
    let poss = [
        "óculos",
        "telegrama",
        "pacote",
        "ferrugem",
        "editor",
        "casa",
        "celular",
    ];

    let index = rand::thread_rng().gen_range(0, poss.len());

    (poss[index], poss[index].len())
}

fn game_f(result: &str) {
    println!("\n Um segundo, irei encontrar uma palavra para brincarmos...");
    thread::sleep(time::Duration::from_secs(3));

    println!(
        "Ok, achei a palavra perfeita para nosso jogo, ela tem: {} letras. \n Agora é sua vez, insira seus chutes!",
        result.len()
    );
}

fn game_playing(result: &str) {
    println!("{}", "_ ".repeat(result.len())); // conta a quantidade de espaços e printa eles.

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erm... Acho que não entendi direito.");

    input.trim().to_string();
}
