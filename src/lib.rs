pub enum Color {
    RGB(u8, u8, u8)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_can_be_rgb() {
        let rgb_color = Color::RGB(200, 100, 54);
        let Color::RGB(r, g, b) = rgb_color;
        assert_eq!(r, 200);
        assert_eq!(g, 100);
        assert_eq!(b, 54);
    }
}
