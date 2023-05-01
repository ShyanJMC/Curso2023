// Importamos el módulo "vec" con la libreria "Vec"
use std::vec::Vec;
// Importamos el módulo "process"
use std::process;
// Importamos el módulo "string" con la libreria "String"
use std::string::String;

// Importamos el módulo "io" con la libreria "Write",
// el "self" importa las funciones que no están en la libreria
// Write
use std::io::{self, Write};
//use std::io::Write;
//use std::io;

fn main(){
	println!("Primer ejercicio de una calculadora");
	let mut entrada: i64;
	let mut input: String = String::new();

	// Inicia el loop
	loop {
		print!("> ");
		// Toma como stdin a la función "stdout" y limpia su pantalla
		io::stdout().flush();
		// Stdin toma nuestro std input y devuelve un vector de bytes
		// c/u de esos bytes es un caracter que depende de la codificación
		// El método read_line toma ese vector y los codifica en UTF-8 y lo guarda
		// como un String en la variable mutable "input"
		io::stdin().read_line(&mut input);
		
		// El métood trim toma la variable de tipo string "input" y limpia de tabuladores, espacios,
		// nuevas líneas  (que es lo que conocemos como Enter) tanto al inicio como al final
		// de ese string y lo que devuelve ya limpio lo guardamos en la variable "input2"
		let input2: &str = input.trim();

		// Muestra la variable "input2" en forma "cruda" (sin formateo en la pantalla) tal y como está almacenada en memoria
		println!("{:?}",input2);

		// Exit no es string si no &str por que sabemos cuanto pesa; 5 bytes, los 4 bytes de la palabra y el 
		// caracter nulo (\0).
		// Se compara input con "exit" y si es verdadero, se ejecuta lo que hay entre llaves
		if input2 == "exit" {
			// Llamsmo a la función "exit" dentro de la libreria "process" para que termine el programa con un código cero.
			process::exit(0);
			
		// El método "contains" verifica que el string "input2" contenga el caracter o string que le pasamos 
		// como argumento; '+', '-', '*', '/'
		} else if input2.contains('+') {

			// Se toma los datos de la variable "input2" y se pasa al método "chars()"
			let buffer = input2
								// el método chars toma cada caracter y devuelve un vector de caracteres
								.chars()
								// el método map toma el vector de caracteres que retorno chars() e identifica
								// cada uno (de esos caracteres) con la letra "e" y lo pasa al método "to_string"
								// para convertirlo de un char a un String
								.map(|e| e.to_string() )
								// Collect toma cada string que le manda el método map y lo guarda en un vector de tipo
								// String
								.collect::<Vec<String>>();
								// Al final de esto, cada caracter del string "input2" queda convertido de char a string
								// cada uno separado en una position diferente dentro de un vector.

			// Inicializa una variable "i" que toma en cada iteracion un valor de la variable "buffer", de uno en uno
			for i in buffer {
				if i.chars().next().unwrap().is_alphabetic() {
					eprintln!("Solo se permiten números");
				}
			}
			
		} else if input2.contains('-') {
			println!("Resta detectada");
		} else if input2.contains('*') {
			println!("Mult detectada");
		} else if input2.contains('/') {
			println!("Div detectada");
		} else {
			// Muestra lo que pasamos como argumento hacia el stderr
			eprintln!("Operación no detectada");
			
		}
		

		// Acá el módulo "clear()" limpia la variable "input" de tipo String para que en el siguiente
		// bucle no se concatene lo anterior con los nuevos.
		input.clear();
	}
}
