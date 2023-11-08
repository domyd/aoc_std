use super::{point::Point2, vector::Vector2};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Bounds2 {
    pub min: Point2,
    pub max: Point2,
}

impl Bounds2 {
    pub fn new() -> Self {
        let min = Point2 {
            x: isize::MAX,
            y: isize::MAX,
        };
        let max = Point2 {
            x: isize::MIN,
            y: isize::MIN,
        };
        Self { min, max }
    }

    pub fn around(a: Point2, b: Point2) -> Self {
        Bounds2 {
            min: Point2 {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            max: Point2 {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        }
    }

    pub fn from_point(point: Point2) -> Self {
        Bounds2 {
            min: point,
            max: point,
        }
    }

    pub fn union_point(&self, other: Point2) -> Self {
        Bounds2 {
            min: self.min.min(other),
            max: self.max.max(other),
        }
    }

    pub fn union(&self, other: Self) -> Self {
        Bounds2 {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn intersect(&self, other: Self) -> Self {
        Bounds2 {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    pub fn overlaps(&self, other: Self) -> bool {
        let x = self.max.x >= other.min.x && self.min.x <= other.max.x;
        let y = self.max.y >= other.min.y && self.min.y <= other.max.y;
        x && y
    }

    pub fn contains(&self, point: Point2) -> bool {
        (self.min.x..=self.max.x).contains(&point.x) && (self.min.y..=self.max.y).contains(&point.y)
    }

    pub fn contains_exclusive(&self, point: Point2) -> bool {
        (self.min.x..self.max.x).contains(&point.x) && (self.min.y..self.max.y).contains(&point.y)
    }

    pub fn expand(&self, delta: isize) -> Self {
        let delta = Vector2 { x: delta, y: delta };
        Bounds2 {
            min: self.min - delta,
            max: self.max + delta,
        }
    }

    pub fn diagonal(&self) -> Vector2 {
        self.max - self.min
    }

    pub fn area(&self) -> isize {
        let diagonal = self.diagonal();
        diagonal.x * diagonal.y
    }
}

pub struct Bounds2Iterator {
    bounds: Bounds2,
    p: Point2,
}

impl Iterator for Bounds2Iterator {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(&self.p);

        let next = if self.bounds.contains_exclusive(self.p) {
            Some(Some(self.p))
        } else {
            eprintln!("not contained");
            None
        }?;

        self.p.x += 1;
        if self.p.x == self.bounds.max.x {
            self.p.x = self.bounds.min.x;
            self.p.y += 1;
        }

        next
    }
}

impl IntoIterator for Bounds2 {
    type Item = Point2;

    type IntoIter = Bounds2Iterator;

    fn into_iter(self) -> Self::IntoIter {
        Bounds2Iterator {
            bounds: self,
            p: self.min,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bb2_iter() {
        let bb = Bounds2 {
            min: Point2 { x: 1, y: 2 },
            max: Point2 { x: 4, y: 5 },
        };
        let points = bb.into_iter().collect::<Vec<_>>();
        let expected = [
            (1, 2),
            (2, 2),
            (3, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (1, 4),
            (2, 4),
            (3, 4),
        ]
        .map(|(x, y)| Point2 { x, y });
        assert_eq!(&expected, &points[..]);
    }
}
