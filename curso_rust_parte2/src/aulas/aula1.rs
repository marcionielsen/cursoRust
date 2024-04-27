

pub fn usando_arrays() {
    let notas = [10.0, 8.0, 9.0, 6.0];

    for nota in notas {
        println!("Nota = {}", nota);
    }
    print!("\n");
    println!("===============================================");
}

pub fn manipulando_tamanho_array() {

    println!("Usando a função len()\n");

    let notas = [10.0, 8.0, 9.0, 6.7, 7.0, 9.0, 9.7, 10.0];

    let mut media : f64 = 0.0;
    let mut tamanho: f64 = 0.0;

    for indice in 0..notas.len() {
        println!("Nota = {}", notas[indice]);
        media += notas[indice];
        tamanho += 1.0;
    }

    println!("\nA Média = {:.2}", (media/tamanho) );
    println!("\nA Média arredondada = {:.2}", f64::round_ties_even(media/tamanho) );

    print!("\n");
    println!("===============================================");

}

pub fn manipulando_matriz() {

    println!("Manipulando Matriz\n");

    let matriz : [[f64; 3]; 3] = [
        [10.0, 8.0, 9.0], 
        [6.7, 7.0, 9.0], 
        [9.7, 10.0, 11.0]
    ];

    for linha in matriz {
        print!("[");

        for item in linha {
            print!("Item = {:.1}, ", item);
        }

        print!("],\n");
    }

    print!("\n");
    println!("===============================================");
}