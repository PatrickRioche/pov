// pov V0.0					Angers, le 09/01/2022
//
//
//
//
//
#! [warn(dead_code)]

use std::io;
use std::process;


fn main() {

    //
    //  Initialisation de variable
    //
	let mut compteur:i32 = 0;
	let mut sens = String::from(">");
    
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
			if n == '>' { sens = String::from(">") }
			if n == '<' { sens = String::from("<") }
			if n == 'z' { compteur = 0; }
			if n == 'm' { compteur = 5000;}
        }

		//
		//	En fonction du sens de rotation
		//

		if sens == ">".to_string() { compteur = compteur + 1;}
		if sens == "<".to_string() { compteur = compteur - 1;}

		if compteur < 1 { compteur = 1;}
		if compteur > 4999 { compteur = 5000;}

        println!("{}:{:4}", sens, compteur);

    }
}