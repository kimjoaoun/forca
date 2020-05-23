use rand::Rng;
use std::{io, thread, time};

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
        result.len()
    );
}

fn game_playing(result: &str) {
    // let mut tentativas: i32 = 6; // numero total de tentativas, valor fixo.
    let mut input = String::new();
    
    //

    println!("> {} \n > Por favor, insira seu chute", 
    "_ ".repeat(result.len())); // conta a quantidade de espaços e printa eles.


    io::stdin()
        .read_line(&mut input)
        .expect("ERR: Erm... Acho que não entendi direito.");

    let input = input.trim();

    println!("> resposta: óculos");    // DEBUGGING LINE
    

    if &input.to_string() == &result.to_string() {
        println!("> Parabéns, você acertou essa de primeira! A resposta era: {}.", 
        &input);

       // break; // descomentar o break quando colocar dentro do loop.
    } else {
        break_checkeq(&input.to_string(), &result.to_string());
    }


    fn break_checkeq(process: &String, result: &str) -> String {
        // Checa a similaridade entre palavras, e retorna o resultado.
      
        let mut compl = " _".repeat(result.len());
       
        // TODO: daqui para baixo, nada foi testado.
        if result.contains(&process.to_string()){
            println!("> Você acertou! '{}' faz parte da resposta.", &process);
            

            // temos um problema aqui. precisamos encontrar o index da letra que desejamos substituir.
             let loc_guess = &result.chars().position(|c| c == &process.to_string()).unwrap();

            

            for i in loc_guess {
               
                compl.replace_range(i+1..&(i+2), &process);
            }

            println!(">{}", &compl);
            compl

        } else {
            compl
        }
    }


    // println!("> {} e {}", input.trim(), tentativas);  
}
