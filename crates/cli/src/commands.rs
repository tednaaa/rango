use std::env::current_dir;

use fs_extra::{copy_items, dir};

#[allow(clippy::vec_init_then_push)]
pub fn create_project(name: &str) -> anyhow::Result<()> {
	println!("Creating project: {name}");

	let options = dir::CopyOptions::new();
	let mut from_paths = Vec::new();

	println!("{}", current_dir()?.display());

	from_paths.push("templates/starter");
	copy_items(&from_paths, name, &options)?;

	Ok(())
}

pub fn generate_entity(name: &str) -> anyhow::Result<()> {
	println!("Generating entity: {name}");

	Ok(())
}
