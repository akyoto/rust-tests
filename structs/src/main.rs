struct Person<'a> {
	name: &'a str,
	age: u8,
}

fn main() {
	let person = Person{name: "Eduard Urbach", age: 28};
	println!("{}, {}", person.name, person.age);
}
