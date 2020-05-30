use rand::Rng;
use std::{io, thread, time};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Primeiro Projeto em Rust: Jogo da Forca \n\n\n");

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
    println!("> Ah, seu nome é {}? Belo nome.", name);
    println!("> Então, {}, vamos jogar forca? Já deve conhecer essa brincadeira, é a mesma que jogava quando estava na escola.", 
    name);

    thread::sleep(time::Duration::from_secs(3));
    println!("> As instruções são simples, você tem 6 chances de acertar a palavra. Escreva a letra (ou a palavra inteira) e a resposta irá se completando");

    thread::sleep(time::Duration::from_secs(2));
    println!("> A qualquer momento você pode pedir ajuda com o comando: !ajuda.");
}

fn your_name() -> String {
    println!("> Olá, vejo que é sua primeira vez por aqui, qual seu nome?");

    let mut nome = String::new();

    io::stdin()
        .read_line(&mut nome)
        .expect("Erm... Acho que não entendi direito.");

    nome.trim().to_string()
}

fn sorted() -> (&'static str, usize) {
    // sorteia um valor de poss, e retorna uma tuple com a palavra sorteada e o tamanho da palavra.
    let poss = [
        "oculos",
        //"telegrama",
        //"pacote",
        //"ferrugem",
        //"editor",
        //"casa",
        //"celular",
    ];

    let index = rand::thread_rng().gen_range(0, poss.len());

    (poss[index], poss[index].len())
}

fn game_f(result: &str) {
    println!("\n > Um segundo, irei encontrar uma palavra para brincarmos...");
    thread::sleep(time::Duration::from_secs(3));

    println!(
        "> Ok, achei a palavra perfeita para nosso jogo, ela tem: {} letras. \nAgora é sua vez, insira seus chutes!",
        result.graphemes(true).count()  
    );
}




fn game_playing(result: &str) -> () {
    // let mut tentativas: i32 = 6; // numero total de tentativas, valor fixo.
    let letters = UnicodeSegmentation::graphemes(result, true).collect::<Vec<&str>>();
    let out_vec: Vec<&str> = Vec::new();
    let tentativas: i32 = 6;

    solver(result, tentativas, letters, out_vec);

    fn solver(resultado: &str, tries:i32, compo: Vec<&str>, out_vec: Vec<&str>){
        
        if &tries == &0 {
            println!("Oops, infelizmente suas chances acabaram, não foi dessa vez.\nO jogo será terminado em 5 segundos.");
            thread::sleep(time::Duration::from_secs(5));
            panic!("Jogo terminado.");
        } else if &tries == &1 {
            println!("Ei, só lhe resta mais uma tentativa!");   
        }   
        
        let mut input = String::new();
        
        io::stdin()
        .read_line(&mut input)
        .expect("ERR: Erm... Acho que não entendi direito.");
        
        


        if compo.iter().any(|&i| i==input) {
            println!("Isso! '{}' é uma letra que faz parte da nossa palavra", &input);
            let mut out = out_vec.clone();
            out.push(&input);
            solver(resultado, tries, compo, out);

        } else {
            println!("Oops! Não foi dessa vez, lhe restam {} tentativas", &tries-1);

            solver(resultado, tries-1, compo, out_vec);

        }
        

    }
}
