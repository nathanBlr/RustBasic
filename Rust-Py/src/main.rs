
fn add(x: i32, y: i32)-> i32{
    x + y
}
fn main() {
 //variavel imutavel
 let x = 1;
 //variavel mutavel
 let mut y = 6;
 y = 7;
 //Tipos de dados
 let integer: i32 = 42;
 let floating_pint: f64 = 3.14;
 let boolean: bool = true;
 let character: char = 'a';
 //Outros tipos de dados
 let array: [i32; 3] = [1, 2, 3];
 let tuple: (i32, f64, char) = (10, 5.3, 'a'); 

 //Controle de dados
 let number = 42;

 if number > 0 {
    println!("positive");
 } else if {
    println!("negative");
 } else {
    println!("zero!");
 }

 let mut counter = 0;
 while counter < 5 {
    println!("counter: {}", counter);
    counter += 1;
 }

 for i in 0..5 {
    println!("interation: {}", i);
 }

 match  number {
    0 => println!("zero"),
    _ if number > 0 => println!("positive"),
    _ => println!("negative"),
 }

 let result = add(1, 2);
 println!("result: {}", result);


 //Dominio 
 let s1 = String::from("hello");
 let s2 = s1; //s1 deixa de existir com valor e agora s2 é o novo s1
 println!("{}, world!", s2);
}
