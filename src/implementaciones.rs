pub trait Implementacion1 {
	fn conversion(&self) -> i64;
}

impl Implementacion1 for String {
	fn conversion(&self) -> i64 {
		let variable1: i64 = match self.parse() {
			Ok(d) => d,
			Err(_e) => {
				eprintln!("Error al convertir de String a su valor en; i64");
				0
			},
		};
		
		variable1
	}
}
