// pov V0.0					Angers, le 09/01/2022
//
//
//
//
//
#! [warn(dead_code)]

use std::io;
use std::process;
use rppal::gpio::Gpio;
use std::{thread, time};

const MOTORPIN1:u8 = 24;
const _MOTORPIN2:u8 = 21;
const _MOTORPIN3:u8 = 19;
const _MOTORPIN4:u8 = 23;
const MOTORSLEEP:u32 = 100;



fn clock(lookup:[u8;8]) {
	let motorsleep = time::Duration::from_millis(MOTORSLEEP.into());
	for av in &lookup {
		println!("{}",av);
		thread::sleep(motorsleep);
	}
}

fn anticlock(lookup:[u8;8]) {
	let motorsleep = time::Duration::from_millis(MOTORSLEEP.into());
	for ar in &lookup {
		println!("{}",ar);
		thread::sleep(motorsleep);
	}
}

fn main() {

    //
    //  Initialisation de variable
    //
	let mut compteur:i32 = 0;
	let mut sens = String::from(">");

	let lookupav:[u8;8] = [8,12,4,6,2,3,1,9];
	let lookupar:[u8;8] = [9,1,3,2,6,4,12,8];
	
	//let now = time::Instant::now();

    println!("MP1:{}", MOTORPIN1);
	println!("clock");
	clock(lookupav);
	println!("anticlock");
	anticlock(lookupar);

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