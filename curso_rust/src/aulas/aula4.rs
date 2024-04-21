
pub fn conceito_ownership() {

    println!("Gerenciamento de memoria em Rust - Conceito de OwnerShip - Exemplo usando String::from(<Uma String>) (<Uma String> é alocada na Heap e um ponteiro é passado) ");
    println!("Valores alocados na Heap só pode ter um DONO. ");
    println!("A String não é copiada, não são geradas cópias da String na Heap. Ao invés disso são geradas cópias dos ponteiros alocados na Stack. ");

    println!("\n");

    println!("let uma_string: String = String::from(\"Marcio\");");
    println!(" ");
    println!("A variavel uma_string está recebendo um ponteiro para a area na Heap onde está a String \"Marcio\" ");
    println!("A variavel uma_string tem o Ownership do valor que está na Heap, onde está a String \"Marcio\" ");

    println!("\nAgora a função rouba(uma_string) recebe o Ownership de uma_string, com isso, quando a função terminar o valor na Heap será desalocado. ");
    println!("Ao tentar usar uma_string, ocorrerá um erro.");

    println!(" println!(\" {{}}\", uma_string); - nesse instante ocorrerá um erro.");

    println!("\nPara corrigir isso, a função deve retornar o valor de param:");
    println!("\n    fn rouba(param : String) -> String {{ ");
    println!("          println!(\"String = {{}}\", param); ");
    println!("          param   ");
    println!("     }}");

    println!(" ");
    println!(" E ");
    println!(" Ao invés de simplesmente chamar rouba(uma_string); ");
    println!(" Atribuir o retorno da função rouba() para uma outra variavel. ");
    println!("    let outra_string = rouba(uma_string); ");
    println!("    println!(\"{{}}\", outra_string); " );

    println!("\n");

    let uma_string: String = String::from("Marcio");
    let outra_string: String = rouba(uma_string);
    println!("Valor de outra_string = {}", outra_string);

    print!(" ");
    println!("===============================================");
}

fn rouba(param : String) -> String {
    println!("String Roubada = {}", param);
    param
}

pub fn conceito_referencia_borrowing() {

    println!("Gerenciamento de memoria em Rust - Conceito de Borrowing e Referencia - Exemplo usando String::from(<Uma String>) (<Uma String> é alocada na Heap e um ponteiro é passado) ");
    println!(" A String não é copiada, não são geradas cópias da String na Heap. Ao invés disso são geradas cópias dos ponteiros alocados na Stack. ");

    println!("\n");

    println!("let mut uma_string: String = String::from(\"Marcio\");");
    println!(" ");
    println!("A variavel uma_string está recebendo um ponteiro para a area na Heap onde está a String \"Marcio\" ");
    println!("A variavel uma_string tem o Ownership do valor que está na Heap, onde está a String \"Marcio\" ");

    println!("\nAgora a função empresta(&mut uma_string) recebe uma refrencia mutavel de uma_string (uma_string é emprestada para a função), com isso,");
    println!("quando a função terminar o valor na Heap não será desalocado e o Ownership voltará para uma_string. ");

    println!("\nAo passar uma referencia mutavel para a função, torna-se possivel a alteração da string emprestada:");
    println!("\n    fn empresta(param : &mut String) {{ ");
    println!("          param.push_str(\" Nielsen Baptista\"); ");
    println!("          println!(\"String Emprestada = {{}}\", param); ");
    println!("     }}");

    println!(" ");
    println!(" uma_string foi emprestada para a função empresta(), quando a função termina, o Ownership é devolvido para uma_string.");
    println!("Como a referencia passada é mutavel (&mut) e a variavel, também foi declarada como mutavel (let mut), foi possivel alterar o valor da String.");
    println!("    empresta(&mut uma_string); ");
    println!("    println!(\"Valor de uma_string = {{}}\", uma_string); " );

    println!("\n");

    let mut uma_string: String = String::from("Marcio");

    empresta(&mut uma_string);

    println!("Valor de uma_string = {}", uma_string);

    print!(" ");
    println!("===============================================");
}

fn empresta(param: &mut String) {
    param.push_str(" Nielsen Baptista");
    println!("String Emprestada = {}", param);
}

pub fn conceito_pattern_matching() {
    print!("\n");
    println!("Conceito de Pattern Matching \n");

    for x in 1..=20 {
        println!("{} : {}", x, match x {
            1  => "Pouco",
            2 | 3 => "Aumentando",
            4..=10 => "Aumentou",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _  => "Muito"
        });
    }
    print!("\n");
    println!("===============================================");
}

pub fn tratamento_erros_panic() {

    // panic!("Ciente Carvalho, já avisei que vai dar merda isso!");
}

pub fn tratamento_erros_sucesso() {
    match resultado_ok() {
        Ok(s) => println!("Processamento executado com sucesso - {}", s),
        Err(codigo_erro) => println!("Processamento executado com Erro - {}", codigo_erro)
    } 
}

pub fn tratamento_erros_erro() {
    match resultado_nok() {
        Ok(s) => println!("Processamento executado com sucesso - {}", s),
        Err(codigo_erro) => println!("Processamento executado com Erro - {}", codigo_erro)
    } 
}

fn resultado_ok() -> Result<String, u8> {
    Ok(String::from("Sucesso"))
}

fn resultado_nok() -> Result<String, u8> {
    Err(042)
}