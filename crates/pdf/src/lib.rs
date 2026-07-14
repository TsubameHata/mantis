use std::env::current_exe;
use image::RgbImage;
use pdfium_render::prelude::*;

/// Represents the range of page to render, whose `Range(usize, usize)` represents a closed interval.
pub enum Pages {
    All,
    // closed interval
    Range(usize, usize)
}

/// Render `pdf_data` into `Vec<image::RgbImage>`.
/// 
/// `pdfium.dll` is searched in the following locations, in order of priority:
/// 
/// 1. The current working directory.
/// 2. The directory containing the executable.
/// 3. Standard system library search paths.
pub fn render(pdf_data: Vec<u8>, target_width: usize, maximum_height: usize, page_range: Pages) -> Vec<RgbImage> {
    if let Pages::Range(beg, end) = page_range {
        assert!(beg<=end);
    }
    
    let bindings = Pdfium::bind_to_library(
        Pdfium::pdfium_platform_library_name_at_path("./")
    ).or_else(
        |_| {
            let exe_path = current_exe().unwrap().parent().unwrap().to_path_buf();
            Pdfium::bind_to_library(
                Pdfium::pdfium_platform_library_name_at_path(&exe_path)
            )
        }
    ).or_else(
        |_| Pdfium::bind_to_system_library() 
    ).unwrap();

    let pdfium = Pdfium::new(bindings);

    let render_config = PdfRenderConfig::new()
        .set_target_width(target_width as i32)
        .set_maximum_height(maximum_height as i32);

    let document = pdfium.load_pdf_from_byte_vec(pdf_data, None).unwrap();

    let pages = document.pages();

    let (beg, end) = match page_range {
        Pages::All => {
            (
                0 as usize,
                pages.len() as usize - 1
            )
        },
        Pages::Range(beg, end) => {
            assert!(end < pages.len() as usize);
            (beg, end)
        }
    };

    let mut ret: Vec<RgbImage> = Vec::with_capacity(end-beg+1);

    for index in beg..=end {
        let page = pages.get(index as i32).unwrap();
        
        ret.push(
            page.render_with_config(&render_config).unwrap()
                .as_image().unwrap()
                .into_rgb8()  
        );
    }

    ret
}