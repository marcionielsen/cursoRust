mod aulas;

use aulas::{aula1, aula2, aula3};

fn main() {

    println!(" ");
    println!("#########################################################");
    println!(" ");

    aula1::ola_mundo();

    println!(" ");
    println!("#########################################################");
    println!(" ");

    aula2::declarando_variaveis();
    aula2::declarando_constantes();
    aula2::declarando_constantes_globais();    
    aula2::definindo_escopos();    
    aula2::definindo_funcao();

    println!(" ");
    println!("#########################################################");
    println!(" ");

    aula3::controle_fluxo_if();

    aula3::controle_fluxo_match_statement();

    aula3::controle_fluxo_while();
    aula3::controle_fluxo_loop();
    aula3::controle_fluxo_for();

    println!(" ");
    println!("#########################################################");
    println!(" ");

}
