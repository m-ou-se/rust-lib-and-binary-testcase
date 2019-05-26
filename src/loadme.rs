#[no_mangle]
pub fn foo() {
	println!("Hello, world!");
	println!("Thread name: {:?}", std::thread::current().name());
}

#[allow(dead_code)]
fn main() {
	foo();
}
