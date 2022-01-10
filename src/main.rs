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
const MOTORSLEEP:u32 = 1000;



fn clock(lookup:[String;8]) {
	let motorsleep = time::Duration::from_millis(MOTORSLEEP.into());
	for av in &lookup {
		//println!("{}",av);
		setoutput(av.to_string());
		thread::sleep(motorsleep);
	}
}

fn anticlock(lookup:[String;8]) {
	let motorsleep = time::Duration::from_millis(MOTORSLEEP.into());
	for ar in &lookup {
		//println!("{}",ar);
		setoutput(ar.to_string());
		thread::sleep(motorsleep);
	}
}

fn setoutput(phase:String) {
	println!("setoutput:");
	println!("{}",phase.to_string());
}

fn main() {

    //
    //  Initialisation de variable
    //
	let mut compteur:i32 = 0;
	let mut sens = String::from(">");

	let lookupav:[String;8] = [
		"1000".to_string(),
		"1100".to_string(),
		"0100".to_string(),
		"0110".to_string(),
		"0010".to_string(),
		"0011".to_string(),
		"0001".to_string(),
		"1001".to_string()];
	let lookupar:[String;8] = [
		"1001".to_string(),
		"0001".to_string(),
		"0011".to_string(),
		"0010".to_string(),
		"0110".to_string(),
		"0100".to_string(),
		"1100".to_string(),
		"1000".to_string()];
	 

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