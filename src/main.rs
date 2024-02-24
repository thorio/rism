#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use tauri::WindowUrl;

mod cli;

#[cfg(windows)]
mod windows_console;

fn main() {
	#[cfg(windows)]
	windows_console::attach();

	let args = cli::Args::parse();

	let mut ctx = tauri::generate_context!();

	let window = ctx.config_mut().tauri.windows.first_mut().unwrap();

	if let Some(url) = args.url {
		window.url = WindowUrl::External(url);
	}

	window.title = args.title;
	window.decorations = !args.no_decorate;
	window.transparent = args.transparent;
	window.width = args.width;
	window.height = args.height;

	tauri::Builder::default()
		.run(ctx)
		.expect("error while running tauri application");

	#[cfg(windows)]
	windows_console::detach();
}
