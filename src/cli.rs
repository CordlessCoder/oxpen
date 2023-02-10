use std::path::PathBuf;

use clap::ColorChoice;
pub use clap::Parser;

use image::imageops::FilterType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "IMAGE")]
    /// The image to display
    pub image: PathBuf,
    #[arg(short, long, default_value = "triangle", value_name = "FILTER")]
    /// The filter method for resizing the image
    pub filter: FilterChoice,
    #[arg(short, long, value_name = "COLOR", default_value = "auto")]
    /// Whether to enable printing in color
    pub color: ColorChoice,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum FilterChoice {
    Nearest,
    Lanczos,
    Triangle,
    Gaussian,
    CatmullRom,
}

impl Into<FilterType> for FilterChoice {
    fn into(self) -> FilterType {
        match self {
            Self::Nearest => FilterType::Nearest,
            Self::Lanczos => FilterType::Lanczos3,
            Self::Triangle => FilterType::Triangle,
            Self::Gaussian => FilterType::Gaussian,
            Self::CatmullRom => FilterType::CatmullRom,
        }
    }
}
