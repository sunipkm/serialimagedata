#![warn(missing_docs)]
use image::{DynamicImage, ImageBuffer, Luma, LumaA, Rgb};
use serde::{Deserialize, Serialize};

pub use image::Primitive;

use super::ImageMetaData;

/// Valid types for the serial image data structure: [`u8`], [`u16`], [`f32`].

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct SerialImageInternal<T: Primitive> {
    luma: Option<Vec<T>>,
    red: Option<Vec<T>>,
    green: Option<Vec<T>>,
    blue: Option<Vec<T>>,
    alpha: Option<Vec<T>>,
    pixel_elems: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// A serializable image data container for [`u8`], [`u16`] and [`f32`] pixel types.
///
/// Image data is organized in channels. For example, a grayscale image stores data in the luma channel, while a color image stores data in the red, green and blue channels. Transparency is stored in the alpha channel.
pub struct SerialImageBuffer<T: Primitive> {
    meta: Option<ImageMetaData>,
    data: SerialImageInternal<T>,
    width: usize,
    height: usize,
}

impl<T: Primitive> SerialImageBuffer<T> {
    /// Create a new serializable image buffer from vector data.
    ///
    /// # Arguments
    ///  - `width`: Image width.
    ///  - `height`: Image height.
    ///  - `data`: Image data.
    ///
    /// Note:
    ///  - If `width * height == data.len()`, the image is assumed to be a grayscale image.
    ///  - If `width * height * 2 == data.len()`, the image is assumed to be a grayscale image with alpha channel, with the odd pixels being the luma channel and the even pixels being the alpha channel.
    ///  - If `width * height * 3 == data.len()`, the image is assumed to be a color image, with the first pixel in the red channel, the second pixel in the green channel, and the third pixel in the blue channel and so on.
    ///  - If `width * height * 4 == data.len()`, the image is assumed to be a color image with alpha channel, with the first pixel in the red channel, the second pixel in the green channel, the third pixel in the blue channel and the fourth pixel in the alpha channel and so on.
    ///
    ///
    /// # Errors
    ///  - If `width * height == 0`.
    ///  - If number of pixel elements is not in `[1..=4]`.
    ///  - If the length of the channel data stored in the image is not equal to `width * height * pixel elements`. Number of pixel elements are inferred using the length of the data vector.
    ///
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Result<Self, &'static str> {
        if width * height == 0 {
            return Err("Width and height must be greater than zero");
        }
        let pixel_elems = data.len() / (width * height);
        if data.len() != width * height * pixel_elems {
            return Err("Data length must be equal to width * height * pixel elements");
        }
        if pixel_elems > 4 || pixel_elems == 0 {
            return Err("Invalid number of pixel elements");
        }

        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems as u8);

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems: pixel_elems as u8,
            },
            width,
            height,
        })
    }

    fn from_vec_unsafe(
        size: usize,
        data: Vec<T>,
        elems: u8,
    ) -> (
        Option<Vec<T>>,
        Option<Vec<T>>,
        Option<Vec<T>>,
        Option<Vec<T>>,
        Option<Vec<T>>,
    ) {
        if elems == 1 {
            return (Some(data), None, None, None, None);
        } else if elems == 2 {
            let mut luma = Vec::with_capacity(size);
            let mut alpha = Vec::with_capacity(size);
            for i in 0..size {
                luma.push(data[i * 2]);
                alpha.push(data[i * 2 + 1]);
            }
            return (Some(luma), None, None, None, Some(alpha));
        } else if elems == 3 {
            let mut red = Vec::with_capacity(size);
            let mut green = Vec::with_capacity(size);
            let mut blue = Vec::with_capacity(size);
            for i in 0..size {
                red.push(data[i * 3]);
                green.push(data[i * 3 + 1]);
                blue.push(data[i * 3 + 2]);
            }
            return (None, Some(red), Some(green), Some(blue), None);
        } else if elems == 4 {
            let mut red = Vec::with_capacity(size);
            let mut green = Vec::with_capacity(size);
            let mut blue = Vec::with_capacity(size);
            let mut alpha = Vec::with_capacity(size);
            for i in 0..size {
                red.push(data[i * 4]);
                green.push(data[i * 4 + 1]);
                blue.push(data[i * 4 + 2]);
                alpha.push(data[i * 4 + 3]);
            }
            return (None, Some(red), Some(green), Some(blue), Some(alpha));
        } else {
            panic!("Invalid number of elements");
        }
    }

    /// Get the image metadata.
    pub fn get_metadata(&self) -> Option<&ImageMetaData> {
        self.meta.as_ref()
    }

    /// Get a mutable reference to the image metadata.
    pub fn get_mut_metadata(&mut self) -> Option<&mut ImageMetaData> {
        self.meta.as_mut()
    }

    /// Update the image metadata.
    ///
    /// # Arguments
    ///  - `meta`: Image metadata.
    pub fn set_metadata(&mut self, meta: Option<ImageMetaData>) {
        self.meta = meta;
    }

    /// Get the luminosity channel data.
    pub fn get_luma(&self) -> Option<&Vec<T>> {
        self.data.luma.as_ref()
    }

    /// Get a mutable reference to the luminosity channel data.
    pub fn get_mut_luma(&mut self) -> Option<&mut Vec<T>> {
        self.data.luma.as_mut()
    }

    /// Get the red channel data.
    pub fn get_red(&self) -> Option<&Vec<T>> {
        self.data.red.as_ref()
    }

    /// Get a mutable reference to the red channel data.
    pub fn get_mut_red(&mut self) -> Option<&mut Vec<T>> {
        self.data.red.as_mut()
    }

    /// Get the green channel data.
    pub fn get_green(&self) -> Option<&Vec<T>> {
        self.data.green.as_ref()
    }

    /// Get a mutable reference to the green channel data.
    pub fn get_mut_green(&mut self) -> Option<&mut Vec<T>> {
        self.data.green.as_mut()
    }

    /// Get the blue channel data.
    pub fn get_blue(&self) -> Option<&Vec<T>> {
        self.data.blue.as_ref()
    }

    /// Get a mutable reference to the blue channel data.
    pub fn get_mut_blue(&mut self) -> Option<&mut Vec<T>> {
        self.data.blue.as_mut()
    }

    /// Get the alpha channel data.
    pub fn get_alpha(&self) -> Option<&Vec<T>> {
        self.data.alpha.as_ref()
    }

    /// Get a mutable reference to the alpha channel data.
    pub fn get_mut_alpha(&mut self) -> Option<&mut Vec<T>> {
        self.data.alpha.as_mut()
    }

    /// Get image width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get image height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the number of pixel elements.
    pub fn pixel_elems(&self) -> u8 {
        self.data.pixel_elems
    }

    /// Check if the image is grayscale.
    pub fn is_luma(&self) -> bool {
        self.data.pixel_elems == 1
    }

    /// Check if the image is RGB.
    pub fn is_rgb(&self) -> bool {
        self.data.pixel_elems == 3
    }

    /// Consume the image buffer and return a contiguous vector.
    ///
    /// Note:
    ///  - If the image is grayscale, the vector contains the luma channel data.
    ///  - If the image is grayscale with alpha channel, odd pixels are luminoisty and even pixels are alpha.
    ///  - If the image is RGB, the first element of the vector is red, the second element is green and the third element is blue and so on.
    ///  - If the image is RGB with alpha channel, the first element of the vector is red, the second element is green, the third element is blue and the fourth element is alpha and so on.
    pub fn into_vec(self) -> Vec<T> {
        let mut data =
            Vec::with_capacity(self.width * self.height * self.data.pixel_elems as usize);

        if self.width * self.height == 0 {
            return Vec::new();
        } else if self.data.pixel_elems == 1 {
            return self.data.luma.unwrap();
        } else if self.data.pixel_elems == 2 {
            let luma = self.data.luma.unwrap();
            let alpha = self.data.alpha.unwrap();
            for i in 0..self.width * self.height {
                data.push(luma[i]);
                data.push(alpha[i]);
            }
        } else if self.data.pixel_elems == 3 {
            let red = self.data.red.unwrap();
            let green = self.data.green.unwrap();
            let blue = self.data.blue.unwrap();
            for i in 0..self.width * self.height {
                data.push(red[i]);
                data.push(green[i]);
                data.push(blue[i]);
            }
        } else if self.data.pixel_elems == 4 {
            let red = self.data.red.unwrap();
            let green = self.data.green.unwrap();
            let blue = self.data.blue.unwrap();
            let alpha = self.data.alpha.unwrap();
            for i in 0..self.width * self.height {
                data.push(red[i]);
                data.push(green[i]);
                data.push(blue[i]);
                data.push(alpha[i]);
            }
        } else {
            panic!("Invalid number of elements");
        }

        return data;
    }
}

impl SerialImageBuffer<u8> {
    /// Create a new serializable image buffer.
    ///
    /// # Arguments
    ///  - `meta`: Image metadata (optional).
    ///  - `luma`: Luminosity data for a grayscale image. Set to `None` if it is a color image.
    ///  - `red`: Red channel data. Set to `None` if it is a grayscale image.
    ///  - `green`: Green channel data. Set to `None` if it is a grayscale image.
    ///  - `blue`: Blue channel data. Set to `None` if it is a grayscale image.
    ///  - `alpha`: Alpha channel data (optional).
    ///
    /// # Errors
    ///  - If `width * height == 0`.
    ///  - If all color channels are not specified.
    ///  - If `luma` and color channels are specified at the same time.
    ///  - If the length of the channel data stored in the image is not equal to `width * height`.
    pub fn new(
        meta: Option<ImageMetaData>,
        luma: Option<Vec<u8>>,
        red: Option<Vec<u8>>,
        green: Option<Vec<u8>>,
        blue: Option<Vec<u8>>,
        alpha: Option<Vec<u8>>,
        width: usize,
        height: usize,
    ) -> Result<Self, &'static str> {
        if width * height == 0 {
            return Err("Width and height must be greater than zero");
        }
        let colors = red.is_some() as u8 + green.is_some() as u8 + blue.is_some() as u8;
        if colors > 0 && colors != 3 {
            return Err("All color channels must be specified.");
        }
        if luma.is_some() && colors > 0 {
            return Err("Luma and color channels cannot be specified at the same time");
        }
        if luma.is_some() && luma.as_ref().unwrap().len() != width * height {
            return Err("Length of luma channel must be equal to width * height");
        }
        if red.is_some() && red.as_ref().unwrap().len() != width * height {
            return Err("Length of red channel must be equal to width * height");
        }
        if green.is_some() && green.as_ref().unwrap().len() != width * height {
            return Err("Length of green channel must be equal to width * height");
        }
        if blue.is_some() && blue.as_ref().unwrap().len() != width * height {
            return Err("Length of blue channel must be equal to width * height");
        }
        if alpha.is_some() && alpha.as_ref().unwrap().len() != width * height {
            return Err("Length of alpha channel must be equal to width * height");
        }
        let pixel_elems = colors + luma.is_some() as u8 + alpha.is_some() as u8;
        Ok(Self {
            meta,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        })
    }
}

impl SerialImageBuffer<u16> {
    /// Create a new serializable image buffer.
    ///
    /// # Arguments
    ///  - `meta`: Image metadata (optional).
    ///  - `luma`: Luminosity data for a grayscale image. Set to `None` if it is a color image.
    ///  - `red`: Red channel data. Set to `None` if it is a grayscale image.
    ///  - `green`: Green channel data. Set to `None` if it is a grayscale image.
    ///  - `blue`: Blue channel data. Set to `None` if it is a grayscale image.
    ///  - `alpha`: Alpha channel data (optional).
    ///
    /// # Errors
    ///  - If `width * height == 0`.
    ///  - If all color channels are not specified.
    ///  - If `luma` and color channels are specified at the same time.
    ///  - If the length of the channel data stored in the image is not equal to `width * height`.
    pub fn new(
        meta: Option<ImageMetaData>,
        luma: Option<Vec<u16>>,
        red: Option<Vec<u16>>,
        green: Option<Vec<u16>>,
        blue: Option<Vec<u16>>,
        alpha: Option<Vec<u16>>,
        width: usize,
        height: usize,
    ) -> Result<Self, &'static str> {
        if width * height == 0 {
            return Err("Width and height must be greater than zero");
        }
        let colors = red.is_some() as u8 + green.is_some() as u8 + blue.is_some() as u8;
        if colors > 0 && colors != 3 {
            return Err("All color channels must be specified.");
        }
        if luma.is_some() && colors > 0 {
            return Err("Luma and color channels cannot be specified at the same time");
        }
        if luma.is_some() && luma.as_ref().unwrap().len() != width * height {
            return Err("Length of luma channel must be equal to width * height");
        }
        if red.is_some() && red.as_ref().unwrap().len() != width * height {
            return Err("Length of red channel must be equal to width * height");
        }
        if green.is_some() && green.as_ref().unwrap().len() != width * height {
            return Err("Length of green channel must be equal to width * height");
        }
        if blue.is_some() && blue.as_ref().unwrap().len() != width * height {
            return Err("Length of blue channel must be equal to width * height");
        }
        if alpha.is_some() && alpha.as_ref().unwrap().len() != width * height {
            return Err("Length of alpha channel must be equal to width * height");
        }
        let pixel_elems = colors + luma.is_some() as u8 + alpha.is_some() as u8;
        Ok(Self {
            meta,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        })
    }
}

impl SerialImageBuffer<f32> {
    /// Create a new serializable image buffer.
    ///
    /// # Arguments
    ///  - `meta`: Image metadata (optional).
    ///  - `red`: Red channel data. Set to `None` if it is a grayscale image.
    ///  - `green`: Green channel data. Set to `None` if it is a grayscale image.
    ///  - `blue`: Blue channel data. Set to `None` if it is a grayscale image.
    ///  - `alpha`: Alpha channel data (optional).
    ///
    /// # Errors
    ///  - If `width * height == 0`.
    ///  - If the length of the channel data stored in the image is not equal to `width * height`.
    pub fn new(
        meta: Option<ImageMetaData>,
        red: Vec<f32>,
        green: Vec<f32>,
        blue: Vec<f32>,
        alpha: Option<Vec<f32>>,
        width: usize,
        height: usize,
    ) -> Result<Self, &'static str> {
        if width * height == 0 {
            return Err("Width and height must be greater than zero");
        }
        if red.len() != width * height {
            return Err("Length of red channel must be equal to width * height");
        }
        if green.len() != width * height {
            return Err("Length of green channel must be equal to width * height");
        }
        if blue.len() != width * height {
            return Err("Length of blue channel must be equal to width * height");
        }
        if alpha.is_some() && alpha.as_ref().unwrap().len() != width * height {
            return Err("Length of alpha channel must be equal to width * height");
        }
        let elems = if alpha.is_some() { 4 } else { 3 };
        Ok(Self {
            meta,
            data: SerialImageInternal {
                luma: None,
                red: Some(red),
                green: Some(green),
                blue: Some(blue),
                alpha,
                pixel_elems: elems,
            },
            width,
            height,
        })
    }
}

impl TryFrom<DynamicImage> for SerialImageBuffer<u8> {
    type Error = &'static str;

    fn try_from(image: DynamicImage) -> Result<Self, Self::Error> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let pixel_elems = image.color().channel_count();
        let luma;
        let red;
        let green;
        let blue;
        let alpha;

        match image {
            DynamicImage::ImageLuma8(img) => {
                luma = Some(img.into_raw());
                red = None;
                green = None;
                blue = None;
                alpha = None;
            }
            DynamicImage::ImageLumaA8(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            DynamicImage::ImageRgb8(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            DynamicImage::ImageRgba8(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            _ => {
                return Err("Image type not supported");
            }
        }

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width: width as usize,
            height: height as usize,
        })
    }
}

impl TryFrom<DynamicImage> for SerialImageBuffer<u16> {
    type Error = &'static str;

    fn try_from(image: DynamicImage) -> Result<Self, Self::Error> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let pixel_elems = image.color().channel_count();
        let luma;
        let red;
        let green;
        let blue;
        let alpha;

        match image {
            DynamicImage::ImageLuma16(img) => {
                luma = Some(img.into_raw());
                red = None;
                green = None;
                blue = None;
                alpha = None;
            }
            DynamicImage::ImageLumaA16(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            DynamicImage::ImageRgb16(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            DynamicImage::ImageRgba16(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            _ => {
                return Err("Image type not supported");
            }
        }

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width: width as usize,
            height: height as usize,
        })
    }
}

impl TryFrom<DynamicImage> for SerialImageBuffer<f32> {
    type Error = &'static str;

    fn try_from(image: DynamicImage) -> Result<Self, Self::Error> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let pixel_elems = image.color().channel_count();
        let luma;
        let red;
        let green;
        let blue;
        let alpha;

        match image {
            DynamicImage::ImageRgb32F(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            DynamicImage::ImageRgba32F(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.into_raw(), pixel_elems)
            }
            _ => {
                return Err("Image type not supported");
            }
        }

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width: width as usize,
            height: height as usize,
        })
    }
}

impl Into<DynamicImage> for SerialImageBuffer<u8> {
    fn into(self) -> DynamicImage {
        let width = self.width;
        let height = self.height;
        let pixel_elems = self.data.pixel_elems;
        let data = self.into_vec();

        match pixel_elems {
            1 => {
                let img = ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLuma8(img)
            }
            2 => {
                let img = ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLumaA8(img)
            }
            3 => {
                let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgb8(img)
            }
            4 => {
                let img = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgba8(img)
            }
            _ => panic!("Pixel elements not supported"),
        }
    }
}

impl Into<DynamicImage> for SerialImageBuffer<u16> {
    fn into(self) -> DynamicImage {
        let width = self.width;
        let height = self.height;
        let pixel_elems = self.data.pixel_elems;
        let data = self.into_vec();

        match pixel_elems {
            1 => {
                let img = ImageBuffer::<image::Luma<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLuma16(img)
            }
            2 => {
                let img = ImageBuffer::<image::LumaA<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLumaA16(img)
            }
            3 => {
                let img = ImageBuffer::<image::Rgb<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgb16(img)
            }
            4 => {
                let img = ImageBuffer::<image::Rgba<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgba16(img)
            }
            _ => panic!("Pixel elements not supported"),
        }
    }
}

impl Into<DynamicImage> for SerialImageBuffer<f32> {
    fn into(self) -> DynamicImage {
        let width = self.width;
        let height = self.height;
        let pixel_elems = self.data.pixel_elems;
        let data = self.into_vec();

        match pixel_elems {
            3 => {
                let img = ImageBuffer::<image::Rgb<f32>, Vec<f32>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgb32F(img)
            }
            4 => {
                let img = ImageBuffer::<image::Rgba<f32>, Vec<f32>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgba32F(img)
            }
            _ => panic!("Pixel elements not supported"),
        }
    }
}

impl TryFrom<&DynamicImage> for SerialImageBuffer<u8> {
    type Error = &'static str;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let pixel_elems = image.color().channel_count();
        let luma;
        let red;
        let green;
        let blue;
        let alpha;

        match image {
            DynamicImage::ImageLuma8(img) => {
                luma = Some(img.as_raw().clone());
                red = None;
                green = None;
                blue = None;
                alpha = None;
            }
            DynamicImage::ImageLumaA8(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.as_raw().clone(), pixel_elems);
            }
            DynamicImage::ImageRgb8(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.as_raw().clone(), pixel_elems);
            }
            DynamicImage::ImageRgba8(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.as_raw().clone(), pixel_elems);
            }
            _ => {
                return Err("Image type not supported");
            }
        }

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width: width as usize,
            height: height as usize,
        })
    }
}

impl TryFrom<&DynamicImage> for SerialImageBuffer<u16> {
    type Error = &'static str;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let pixel_elems = image.color().channel_count();
        let luma;
        let red;
        let green;
        let blue;
        let alpha;

        match image {
            DynamicImage::ImageLuma16(img) => {
                luma = Some(img.as_raw().clone());
                red = None;
                green = None;
                blue = None;
                alpha = None;
            }
            DynamicImage::ImageLumaA16(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.as_raw().clone(), pixel_elems);
            }
            DynamicImage::ImageRgb16(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.as_raw().clone(), pixel_elems);
            }
            DynamicImage::ImageRgba16(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.as_raw().clone(), pixel_elems);
            }
            _ => {
                return Err("Image type not supported");
            }
        }

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width: width as usize,
            height: height as usize,
        })
    }
}

impl TryFrom<&DynamicImage> for SerialImageBuffer<f32> {
    type Error = &'static str;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let pixel_elems = image.color().channel_count();
        let luma;
        let red;
        let green;
        let blue;
        let alpha;

        match image {
            DynamicImage::ImageRgb32F(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.clone().into_raw(), pixel_elems)
            }
            DynamicImage::ImageRgba32F(img) => {
                (luma, red, green, blue, alpha) =
                    Self::from_vec_unsafe(width * height, img.clone().into_raw(), pixel_elems)
            }
            _ => {
                return Err("Image type not supported");
            }
        }

        Ok(Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width: width as usize,
            height: height as usize,
        })
    }
}

impl Into<DynamicImage> for &SerialImageBuffer<u8> {
    fn into(self) -> DynamicImage {
        let width = self.width;
        let height = self.height;
        let pixel_elems = self.data.pixel_elems;
        let data = self.clone().into_vec();

        match pixel_elems {
            1 => {
                let img = ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLuma8(img)
            }
            2 => {
                let img = ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLumaA8(img)
            }
            3 => {
                let img = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgb8(img)
            }
            4 => {
                let img = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgba8(img)
            }
            _ => panic!("Pixel elements not supported"),
        }
    }
}

impl Into<DynamicImage> for &SerialImageBuffer<u16> {
    fn into(self) -> DynamicImage {
        let width = self.width;
        let height = self.height;
        let pixel_elems = self.data.pixel_elems;
        let data = self.clone().into_vec();

        match pixel_elems {
            1 => {
                let img = ImageBuffer::<image::Luma<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLuma16(img)
            }
            2 => {
                let img = ImageBuffer::<image::LumaA<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageLumaA16(img)
            }
            3 => {
                let img = ImageBuffer::<image::Rgb<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgb16(img)
            }
            4 => {
                let img = ImageBuffer::<image::Rgba<u16>, Vec<u16>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgba16(img)
            }
            _ => panic!("Pixel elements not supported"),
        }
    }
}

impl Into<DynamicImage> for &SerialImageBuffer<f32> {
    fn into(self) -> DynamicImage {
        let width = self.width;
        let height = self.height;
        let pixel_elems = self.data.pixel_elems;
        let data = self.clone().into_vec();

        match pixel_elems {
            3 => {
                let img = ImageBuffer::<image::Rgb<f32>, Vec<f32>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgb32F(img)
            }
            4 => {
                let img = ImageBuffer::<image::Rgba<f32>, Vec<f32>>::from_raw(
                    width as u32,
                    height as u32,
                    data,
                )
                .unwrap();
                DynamicImage::ImageRgba32F(img)
            }
            _ => panic!("Pixel elements not supported"),
        }
    }
}

impl<T: Primitive> TryInto<ImageBuffer<Luma<T>, Vec<T>>> for SerialImageBuffer<T> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Luma<T>, Vec<T>>, Self::Error> {
        if self.data.pixel_elems != 1 {
            return Err("Image must have one element per pixel");
        }
        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }
        let img = ImageBuffer::<Luma<T>, Vec<T>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl<T: Primitive> TryInto<ImageBuffer<Luma<T>, Vec<T>>> for &SerialImageBuffer<T> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Luma<T>, Vec<T>>, Self::Error> {
        if self.data.pixel_elems != 1 {
            return Err("Image must have one element per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Luma<T>, Vec<T>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.clone().unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl<T: Primitive> TryInto<ImageBuffer<LumaA<T>, Vec<T>>> for SerialImageBuffer<T> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<LumaA<T>, Vec<T>>, Self::Error> {
        if self.data.pixel_elems != 2 {
            return Err("Image must have two elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<LumaA<T>, Vec<T>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl<T: Primitive> TryInto<ImageBuffer<LumaA<T>, Vec<T>>> for &SerialImageBuffer<T> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<LumaA<T>, Vec<T>>, Self::Error> {
        if self.data.pixel_elems != 2 {
            return Err("Image must have two elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<LumaA<T>, Vec<T>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.clone().unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl TryInto<ImageBuffer<Rgb<u8>, Vec<u8>>> for SerialImageBuffer<u8> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Self::Error> {
        if self.data.pixel_elems != 3 {
            return Err("Image must have three elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl TryInto<ImageBuffer<Rgb<u8>, Vec<u8>>> for &SerialImageBuffer<u8> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Self::Error> {
        if self.data.pixel_elems != 3 {
            return Err("Image must have three elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.clone().unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl TryInto<ImageBuffer<Rgb<u16>, Vec<u16>>> for SerialImageBuffer<u16> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Rgb<u16>, Vec<u16>>, Self::Error> {
        if self.data.pixel_elems != 3 {
            return Err("Image must have three elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl TryInto<ImageBuffer<Rgb<u16>, Vec<u16>>> for &SerialImageBuffer<u16> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Rgb<u16>, Vec<u16>>, Self::Error> {
        if self.data.pixel_elems != 3 {
            return Err("Image must have three elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.clone().unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl TryInto<ImageBuffer<Rgb<f32>, Vec<f32>>> for SerialImageBuffer<f32> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Rgb<f32>, Vec<f32>>, Self::Error> {
        if self.data.pixel_elems != 3 {
            return Err("Image must have three elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl TryInto<ImageBuffer<Rgb<f32>, Vec<f32>>> for &SerialImageBuffer<f32> {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageBuffer<Rgb<f32>, Vec<f32>>, Self::Error> {
        if self.data.pixel_elems != 3 {
            return Err("Image must have three elements per pixel");
        }

        if self.width * self.height == 0 {
            return Err("Image must have non-zero dimensions");
        }

        let img = ImageBuffer::<Rgb<f32>, Vec<f32>>::from_raw(
            self.width as u32,
            self.height as u32,
            self.data.luma.clone().unwrap(),
        );
        if img.is_none() {
            return Err("Failed to convert to image buffer");
        }
        Ok(img.unwrap())
    }
}

impl<T: Primitive> From<ImageBuffer<Luma<T>, Vec<T>>> for SerialImageBuffer<T> {
    fn from(img: ImageBuffer<Luma<T>, Vec<T>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 1;
        let data = img.into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl<T: Primitive> From<&ImageBuffer<Luma<T>, Vec<T>>> for SerialImageBuffer<T> {
    fn from(img: &ImageBuffer<Luma<T>, Vec<T>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 1;
        let data = img.clone().into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl<T: Primitive> From<ImageBuffer<LumaA<T>, Vec<T>>> for SerialImageBuffer<T> {
    fn from(img: ImageBuffer<LumaA<T>, Vec<T>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 2;
        let data = img.into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl<T: Primitive> From<&ImageBuffer<LumaA<T>, Vec<T>>> for SerialImageBuffer<T> {
    fn from(img: &ImageBuffer<LumaA<T>, Vec<T>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 2;
        let data = img.clone().into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl From<ImageBuffer<Rgb<u8>, Vec<u8>>> for SerialImageBuffer<u8> {
    fn from(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 3;
        let data = img.into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl From<&ImageBuffer<Rgb<u8>, Vec<u8>>> for SerialImageBuffer<u8> {
    fn from(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 3;
        let data = img.clone().into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl From<ImageBuffer<Rgb<u16>, Vec<u16>>> for SerialImageBuffer<u16> {
    fn from(img: ImageBuffer<Rgb<u16>, Vec<u16>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 3;
        let data = img.into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl From<&ImageBuffer<Rgb<u16>, Vec<u16>>> for SerialImageBuffer<u16> {
    fn from(img: &ImageBuffer<Rgb<u16>, Vec<u16>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 3;
        let data = img.clone().into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl From<ImageBuffer<Rgb<f32>, Vec<f32>>> for SerialImageBuffer<f32> {
    fn from(img: ImageBuffer<Rgb<f32>, Vec<f32>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 3;
        let data = img.clone().into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}

impl From<&ImageBuffer<Rgb<f32>, Vec<f32>>> for SerialImageBuffer<f32> {
    fn from(img: &ImageBuffer<Rgb<f32>, Vec<f32>>) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixel_elems = 3;
        let data = img.clone().into_raw();
        let (luma, red, green, blue, alpha) =
            Self::from_vec_unsafe(width * height, data, pixel_elems);
        Self {
            meta: None,
            data: SerialImageInternal {
                luma,
                red,
                green,
                blue,
                alpha,
                pixel_elems,
            },
            width,
            height,
        }
    }
}
