use image::{DynamicImage, GenericImageView, Rgb, RgbImage};

use crate::split::mask::FinalMask;

/// Defines the action to take when the size of image, after scaling according to the padding, 
/// is greater than the output canvas.
pub enum Overflow {
    Hidden,
    Shrink
}

#[derive(Default)]
pub struct ImageComposer {
    /// The mask to be proceeded.
    mask: Option<FinalMask>, 

    /// The height of the beginning and end of the main area of the mask. Closed interval.
    /// If not set, the margin will be considered in place of it.
    mask_padding_y: Option<(usize, usize)>,

    /// The original image to be proceeded. Must be the same size as the mask.
    img: Option<DynamicImage>,

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
impl ImageComposer {
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
    pub fn img(mut self, i: DynamicImage) -> Self {
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

    // /// Set the action to take when overflow happens in x.
    // /// 
    // /// Default value is `Hidden`.
    // pub fn overflow_x(mut self, action: Overflow) -> Self {
    //     self.overflow_x = Some(action);

    //     self
    // }
}

// This `impl` block defines methods about actual operations.
impl ImageComposer {
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

        // if self.overflow_x.is_none() {
        //     self.overflow_x = Some(Overflow::Hidden);
        // }
        
        if self.overflow_y.is_none() {
            self.overflow_y = Some(Overflow::Hidden);
        }

        self
    }

    pub fn compose(mut self) -> RgbImage {

        // utility function
        fn compute_range_with_correction(origin_range: &(usize, usize), ratio_num: usize, ratio_den: usize) -> (usize, usize) {
            // emphasize clone semantic
            let tmp = origin_range;

            // if ignore the correction below
            // the length of the final result may be slightly different from output
            let correction = ratio_den / 2;
            // (tmp.0 * ratio_num / ratio_den, tmp.1 * ratio_num / ratio_den)
            ((tmp.0 * ratio_num + correction)/ratio_den, (tmp.1 * ratio_num + correction)/ratio_den)
        }

        self.fill_options();

        // with revision, 
        // to make the image aligned by the middle height of padding
        let origin_y_middle = {
            let mpy = self.mask_padding_y.unwrap();
            (mpy.0 + mpy.1)/2
        };
        let origin_y_range_no_rev = self.mask.as_ref().unwrap().y_range().unwrap();
        let origin_height_half = usize::max(origin_y_range_no_rev.1 - origin_y_middle, 
            origin_y_middle - origin_y_range_no_rev.0);
        let origin_height = 2*origin_height_half;
        let origin_y_range = (origin_y_middle-origin_height_half, origin_y_middle+origin_height_half);

        let origin_inner_height = {
            let mpy = self.mask_padding_y.unwrap();
            mpy.1 - mpy.0
        };
            
        let output_height = self.output_size.unwrap().1;
        let output_inner_height = output_height - 2*self.output_padding_y.unwrap();

        // store the numerator and denominator seperately for convenience and percision
        let (mut ratio_num, mut ratio_den) = (output_inner_height, origin_inner_height);

        // coordinate of resized and not cropped picture
        let mut needed_y_range = compute_range_with_correction(&origin_y_range, ratio_num, ratio_den);

        // deal with y-overflow first
        // y-overflow: output_inner_height/output_height > origin_inner_height/origin_height,
        // or, output_height < origin_height * ratio
        if output_height * ratio_den < origin_height * ratio_num {
            // y-overflow occurs
            match self.overflow_y.unwrap() {
                Overflow::Hidden => {
                    // the same way of correction
                    let correction = ratio_den / 2;

                    let half_fittable_height = (origin_height_half * ratio_num + correction) / ratio_den;
                    needed_y_range = 
                        (origin_y_middle-half_fittable_height, origin_y_middle+half_fittable_height);
                },
                Overflow::Shrink => {
                    (ratio_num, ratio_den) = 
                        (output_height, origin_height);
                    needed_y_range = compute_range_with_correction(&origin_y_range, ratio_num, ratio_den);
                }
            }
        }
        
        todo!()
    }
}