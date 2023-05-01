// Importamos el modulo "vec" con la libreria "Vec"
use std::vec::Vec;
// Importamos el modulo "process"
use std::process;
// Importamos el modulo "string" con la libreria "String"
use std::string::String;

// Importamos el modulo "io" con la libreria "Write",
// el "self" importa las funciones que no estan en la libreria
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
        // Toma como stdin a la funcion "stdout" y limpia su pantalla
        io::stdout().flush();        
        // Stdin toma nuestro std input (teclado) y devuelve un vector de bytes
        // c/u de esos bytes es un caracter que depende de la codificacion
        // El metodo read_line toma ese vector y los codifica en UTF-8 y lo guarda
        // como un String en la variable mutable "input"
     	io::stdin().read_line(&mut input);

     	// El metodo trim toma la variable de tipo string "input" y limpia de tabuladores, espacios,
     	// nuevas lineas (que es lo que conocemos como Enter) tanto al inicio como al final
     	// de ese string y lo que devuelve ya limpio lo guardamos en la variable "input2"
     	let input2: &str = input.trim();

     	// Muestra la variable en forma "cruda" (sin formateo en la pantalla) tal y como esta almacenada en memoria
     	println!("{:?}",input2);

        // Exit no es String si no &str porque sabemos cuanto pesa; 5 bytes, los 4 bytes de la palabra
        // y el caracter nulo (\0).
        // Se compara input con "exit" y si es verdadero, se ejecuta lo que hay entre llaves
		if input2 == "exit" {
	        // Llamamos a la funcion "exit" dentro de la libreria "process" para que termine el programa con un codigo cero	
			process::exit(0);

        // El metodo "contains" verifica que el String "input2" contenga el caracter o String que le pasamos
        // como argumento; '+','-','*','/'
		} else if input2.contains('+') {
		
            // Se toma los datos de la variable "input2" y se pasa al metodo "chars()"
            let buffer = input2
                                // El metodo chars() toma cada caracter y devuelve un vector de caracteres    
                                .chars()
                                // El metodo map toma el vector de caracteres que retorno chars() e identifica
                                // cada uno (de esos caracteres) con la letra "e" y lo pasa al metodo "to_string"
                                // para convertirlo de un char a un String
                                .map(|e| e.to_string() )
                                // Collect toma cada string que le manda el metodo map y lo guarda en un vector de tipo String
                                .collect::<Vec<String>>();
                                // Al final de esto, cada caracter del string "input2" queda convertido de char a string
                                // cada uno separado en una posicion diferente dentro de un vector.


            // Inicializa una variable "i" que toma en cada interaccion un valor de la variable "buffer", de uno en uno                                
            for i in buffer {
                // El metodo chars() toma el valor de "i" y lo convierte a un vector de chars.
                // Como cada valor que toma "i" es un solo valor (porque toma un solo valor de la variable buffer)
                // el vector de chars que devuelve el metodo es un vector de una sola posicion.
            	if i.chars()
            	    // El metodo next() avanza 1 posicion (que siempre empieza desde cero) y devuelve el char 
            	    // que esta en esa posicion 
            	    .next()
            	    // El metodo unwrap() detecta si lo que devolvio next() es un error y si lo es finaliza
            	    // el programa mostrando ese error
            	    .unwrap()
            	    // El metodo "is_alphabetic()" detecta si el caracter devuelto por next() es un
            	    // caracter alfabetico o no
            	    .is_alphabetic() {
            	        // Si lo anterior es verdadero, se muestra en el stderr el mensaje 
            		    eprintln!("Solo se permiten numeros")
            	}          
              }
		
		} else if input2.contains('-') {
			println!("Resta detectada");
		} else if input2.contains('*') {
			println!("Multiplicacion detectada");
		} else if input2.contains('/') {
			println!("Division detectada");
		} else {
		    // Muestra lo que pasamos como argumento hacia el stderr
			eprintln!("Operacion no detectada");
		
		}

		
        // Aca el modulo "clear()" limpia la variable "input" de tipo String para que en el siguiente
        // bucle no se concatene lo anterior con los nuevos.
		input.clear();
	}
}
