use image::{DynamicImage, GenericImageView, Rgb};

use crate::split::mask::FinalMask;

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

    shrink_y_overflow: Option<bool>,

    shrink_x_overflow: Option<bool>
}

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
    pub fn output_size(mut self, width: usize, height: usize) -> Self {
        self.output_size = Some((width, height));

        self
    }

    pub fn output_padding_y(mut self, padding: usize) -> Self {
        todo!();
        self        
    }
}