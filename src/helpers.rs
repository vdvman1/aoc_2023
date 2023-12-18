use std::{
    borrow::Borrow,
    ops::{Add, Index, RangeInclusive, Sub},
    str::FromStr,
};

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

pub fn get_bytes_len<const START: usize, const LEN: usize>(input: &str) -> [u8; LEN] {
    input[START..(START + LEN)].as_bytes().try_into().unwrap()
}

// TODO: Implement gcd for more types generically

/// Calculate the greatest common divisor
///
/// Uses [Stein's algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm)
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 || b == 0 {
        return a | b;
    }

    // gcd(2u, 2v) = 2*gcd(u, v)
    // trailing_zeros effectivelly gives us the greatest power of 2, as an amount to shift
    let greatest_common_pow2_shift = (a | b).trailing_zeros();

    // gcd(2u, v) = gcd(u, v) if v is odd
    // gcd(u, 2v) = gcd(u, v) if u is odd
    // Divide out greatest power of 2 for each to get both to be odd
    // Includes greatest_common_pow2_shift automatically to make at least one odd before applying the odd vs even rule
    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    // Both a and b are guaranteed to be odd throughout this loop
    while a != b {
        if a < b {
            // Ensure difference of a and b will be positive
            std::mem::swap(&mut a, &mut b);
        }

        // gcd(a, b) = gcd(a-b, b) if both a and b are odd
        a -= b;

        // gcd(2u, b) = gcd(u, b) if b is odd
        // Divide out greatest power of 2 to ensure a is odd
        a >>= a.trailing_zeros();
        // b is already odd
    }

    a << greatest_common_pow2_shift
}

pub fn lcm(a: usize, b: usize) -> usize {
    // Doing the division before the multiplication avoids overflow in the intermediate calculation
    // The division is guaranteed to be an integer since the gcd inherently divides b
    a * (b / gcd(a, b))
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

#[derive(Default)]
pub struct RangeMap<K: Ord + Copy, V> {
    entries: Vec<(RangeInclusive<K>, V)>,
}

impl<K: Ord + Copy, V> RangeMap<K, V> {
    pub fn new() -> Self {
        RangeMap {
            entries: Vec::new(),
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = (RangeInclusive<K>, V)>) -> Self {
        let mut map = RangeMap {
            entries: Vec::from_iter(iter),
        };

        map.entries
            .sort_unstable_by_key(|(range, _)| *range.start());

        map
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get_range(key).map(|(_, value)| value)
    }

    pub fn get_range<Q>(&self, key: &Q) -> Option<(&RangeInclusive<K>, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        for (range, value) in &self.entries {
            if key < range.start().borrow() {
                // larger than previous range, and smaller than this range
                return None;
            }

            if key <= range.end().borrow() {
                // in this range
                return Some((range, value));
            }
        }

        // Larger than all ranges, or no ranges exist
        None
    }
}

impl<T: Ord + Copy> RangeMap<T, T> {
    pub fn get_or_key<'a, 'b, Q>(&'a self, key: &'b Q) -> &'b Q
    where
        'a: 'b,
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.get(key).map(Borrow::borrow).unwrap_or(key)
    }

    pub fn get_and_offset_or_key(&self, key: T) -> T
    where
        T: Sub<T, Output = T> + Add<T, Output = T>,
    {
        match self.get_range(&key) {
            Some((range, value)) => key - *range.start() + *value,
            None => key,
        }
    }
}
