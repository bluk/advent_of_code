use std::convert::TryFrom;
use std::num;

pub mod error;

use error::Error;

pub fn parse_input(input: &str) -> Result<Vec<u8>, num::ParseIntError> {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u8>())
        .collect::<Result<Vec<u8>, num::ParseIntError>>()
}

pub struct SpaceImg {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

struct SpaceImgIter<'a> {
    layer: usize,
    space_img: &'a SpaceImg,
}

impl<'a> Iterator for SpaceImgIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let layer_size = self.space_img.width * self.space_img.height;
        if layer_size * self.layer < self.space_img.pixels.len() {
            let ret_value = Some(
                &self.space_img.pixels[self.layer * layer_size..(self.layer + 1) * layer_size],
            );
            self.layer += 1;
            ret_value
        } else {
            None
        }
    }
}

impl SpaceImg {
    pub fn new(pixels: Vec<u8>, width: usize, height: usize) -> Self {
        SpaceImg {
            pixels,
            width,
            height,
        }
    }

    fn layers_iter(&self) -> SpaceImgIter {
        SpaceImgIter {
            layer: 0,
            space_img: self,
        }
    }

    fn verification_code(layer: &[u8]) -> Result<u64, Error> {
        let ones_count = u64::try_from(layer.iter().filter(|&x| *x == 1).count())?;
        let twos_count = u64::try_from(layer.iter().filter(|&x| *x == 2).count())?;
        Ok(ones_count * twos_count)
    }

    pub fn verify(&self) -> Result<u64, Error> {
        let (_, verification) = self.layers_iter().try_fold(
            (None, 0),
            |(mut min_zeroes_in_layer, mut verification), layer| {
                let zeroes_count = layer.iter().filter(|&x| *x == 0).count();
                if let Some(existing_zeroes_in_layer) = min_zeroes_in_layer {
                    if zeroes_count < existing_zeroes_in_layer {
                        min_zeroes_in_layer = Some(zeroes_count);
                        verification = match Self::verification_code(layer) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                    }
                } else {
                    min_zeroes_in_layer = Some(zeroes_count);
                    verification = match Self::verification_code(layer) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                }

                Ok((min_zeroes_in_layer, verification))
            },
        )?;

        Ok(verification)
    }

    pub fn compose(&self) -> Result<Vec<u8>, Error> {
        Ok(self
            .layers_iter()
            .fold(vec![2; self.width * self.height], |final_img, layer| {
                layer
                    .iter()
                    .zip(final_img.into_iter())
                    .map(|(l, f)| if f == 2 { *l } else { f })
                    .collect()
            }))
    }
}

pub fn display(pixels: Vec<u8>, width: usize, height: usize) {
    for row in 0..height {
        for col in 0..width {
            let color = pixels[col + row * width];
            match color {
                0 => print!(" "),
                1 => print!("X"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_ex1() {
        let input = parse_input("123456789012").unwrap();
        let img = SpaceImg::new(input, 3, 2);

        let mut layers = img.layers_iter();
        assert_eq!(Some(&vec![1, 2, 3, 4, 5, 6][..]), layers.next());
        assert_eq!(Some(&vec![7, 8, 9, 0, 1, 2][..]), layers.next());
        assert_eq!(None, layers.next());
    }

    #[test]
    fn day8_ex2() {
        let input = parse_input("0222112222120000").unwrap();
        let img = SpaceImg::new(input, 2, 2);

        let composed_img = img.compose().unwrap();

        assert_eq!(composed_img[0], 0);
        assert_eq!(composed_img[1], 1);
        assert_eq!(composed_img[2], 1);
        assert_eq!(composed_img[3], 0);
        assert_eq!(composed_img.len(), 4);
    }
}
