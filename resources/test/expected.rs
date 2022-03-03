struct Hello {
    world: u32,
}

pub fn say_hello(hello: &Hello) {
    println!("HELLO: {}", hello.world);
}

pub fn say_goodbye() {
    println!("Goodbye!");
}
