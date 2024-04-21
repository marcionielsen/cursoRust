
pub fn controle_fluxo_if() {

    println!("Controlando do fluxo de execução do programa - Condicionais - 'if' ");

    let idade = 18;
    let eh_maior : bool = idade >= 18;
    let condicao = if eh_maior { "é maior de idade!" } else { "é menor de idade!" };

    println!(" ");
    println!("A pessoa, tem {} anos, logo {} ", idade, condicao);
    println!("===============================================");
}

pub fn controle_fluxo_match_statement() {

    println!(" ");
    println!("Controlando do fluxo de execução do programa - Condicionais - 'match' ");

    let lista = ["PHP", "Java", "Go", "Kotlin", "Python", "C", "C++", "C#", "Rust", "Assembly", "COBOL", "FORTRAN", "BASIC", "Delphi", "Clipper"]; 
    let mut linguagem : &str;
    let mut proposito :&str;

    print!(" ");
    println!("-----------------------------------------------");

    for elemento in lista {
        linguagem = elemento;

        proposito = verificar_proposito(linguagem);

        println!(" ");
        println!("O proposito de {} é {} ", linguagem, proposito);
    }

    println!("-----------------------------------------------");
    print!("\n");

    println!("===============================================");
}

fn verificar_proposito(linguagem : &str) -> &str {
    match linguagem {
        "PHP"                         => "Web",
        "Kotlin"                      => "Android",
        "Python"                      => "Data science",
        "Rust"                        => "substituir C/C++ e tornar as aplicações mais seguras!",
        "Java"  | "C#" | "Go"         => "Multiplas aplicações!",
        "COBOL" | "FORTRAN" | "BASIC" => "Manter aplicações Legadas!",        
        "C" | "C++"                   => "Implementar Kernel de Sistemas Operacionais",
        "Delphi"                      => "Aplicações Desktop!",        
        "Assembly"                    => "Implementar Sistemas Operacionais",        
                                    _ => "desconhecido!"
    }
}

pub fn controle_fluxo_while() {
    println!("\n");
    println!("Controlando do fluxo de execução do programa - Laços - 'while' ");

    let multiplicador : u8 = 5;
    let mut contador  : u8 = 0;

    println!(" ");
    println!("Imprimindo tabuada de {}", multiplicador);
    println!("===============================================");

    while contador < 10 {
        contador += 1;

        println!("{} X {} = {}", multiplicador, contador, (multiplicador * contador));
    }     

    println!("===============================================");
}

pub fn controle_fluxo_loop() {
    println!("\n");
    println!("Controlando do fluxo de execução do programa - Laços - 'loop' ");

    let multiplicador : u8 = 6;
    let mut contador  : u8 = 0;

    println!(" ");
    println!("Imprimindo tabuada de {}", multiplicador);
    println!("===============================================");

    loop  {
        contador += 1;

        println!("{} X {} = {}", multiplicador, contador, (multiplicador * contador));

        if contador == 10 {
            break;
        }
    }     

    println!("===============================================");
}

pub fn controle_fluxo_for() {
    println!("\n");
    println!("Controlando do fluxo de execução do programa - Laços - 'for' ");

    let multiplicador : u8 = 7;
    // let mut contador  : u8 = 0;

    println!(" ");
    println!("Imprimindo tabuada de {}", multiplicador);
    println!("===============================================");

    for contador in 1..=10 {
        println!("{} X {} = {}", multiplicador, contador, (multiplicador * contador));
    }     

    println!("===============================================");
}
