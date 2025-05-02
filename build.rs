fn main() {
	println!("cargo::rustc-link-search=asm/build");

	println!("cargo::rustc-link-lib=static=neon");
}