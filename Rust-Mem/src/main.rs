//Emprestar dados da varivael sem a utilizar
fn calculate_lenght(s: String) -> usize{
    s.len()
}
fn add_one( x: &mut i32){
    *x += 1; // Usamos `*` para desreferenciar o ponteiro e modificar o valor
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i]; // Retorna uma fatia (slice) que vai do início até o índice `i`
        }
    }
    &s[..] // Se não encontrar um espaço, retorna a string inteira
}

fn main() {
    let s = String::from("Hello");
    let len = calculate_lenght(&s);//& para pegar emprestado a referencia
    println!("Length of '{}' is {}.", s, len);
    let mut num = 5;
    add_one(&mut num); // Passamos uma referência mutável para `num`
    println!("Modified number: {}", num);
    let h = String::from("hello world");
    let first = first_word(&h);
    println!("First word: {}", first);
}
