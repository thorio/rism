#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use tauri::WindowUrl;

mod cli;

fn main() {
	let args = cli::Args::parse();

	let mut ctx = tauri::generate_context!();

	let window = ctx.config_mut().tauri.windows.first_mut().unwrap();

	window.url = WindowUrl::External(args.url);
	window.title = args.title;
	window.decorations = !args.no_decorate;
	window.transparent = args.transparent;
	window.width = args.width;
	window.height = args.height;

	tauri::Builder::default()
		.run(ctx)
		.expect("error while running tauri application");
}
