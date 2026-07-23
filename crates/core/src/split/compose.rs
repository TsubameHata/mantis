use image::{Rgb, RgbImage, imageops::{FilterType, resize, crop}};

use crate::types::{MaskValue, Overflow};
use crate::split::mask::FinalMask;

#[derive(Default)]
pub struct ImageComposer<'a> {
    /// The mask to be proceeded.
    mask: Option<FinalMask>, 

    /// The height of the beginning and end of the main area of the mask. Closed interval.
    /// If not set, the margin will be considered in place of it.
    mask_padding_y: Option<(usize, usize)>,

    /// The original image to be proceeded. Must be the same size as the mask.
    img: Option<&'a RgbImage>,

    /// The size of the output, the width followed by the height. Example: (1920, 1080).
    output_size: Option<(usize, usize)>,

    /// The padding of the output, measured from the top border to the beginning of the main area of content.
    output_padding_y: Option<usize>,

    /// The background color of the output.
    /// Default value: (255, 255, 255).
    background_color: Option<Rgb<u8>>,

    overflow_y: Option<Overflow>,

    overflow_x: Option<Overflow>
}

// This `impl` block defines the initialization methods, which is all marked `pub`.
// All methods here are expected to be called in chain.
impl<'a> ImageComposer<'a> {
    /// Create an empty `ImageComposer`.
    pub fn new() -> Self {
        ImageComposer { mask: None, mask_padding_y: None, img: None, output_size: None, output_padding_y: None, background_color: None, overflow_y: None, overflow_x: None }
    }

    /// Set the mask to be proceeded. Panics if the image is already set and the size is inconsistent.
    pub fn mask(mut self, m: FinalMask) -> Self {
        if self.img.is_some() {
            assert_eq!(self.img.as_ref().unwrap().dimensions(), m.0.dimensions());
        }
        self.mask = Some(m);

        self
    }

    /// Set the height of the beginning and end of the main area of the mask. Closed interval.
    /// If not set, the margin will be considered in place of it.
    /// 
    /// Must be called after the mask is set.
    pub fn mask_padding_y(mut self, begin: usize, end: usize) -> Self {
        assert!(self.mask.is_some());
        assert!(end > begin);
        assert!(end < self.mask.as_ref().unwrap().0.height() as usize);

        self.mask_padding_y = Some((begin, end));

        self
    }

    /// Set the original image to be proceeded, which must be the same size as the mask, and panics if the size is inconsistent.
    pub fn img(mut self, i: &'a RgbImage) -> Self {
        if self.mask.is_some() {
            assert_eq!(self.mask.as_ref().unwrap().0.dimensions(), i.dimensions());
        }
        self.img = Some(i);

        self
    }

    /// Set the size of the output.
    /// 
    /// Default value is `(1920, 1080)`.
    pub fn output_size(mut self, width: usize, height: usize) -> Self {
        self.output_size = Some((width, height));

        self
    }

    /// Set the padding of the output, measured from the top border to the beginning of the main area of content.
    /// 
    /// This method should be called after the size of the output size is set, 
    /// and the padding must not be greater than half of the height of the page, otherwise the program panics.
    /// 
    /// Default value is `0.08*output_height`.
    pub fn output_padding_y(mut self, padding: usize) -> Self {
        assert!(self.output_size.is_some());
        assert!(2*padding < self.output_size.unwrap().1);

        self.output_padding_y = Some(padding);

        self        
    }

    /// Set the background color of the output.
    /// 
    /// Default value is `Rgb(255, 255, 255)`.
    pub fn background_color(mut self, color: Rgb<u8>) -> Self {
        self.background_color = Some(color);

        self
    }

    /// Set the action to take when overflow happens in y.
    /// 
    /// Default value is `Hidden`.
    pub fn overflow_y(mut self, action: Overflow) -> Self {
        self.overflow_y = Some(action);

        self
    }

    /// Set the action to take when overflow happens in x.
    /// 
    /// Default value is `Hidden`.
    pub fn overflow_x(mut self, action: Overflow) -> Self {
        self.overflow_x = Some(action);

        self
    }
}

/// For internal use only.
/// 
/// Revise the range to make the image aligned by the middle.
/// 
/// Return value: `(middle, half_length, new_range)`
fn symmetrize_range(content: (usize, usize), inner: (usize, usize), content_total_size: usize) -> (usize, usize, (usize, usize)) {
    let anchor = (inner.0 + inner.1)/2;

    // need to defend against potential underflow because inner is inputed by user
    let half = usize::max(anchor.checked_sub(content.0).unwrap_or(0), content.1.checked_sub(anchor).unwrap_or(0));

    (anchor, half, (anchor.checked_sub(half).unwrap_or(0), (anchor+half).min(content_total_size - 1)))
}

/// For internal use only. 
/// 
/// Return value: `(ratio, origin_range)`.
fn fix_ratio(
    origin_range_no_rev: (usize, usize),
    origin_inner_range: (usize, usize),
    origin_total: usize,
    output_total: usize,
    mode: Overflow,
    ratio: (usize, usize)
) -> ((usize, usize), (usize, usize)){
    let (origin_middle, origin_range_half, mut origin_range) = symmetrize_range(origin_range_no_rev, origin_inner_range, origin_total);

    let mut new_ratio = ratio;
    
    // deal with overflow
    // overflow <=> output_total < origin_total * ratio
    // <=> the following condition
    if output_total * ratio.1 < origin_range_half * 2 * ratio.0 {
        use Overflow::*;
        // overflow occurs
        match mode {
            Hidden => {
                // the same way of correction
                let den = 2 * ratio.0;
                let half = ((output_total - 1)*ratio.1 + den/2) / den;
                origin_range = (origin_middle.checked_sub(half).unwrap_or(0), (origin_middle + half).min(origin_total - 1))
            },
            
            Shrink => {
                new_ratio = (output_total, 2*origin_range_half);
            }
        }
    }
    
    (new_ratio, origin_range)
}

/// For internal use only. 
/// Add `pad` to the end of `range`.
fn expand_range(range: (usize, usize), total: usize, pad: usize) -> (usize, usize) {
    (
        range.0,
        (range.1 + pad).min(total - 1),
    )
}

// This `impl` block defines methods about actual operations.
impl<'a> ImageComposer<'a> {
    /// Set all unfilled options to default value. 
    /// This function must be called after `img` `mask` are set, otherwise it panics.
    /// 
    /// `mask` must not be empty, otherwise it also panics.
    /// 
    /// Intended to be called internally and not chained, exposed to `pub` for convenience.
    pub fn fill_options(&mut self) -> &mut Self {
        assert!(self.img.is_some());
        assert!(self.mask.is_some());

        if self.mask_padding_y.is_none() {
            // may panic if self.mask is empty
            self.mask_padding_y = Some(self.mask.as_ref().unwrap().y_range().unwrap());
        }

        if self.output_size.is_none() {
            self.output_size = Some((1920, 1080));
        }

        if self.output_padding_y.is_none() {
            self.output_padding_y = Some(
                ((self.output_size.as_ref().unwrap().1 as f32)*0.08f32) 
                as usize);
        }

        if self.background_color.is_none() {
            self.background_color = Some(Rgb([255u8, 255, 255]));
        }

        if self.overflow_x.is_none() {
            self.overflow_x = Some(Overflow::Hidden);
        }
        
        if self.overflow_y.is_none() {
            self.overflow_y = Some(Overflow::Hidden);
        }

        self
    }

    pub fn compose(mut self) -> RgbImage {
        self.fill_options();

        let (page_width, page_height) = self.mask.as_ref().unwrap().0.dimensions();
        let page_width = page_width as usize;
        let page_height = page_height as usize;

        let (output_width, output_height) = self.output_size.unwrap();
        let output_inner_height = output_height - self.output_padding_y.unwrap()*2;

        let mask_padding_y = self.mask_padding_y.unwrap();
        let origin_inner_height = mask_padding_y.1 - mask_padding_y.0 + 1;
        let origin_height_range = self.mask.as_ref().unwrap().y_range().unwrap();

        let ratio = (output_inner_height, origin_inner_height);

        let (ratio, _) = fix_ratio(
            origin_height_range, mask_padding_y, page_height, output_height, self.overflow_y.unwrap(), ratio);

        let origin_width_range = self.mask.as_ref().unwrap().x_range().unwrap();

        let (ratio, origin_x_range) = fix_ratio(
            origin_width_range, origin_width_range, page_width, output_width, self.overflow_x.unwrap(), ratio);
        
        let (ratio, origin_y_range) = fix_ratio(
            origin_height_range, mask_padding_y, page_height, output_height, self.overflow_y.unwrap(), ratio);

        let origin_x_range = expand_range(origin_x_range, page_width, 1);
        let origin_y_range = expand_range(origin_y_range, page_height, 1);

        let crop_width = origin_x_range.1 - origin_x_range.0 + 1;
        let crop_height = origin_y_range.1 - origin_y_range.0 + 1;

        // mutability for use of `crop`
        let mut mask = self.mask.as_ref().unwrap().0.clone();
        let mut img = self.img.unwrap().clone();

        let cropped_mask = crop(&mut mask, origin_x_range.0 as u32, origin_y_range.0 as u32, 
            crop_width as u32, crop_height as u32).to_image();
        let cropped_img = crop(&mut img, origin_x_range.0 as u32, origin_y_range.0 as u32, 
            crop_width as u32, crop_height as u32).to_image();

        let final_width = (crop_width * ratio.0 / ratio.1).min(output_width);
        let final_height = (crop_height * ratio.0 / ratio.1).min(output_height);

        // for the special format of mask, we must use `FilterType::Nearest`
        let resized_mask = resize(&cropped_mask, final_width as u32, final_height as u32, FilterType::Nearest);
        // the filter here can be more considered
        let resized_img = resize(&cropped_img, final_width as u32, final_height as u32, FilterType::Triangle);

        // the result cannot be perfectly centered, with an error less than 2px.
        // it is designed to align the target image slightly left and above, as the following offset defines.
        let offset_x = (output_width - final_width)/2;
        let offset_y = (output_height - final_height)/2;
        
        // create output canvas and paint

        // create canvas with background color
        let mut output = RgbImage::from_pixel(output_width as u32, output_height as u32, self.background_color.unwrap());
        
        // directly consumes mask and img
        let mask_raw = resized_mask.into_raw();
        let img_raw = resized_img.into_raw();

        // write pixels
        for y in 0..final_height {
            let mask_row = &mask_raw[y*final_width .. (y+1)*final_width];
            let img_row = &img_raw[3*y*final_width .. 3*(y+1)*final_width];
            for (x, (&m, pixel)) in mask_row.iter().zip(img_row.chunks(3)).enumerate() {
                if m==(MaskValue::Include as u8) {
                    output.put_pixel((x + offset_x) as u32, (y + offset_y) as u32, 
                        Rgb([pixel[0], pixel[1], pixel[2]]));
                }
            }
        }

        output
    }
}
