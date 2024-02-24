use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
	/// URL to navigate to on startup
	#[arg()]
	pub url: Option<Url>,

	#[arg(long, short, default_value = "rism")]
	pub title: String,

	/// Disable window borders and bars
	#[arg(long)]
	pub no_decorate: bool,

	/// Allows the window to be transparent if the website supports it
	#[arg(long)]
	pub transparent: bool,

	#[arg(long, default_value = "800")]
	pub width: f64,

	#[arg(long, default_value = "600")]
	pub height: f64,
}
