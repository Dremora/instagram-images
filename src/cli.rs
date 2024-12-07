use clap::Parser;

use crate::{aspect_ratio::AspectRatio, pad_color::PadColor};

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[arg(long)]
    pub source: String,

    #[arg(long)]
    pub target: String,

    #[arg(long, value_enum)]
    pub aspect_ratio: AspectRatio,

    #[arg(long, value_enum)]
    pub pad_color: PadColor,

    #[arg(long, default_value = "false")]
    pub force: bool,
}
