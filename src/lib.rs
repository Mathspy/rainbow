use std::cmp::PartialEq;

#[derive(Debug)]
pub enum Color {
    RGB(u8, u8, u8),
    HSL(u16, u8, u8),
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        match self {
            Color::RGB(self_r, self_g, self_b) => match other {
                Color::RGB(other_r, other_g, other_b) => {
                    self_r == other_r && self_g == other_g && self_b == other_b
                }
                Color::HSL(_, _, _) => rgb_to_hsl(self_r, self_g, self_b) == *other,
            },
            Color::HSL(self_h, self_s, self_l) => match other {
                Color::RGB(other_r, other_g, other_b) => {
                    self == &rgb_to_hsl(other_r, other_g, other_b)
                }
                Color::HSL(other_h, other_s, other_l) => {
                    self_h == other_h && self_s == other_s && self_l == other_l
                }
            },
        }
    }
}

fn rgb_to_hsl(&r: &u8, &g: &u8, &b: &u8) -> Color {
    let scaled_r = r as f32 / 255.0;
    let scaled_g = g as f32 / 255.0;
    let scaled_b = b as f32 / 255.0;

    let max = scaled_r.max(scaled_g).max(scaled_b);
    let min = scaled_r.min(scaled_g).min(scaled_b);

    let chroma = max - min;

    let hue = if chroma == 0.0 {
        0
    } else {
        ((match max {
            _ if max == scaled_r => {
                let segment = (scaled_g - scaled_b) / chroma;

                segment + if segment < 0.0 {
                    360.0 / 60.0
                } else {
                    0.0 / 60.0
                }
            }
            _ if max == scaled_g => {
                let segment = (scaled_b - scaled_r) / chroma;

                segment + 120.0 / 60.0
            }
            _ if max == scaled_b => {
                let segment = (scaled_r - scaled_g) / chroma;

                segment + 240.0 / 60.0
            }
            _ => panic!("Unreachable panic"),
        }) * 60.0)
            .round() as u16
    };

    let lightness = 0.5 * (max + min);

    let saturation = if lightness == 1.0 {
        0.0
    } else {
        chroma / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    Color::HSL(hue, (saturation * 100.0) as u8, (lightness * 100.0) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_can_be_rgb() {
        let rgb_color = Color::RGB(200, 100, 54);
        if let Color::RGB(r, g, b) = rgb_color {
            assert_eq!(r, 200);
            assert_eq!(g, 100);
            assert_eq!(b, 54);
        }
    }

    #[test]
    fn color_can_be_hsl() {
        let hsl_color = Color::HSL(299, 78, 12);
        if let Color::HSL(h, s, l) = hsl_color {
            assert_eq!(h, 299);
            assert_eq!(s, 78);
            assert_eq!(l, 12);
        }
    }

    #[test]
    fn color_eq() {
        assert_eq!(Color::RGB(150, 50, 60), Color::HSL(354, 50, 39));
        assert_eq!(Color::HSL(354, 50, 39), Color::RGB(150, 50, 60));
        assert_eq!(Color::RGB(150, 50, 60), Color::RGB(150, 50, 60));
        assert_eq!(Color::HSL(354, 50, 39), Color::HSL(354, 50, 39));
    }
}
