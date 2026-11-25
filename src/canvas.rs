use std::fmt;

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

    pub fn set(&mut self, x: i32, y: i32, z: f32, c: char) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let i = y as usize * self.width + x as usize;
        if z < self.depth[i] {
            self.depth[i] = z;
            self.chars[i] = c;
        }
    }

    pub fn clear(&mut self) {
        self.chars.fill(' ');
        self.depth.fill(f32::INFINITY);
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.chars.chunks(self.width) {
            let line: String = row.iter().collect();
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_test() {
        let mut c = Canvas::new(4, 3);
        c.set(1, 1, 5.0, 'f'); // far
        c.set(1, 1, 2.0, 'n'); // near: replaces it
        c.set(1, 1, 9.0, 'x'); // farther: ignored
        assert_eq!(c.chars[1 * 4 + 1], 'n');

        c.set(-1, 0, 1.0, '!'); // off-screen: ignored, no panic
        c.set(4, 0, 1.0, '!');
        assert!(!c.chars.contains(&'!'));
    }
}
