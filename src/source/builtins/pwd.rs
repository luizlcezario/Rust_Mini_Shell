
pub fn built_pwd() {
	let path = std::env::current_dir().unwrap();
	println!("{}", path.display());
}