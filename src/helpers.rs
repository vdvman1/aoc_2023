use std::{ops::Index, str::FromStr};

pub fn try_ascii_to_digit(c: u8) -> Option<u32> {
    let digit = c.wrapping_sub(b'0');
    if digit > 9 {
        None
    } else {
        Some(digit as u32)
    }
}

pub fn ascii_to_digit(c: u8) -> u32 {
    (c - b'0').into()
}

pub fn parse_vec<T: FromStr>(input: &str, delimiter: &str) -> Vec<T> {
    input.split(delimiter).flat_map(str::parse).collect()
}

/// Simple 2d grid of elements
///
/// Doesn't have all the checking that a good implementation should have
pub struct Vec2d<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Vec2d<T> {
    pub fn new() -> Self {
        Vec2d {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn iter(&self) -> Vec2dIter<T> {
        Vec2dIter {
            x: 0,
            y: 0,
            width: self.width,
            iter: self.data.iter(),
        }
    }

    /// Adds a new row to the [Vec2d<T>]
    ///
    /// WARNING: This doesn't check that the iterator produces the correct number of elements
    pub fn add_row<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
        if self.width == 0 {
            self.width = self.data.len();
        }
        self.height += 1;
    }

    pub fn rows(&self) -> std::slice::Chunks<T> {
        self.data.chunks(self.width)
    }

    pub fn above_left(&self, x: usize, y: usize) -> Option<&T> {
        if x == 0 || y == 0 {
            None
        } else {
            Some(&self[(x - 1, y - 1)])
        }
    }

    pub fn above(&self, x: usize, y: usize) -> Option<&T> {
        if y == 0 {
            None
        } else {
            Some(&self[(x, y - 1)])
        }
    }

    pub fn above_right(&self, x: usize, y: usize) -> Option<&T> {
        if x == self.width - 1 || y == 0 {
            None
        } else {
            Some(&self[(x + 1, y - 1)])
        }
    }

    pub fn left(&self, x: usize, y: usize) -> Option<&T> {
        if x == 0 {
            None
        } else {
            Some(&self[(x - 1, y)])
        }
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        &self[(x, y)]
    }

    pub fn right(&self, x: usize, y: usize) -> Option<&T> {
        if x == self.width - 1 {
            None
        } else {
            Some(&self[(x + 1, y)])
        }
    }

    pub fn below_left(&self, x: usize, y: usize) -> Option<&T> {
        if x == 0 || y == self.height - 1 {
            None
        } else {
            Some(&self[(x - 1, y + 1)])
        }
    }

    pub fn below(&self, x: usize, y: usize) -> Option<&T> {
        if y == self.height - 1 {
            None
        } else {
            Some(&self[(x, y + 1)])
        }
    }

    pub fn below_right(&self, x: usize, y: usize) -> Option<&T> {
        if x == self.width - 1 || y == self.height - 1 {
            None
        } else {
            Some(&self[(x + 1, y + 1)])
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;

        &self.data[self.width * y + x]
    }
}

impl<T> Default for Vec2d<T> {
    fn default() -> Self {
        Vec2d::new()
    }
}

impl<'a, T> IntoIterator for &'a Vec2d<T> {
    type Item = (usize, usize, &'a T);
    type IntoIter = Vec2dIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Vec2dIter<'a, T> {
    x: usize,
    y: usize,
    width: usize,
    iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Vec2dIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let x = self.x;
        let y = self.y;

        self.x += 1;
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }

        Some((x, y, item))
    }
}
