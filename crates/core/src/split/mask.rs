use image::{GrayImage, RgbImage, Rgb};

/// Allowed pixel values in `MaskLayer` and `FinalMask`.
#[repr(u8)]
pub enum MaskValue {
    Transparent = 0,
    Include = 1,
    Exclude = 2
}

impl TryFrom<u8> for MaskValue {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Transparent),
            1 => Ok(Self::Include),
            2 => Ok(Self::Exclude),
            _ => Err(())
        }
    }
}

/// Represents a mask layer, which is fundamentally an 8-bit gray image.
/// 
/// The value in its buffer should be valid `MaskValue`s by convention and is interpreted as its enum value, 
/// where `0` means transparent, `1` means include, and `2` means exclude.
/// 
/// When one `MaskLayer` is composited over another, each pixel in the upper one overrides the corresponding pixel in the lower layer.
/// Transparent pixels leave the lower layer unchanged.
pub struct MaskLayer(pub GrayImage);

/// Similar to `MaskLayer`, but the pixel value `0` is not allowed by convention.
pub struct FinalMask(pub GrayImage);


impl From<MaskLayer> for FinalMask {
    /// Drops `MaskValue::Transparent` in the `MaskLayer` and change its type.
    fn from(mut layer: MaskLayer) -> Self {
        for p in layer.0.as_mut() {
            if *p == MaskValue::Transparent as u8 {
                *p = MaskValue::Exclude as u8;
            }
        }

        Self(layer.0)
    }
}

impl From<FinalMask> for MaskLayer {
    fn from(mask: FinalMask) -> Self {
        Self(mask.0)
    }
}

impl MaskLayer {
    /// Converts image-based mask file with multiple masks in different colors in the legacy project into `MaskLayer`.
    pub fn from_legacy(img: RgbImage, color: Rgb<u8>) -> Self {
        let w = img.width() as usize;
        let h = img.height() as usize;
        
        let mut buf: Vec<u8> = Vec::with_capacity(w*h);
        for &pixel in img.pixels() {
            buf.push(
                if pixel==color {
                    MaskValue::Include as u8
                } else {
                    MaskValue::Transparent as u8
                }
            );
        }

        MaskLayer(GrayImage::from_raw(w as u32, h as u32, buf).unwrap())
    }

    /// Maps the value in the mask into a more explicit value for inspection.
    /// 
    /// `Transparent`, `Include`, `Exclude` in the output debug image are mapped to `127`, `0`, `255`, respectively, 
    /// based on the tendency of the mask to accept the pixel.
    pub fn to_debug_image(&self) -> GrayImage {
        let mut img = self.0.clone();

        for p in img.as_mut() {
            *p = match *p {
                0 => 127, // Transparent
                1 => 0, // Include
                2 => 255, // Exclude
                _ => panic!()
            }
        }

        img
    }

    /// Composites this layer over `lower`, and returns a new `MaskLayer`.
    pub fn composite_over(&self, lower: &Self) -> Self {
        let width = self.0.width();
        let height = self.0.height();
        
        assert!(width==lower.0.width() && height==lower.0.height());

        let mut buf: Vec<u8> = Vec::with_capacity((width*height) as usize);
        for (&h, &l) in self.0.iter().zip(lower.0.iter()) {
            buf.push(if h==MaskValue::Transparent as u8 {
                l
            } else {h});
        }

        let img = GrayImage::from_raw(width, height, buf).unwrap();

        Self(img)
    }

    /// Composites this layer under `upper`, and returns a new `MaskLayer`
    pub fn composite_under(&self, upper: &Self) -> Self {
        upper.composite_over(self)
    }

    /// Composites `lower` under this layer in place.
    pub fn composite_over_mut(&mut self, lower: &Self) {
        assert!(self.0.width()==lower.0.width() && self.0.height()==lower.0.height());

        for (h, &l) in self.0.iter_mut().zip(lower.0.iter()) {
            if *h==MaskValue::Transparent as u8 {
                *h = l;
            }
        }
    }

    /// Composites `upper` over this layer in place.
    pub fn composite_under_mut(&mut self, upper: &Self) {
        assert!(self.0.width()==upper.0.width() && self.0.height()==upper.0.height());

        for (l, &h) in self.0.iter_mut().zip(upper.0.iter()) {
            if h!=MaskValue::Transparent as u8 {
                *l = h;
            }
        }
    }
}

impl FinalMask {
    /// Composites `upper` over this layer in place. It is strongly recommended to do this operation in place, 
    /// given its purpose, therefore, no method without `_mut` is provided.
    pub fn composite_under_mut(&mut self, upper: &MaskLayer){
        assert!(self.0.width()==upper.0.width() && self.0.height()==upper.0.height());

        for (l, &h) in self.0.iter_mut().zip(upper.0.iter()) {
            if h!=MaskValue::Transparent as u8 {
                *l = h;
            }
        }
    }
}