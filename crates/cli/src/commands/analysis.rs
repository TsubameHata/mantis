use std::path::PathBuf;

use core::analysis::gaussian_conv;

#[derive(clap::Args)]
pub struct Args {
    /// The path of the image to analyze
    image: PathBuf,

    // can consider if we need option specifying min_peak_distance in pixels

    /// Minimum distance between split lines in ratio relative to the height of page
    #[arg(short='d', 
        long="min-distance-r", 
        default_value_t=0.2
    )]
    min_distance_ratio: f32,

    /// Minimum prominence of y index to be considered as split line
    #[arg(short='p', long="min-prominence", default_value_t=0.03)]
    min_prominence: f32,

    /// Export the detailed probability as image
    #[arg(short='e', long="export-prob", default_value_t=false)]
    export_prob: bool,

    /// Specify the path of output file [default: input_file.out.jpg]
    #[arg(short='o', long="output-file", requires="export_prob")]
    output_file: Option<PathBuf>
}

pub fn handle(args: Args) {

}