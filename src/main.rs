/* Librairie input/output venant de la librairie standard */
use std::io;
use rand::Rng;
/* Ordering is a type which is an enumeration with 3 variants: Less, Greater and Equal */
use std::cmp::Ordering;
fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number} !");

    loop {
        println!("Please input your guess : ");
        /* String type is provided by the standard library */
        /* new() est une fonction qui instancie un nouvel objet/type String */
        /* "::" signifie que new() est un trait implémenté sur le type String. Ceci diffère du simple . pour une fonction sur un objet */
        let mut guess = String::new(); 
        /* read_line retourne une valeur Result qui est une énumération possédant 2 variants: Ok et Err */
        /* absence d'expect entraîne warning. Toutefois quand valeur Result, il est propre de mettree .expect() après.*/
        /* Si le variant renvoyé est Err alors ça affichera notre message et le programme coupera. */
        /* read_line est satisfait quand l'utilisateur saisit Entrée et ajoute donc \n dans la String */
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {guess}");
        /* Convertis guess de String --> Number afin de pouvoir le comparer à secret_number dans le match */
        /* Shadowing de guess */
        /* la fonction trim() retire tous les espaces et \n */
        /* parse retourne un type Result:Ok ou Err qui est donc traité avec expect */
        /* let guess: u32 = guess.trim().parse().expect("Please type a number!"); */

        /* Empêche que le programme crash par rapport à la ligne 30 et nous permet de continuer à jouer */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The match expression ends after the first successfull match
        match guess.cmp (&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You won !");
                break;
            }
        }
    }
}