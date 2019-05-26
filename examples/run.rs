fn main() {
	println!("\n-> Loading and running foo() from the cdylib...\n");
	unsafe {
		let m = libc::dlopen("target/debug/libloadme.so\0".as_ptr() as _, libc::RTLD_NOW);
		let f = libc::dlsym(m, "foo\0".as_ptr() as _);
		let f: extern "C" fn() = std::mem::transmute(f);
		f();
	}
	println!("\n-> Loading and running foo() from the binary...\n");
	unsafe {
		let m = libc::dlopen("target/debug/loadme\0".as_ptr() as _, libc::RTLD_NOW);
		let f = libc::dlsym(m, "foo\0".as_ptr() as _);
		let f: extern "C" fn() = std::mem::transmute(f);
		f();
	}
}
