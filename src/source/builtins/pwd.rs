
pub fn pwd() {
	let path = std::env::current_dir().unwrap();
	println!("{}", path.display());
}