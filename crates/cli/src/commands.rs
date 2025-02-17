use std::{env, fs, path::Path};

use include_dir::{include_dir, Dir};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

const TEMPLATE_STARTER: &str = "starter";

#[allow(clippy::vec_init_then_push)]
pub fn setup_project(name: &str) -> anyhow::Result<()> {
	let current_dir = env::current_dir()?;
	let project_path = current_dir.join(name);

	if project_path.exists() {
		anyhow::bail!("Directory already exists: {project_path:?}");
	}

	fs::create_dir_all(&project_path)?;

	if let Some(template_dir) = TEMPLATES_DIR.get_dir(TEMPLATE_STARTER) {
		copy_dir_from_embedded(template_dir, &project_path)?;
	}

	println!("Project created: {project_path:?}");
	Ok(())
}

fn copy_dir_from_embedded(dir: &Dir, dest: &Path) -> anyhow::Result<()> {
	for file in dir.files() {
		let relative_path = file
			.path()
			.strip_prefix(dir.path())
			.map_err(|e| anyhow::anyhow!("Failed to strip directory prefix for file: {}", e))?;
		let dest_file_path = dest.join(relative_path);

		if let Some(parent) = dest_file_path.parent() {
			fs::create_dir_all(parent)?;
		}

		fs::write(&dest_file_path, file.contents())?;
	}

	for subdir in dir.dirs() {
		let subdir_relative_path = subdir
			.path()
			.strip_prefix(dir.path())
			.map_err(|e| anyhow::anyhow!("Failed to strip directory prefix for subdir: {}", e))?;

		let subdir_dest = dest.join(subdir_relative_path);
		copy_dir_from_embedded(subdir, &subdir_dest)?;
	}

	Ok(())
}

pub fn generate_entity(name: &str) -> anyhow::Result<()> {
	println!("Generating entity: {name}");

	Ok(())
}
