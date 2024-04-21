
static GLOBAL_CONST_AVOGRADO : f32 = 6.02; 

pub fn declarando_variaveis() {

    let variavel_contador = 123;

    println!("Contador = {}", variavel_contador);
}

pub fn declarando_constantes() {
    const CONSTANTE_PI : f32 = 3.14;

    println!("Constante PI = {} ", CONSTANTE_PI);
}

pub fn declarando_constantes_globais() {
    println!("Numero de Avogrado = {} ", GLOBAL_CONST_AVOGRADO);
}

pub fn definindo_escopos() {
    println!("Escopos sao blocos de codigo delimitados por '{{' e '}}', quando a execucao do bloco termina, todos os recursos definidos nele, sao desalocados.");
    
    let mut variavel_a : i32 = 123;
    println!("variavel_a, mutavel, definida no escopo da funcao, fora do bloco anonimo = {}", variavel_a);

    variavel_a = {
        let variavel_a : i32 = 456;
        println!("variavel_a definida no escopo do bloco anonimo da funcao, dentro do bloco anonimo = {}", variavel_a);    
        variavel_a
    };

    println!("variavel_a, apos receber o retorno do bloco anonimo = {}", variavel_a);

    variavel_a = variavel_a + 1;

    println!("variavel_a, apos receber adicao de 1 = {}", variavel_a);

}

pub fn definindo_funcao() {
    println!("\nDefinindo uma funcao em Rust: ");
    println!("\n   fn nome_da_funcao ( param1 : tipo, param2 : tipo, ... paramN : tipo) -> tipo do retorno '{{' e '}}' ");
    println!("\n  Ex.: ");
    println!("\n       fn adicao(a : i32, b : i32) -> i32 '{{' a + b '}}' ");
    println!("\n   Ou  fn adicao(a : i32, b : i32) -> i32 '{{' return a + b; '}}' ");
 
    print!("\n");
    println!("Resultado da funcao adicao(2, 2) = {}", adicao(2, 2));

    print!("\n");
    equacao_2_grau(2.0, 3.0, -5.0);
}

fn adicao(a : i32, b : i32) -> i32 {
    a + b
}

pub fn equacao_2_grau(a: f64, b : f64, c : f64) {

    let resultado_delta = delta(a, b, c);
    let resultado_raiz_de_delta = f64::sqrt(resultado_delta);

    let resultado_x_1 = (-b + resultado_raiz_de_delta) / ( 2.0 * a );
    let resultado_x_2 = (-b - resultado_raiz_de_delta) / ( 2.0 * a ); 

    print!("\n");
    println!("Resultado da funcao equacao_2_grau(2.0, 3.0, -5.0) ");
    print!("\n");
    println!("Resolvendo equacao 2 grau ({}x^2 + {}x + {} = 0)", a, b, c);

    print!("\n");
    println!("Resultado de Delta = b^2 - 4 * a * c = {}^2 - 4 * {} * {} = {}", b, a, c, resultado_delta);
    print!("\n");
    println!("Resultado da Raiz de delta = SQRT({}) = {}", resultado_delta, resultado_raiz_de_delta);
    print!("\n");
    println!("X´  = -{} + {} / 2*{} = {}",b, resultado_raiz_de_delta, a, resultado_x_1);
    print!("\n");
    println!("X´´ = -{} - {} / 2*{} = {}", b, resultado_raiz_de_delta, a, resultado_x_2);
    
    print!("\n");
    println!("Conjunto Solucao = [{}, {}]", resultado_x_1, resultado_x_2);
    print!("\n");
    print!("\n");

}

fn delta(a: f64, b : f64, c : f64) -> f64 {
    f64::powf(b, 2.0) - (4.0 * a * c)
}

// ax^2 + bx + c = 0

// delta = b^2 - 4*a*c

// X = -b +- raiz(delta) / 2*a

// X' = -b + raiz(delta) / 2*a
// X" = -b - raiz(delta) / 2*a

    // let mut sinal_b: char = '+';
    // let mut sinal_c: char = '+';

    // if b > 0.0 {
    //     sinal_b = '+';
    // }

    // if b < 0.0 {
    //     sinal_b = '-';
    // }

    // if b == 0.0 {
    //     sinal_b = ' ';
    // }

    // if c > 0.0 {
    //     sinal_c = '+';
    // }

    // if c < 0.0 {
    //     sinal_c = '-';
    // }

    // if c < 0.0  {
    //     println!("Resolvendo equacao 2 grau ({}x^2 + {}x {} = 0)", a, b, c);
    // }
    // if c > 0.0  {
    //     println!("Resolvendo equacao 2 grau ({}x^2 + {}x + {} = 0)", a, b, c);
    // }
    // if c == 0.0 {
    //    println!("Resolvendo equacao 2 grau ({}x^2 + {}x = 0)", a, b);
    // }

