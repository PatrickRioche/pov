// pov V0.0					Angers, le 09/01/2022
//
//
//
//
//#! [warn(dead_code)]

use std::io;
use std::process;

fn main() {

    //
    //  Initialisation de variable
    //
	let mut compteur:i8 = 1;
    
    println!("POV:");
    loop {

		let mut input = String::new();

		match io::stdin().read_line(&mut input) {
			Ok(_n) => {
				//println!("{} bytes read", n);
				//println!("input ={}", input);
			}
			Err(error) => println!("error: {}", error),
		};

   		//
		//	Test si le caractere q a ete saisir au clavier
		//
		for n in input.chars() {
            //println!("{}",n);
            if n == 'q' { process::exit(0); }
        }
        println!("compteur : {}", compteur);
		compteur = compteur + 1;

    }
}
