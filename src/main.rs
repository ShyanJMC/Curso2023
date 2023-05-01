use std::vec::Vec;
use std::process;
use std::string::String;
use std::io;



fn main() {
    println!("Primer ejercicio de la Calculadora");
	let mut entrada: i64;
	let mut input: String = String::new();

	loop{
		
		println!("> ");

		io::stdout().flush();



	    io::stdin().read_line(&mut input);


		let input2 = input.trim();


		println!(":?",input2);


		if input == "exit".to_string() {


			process::exit(0);
		}else if input2.contains("+"){

		let buffer = input2
							.chars()
							.map(|e| e.to_string())
							.collect::<Vec<String>>();
		

		for i in buffer {
			if i.chars()
				.next()
				.unwrap()
				.is_alphabetic(){
					eprintln!("Solo se permiten numeros");
				}
		}

		
			println!("Suma detectada");
		}else if input2.contains("-"){
			println!("Resta detectada");
		}else if input2.contains("*"){
			println!("Multiplicacion detectada");
		}else if input2.contains("/"){
			println!("Division detectada");
		}else {
			eprintln!("Operacion no Detectada");
		}

		input.clear();



	}

    
}
