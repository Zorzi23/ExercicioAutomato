fn main() {

    let transicoes = [
        [(1, 'a'), (1, 'a'), (1, 'b')],
        [(1, 'a'), (1, 'b'), (1, 'b')],
        [(2, 'a'), (2, 'a'), (2, 'b')],
    ];

    let estado_inicial = 0;
    let estados_de_aceitacao = [2];

    let entrada = "abba";

    // Execução do autômato
    let mut estado_atual = estado_inicial;
    for c in entrada.chars() {
        let simbolo_entrada = match c {
            'a' => 0,
            'b' => 1,
            _ => panic!("Símbolo inválido: {}", c),
        };

        estado_atual = transicoes[estado_atual][simbolo_entrada].0;
    }

    let foi_aceita = estados_de_aceitacao.contains(&estado_atual);

    if foi_aceita {
        println!("A entrada {} foi aceita.", entrada);
    } else {
        println!("A entrada {} foi rejeitada.", entrada);
    }
}
