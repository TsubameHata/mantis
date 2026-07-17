use std::path::PathBuf;

use image::ImageReader;

use mantis::analysis::gaussian_conv;

#[derive(clap::Args)]
pub struct Args {
    /// The path of the image to analyze
    pub image: PathBuf,

    // can consider if we need option specifying min_peak_distance in pixels

    /// Minimum distance between split lines in ratio relative to the height of page
    #[arg(short='d', 
        long="min-distance-r", 
        default_value_t=0.15
    )]
    pub min_distance_ratio: f32,

    /// Minimum prominence of y index to be considered as split line
    #[arg(short='p', long="min-prominence", default_value_t=0.03)]
    pub min_prominence: f32,

    /// Export the detailed probability as image
    #[arg(short='e', long="export-prob", default_value_t=false)]
    pub export_prob: bool,

    /// Specify the path of output file [default: input_file.out.jpg]
    #[arg(short='o', long="output-file", requires="export_prob")]
    pub output_file: Option<PathBuf>
}

pub fn handle(args: Args) {
    let img = ImageReader::open(&args.image).unwrap().decode().unwrap();
    let prob = gaussian_conv::prob(&img.to_luma8());

    let h = img.height() as usize;
    let min_d = ( args.min_distance_ratio * (h as f32) ) as usize;
    let peaks = gaussian_conv::find_peaks(&prob, min_d, args.min_prominence);

    println!("{:#?}", &peaks);

    if !args.export_prob { return; }

    let result = gaussian_conv::paint_peaks(&img.to_rgb8(), &prob, &peaks);
    
    let path = match args.output_file {
        Some(p) => p,
        None => {
            let mut p = args.image;
            p.set_extension("out.jpg");
            p
        }
    };
    result.save(path).unwrap();
}