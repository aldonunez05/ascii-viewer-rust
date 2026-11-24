pub struct Canvas {
    pub width: usize,
    pub height: usize,
    chars: Vec<char>,
    depth: Vec<f32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chars: vec![' '; width * height],
            depth: vec![f32::INFINITY; width * height],
        }
    }
}
