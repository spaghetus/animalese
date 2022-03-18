fn main() {
	println!("cargo:rerun-if-changed=model.url");
	println!("cargo:rerun-if-changed=dist.url");
	println!("cargo:rerun-if-changed=build.rs");
	std::fs::write(
		"./model.pbmm",
		reqwest::blocking::get(std::fs::read_to_string("model.url").unwrap())
			.unwrap()
			.bytes()
			.unwrap(),
	)
	.unwrap();
	std::fs::write(
		"./libstt.zip",
		reqwest::blocking::get(std::fs::read_to_string("dist.url").unwrap())
			.unwrap()
			.bytes()
			.unwrap(),
	)
	.unwrap();
	std::process::Command::new("unzip")
		.arg("-o")
		.arg("libstt.zip")
		.arg("-d")
		.arg(".")
		.status()
		.unwrap();
	println!("cargo:rustc-link-search=.");
	// println!("cargo:rustc-link-lib=static=libstt.so");
	// println!("cargo:rustc-link-lib=static=libkenlm.so");
	// println!("cargo:rustc-link-lib=sox");
}
