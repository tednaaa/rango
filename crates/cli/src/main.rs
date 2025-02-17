use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "rango")]
#[command(version = "0.1.0")]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Setup {
		#[arg(short, long)]
		name: String,
	},

	Generate {
		#[command(subcommand)]
		component: Components,
	},
}

#[derive(Subcommand)]
enum Components {
	Entity {
		#[arg(help = "Entity name (e.g. users)")]
		name: String,
	},
}

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Setup { name } => commands::setup_project(name),
		Commands::Generate { component } => match component {
			Components::Entity { name } => commands::generate_entity(name),
		},
	}
}
