use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use super::geometry::{Bounds2, Point2, Vector2};

#[derive(Clone, Debug)]
pub struct Grid<V> {
    pub map: HashMap<Point2, V>,
    pub bounds: Bounds2,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn cardinals() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn ordinals() -> [Direction; 4] {
        [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
    }

    pub fn all() -> [Direction; 8] {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }

    pub fn offset(&self) -> Vector2 {
        let (x, y) = match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
            Direction::NorthWest => (-1, -1),
        };
        Vector2 { x, y }
    }
}

impl<V: Clone> Grid<V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            bounds: Bounds2::new(),
        }
    }

    pub fn find<P>(&self, predicate: P) -> Option<(Point2, V)>
    where
        P: Fn(&V) -> bool,
    {
        self.map
            .iter()
            .skip_while(|x| !predicate(x.1))
            .map(|x| (*x.0, x.1.clone()))
            .next()
    }

    pub fn row(&self, y: isize) -> Vec<(Point2, &V)> {
        self.map
            .keys()
            .filter(|k| k.y == y)
            .map(|k| (*k, self.map.get(k).unwrap()))
            .sorted_by_key(|(p, _)| p.x)
            .collect()
    }

    pub fn col(&self, x: isize) -> Vec<(Point2, &V)> {
        self.map
            .keys()
            .filter(|k| k.x == x)
            .map(|k| (*k, self.map.get(k).unwrap()))
            .sorted_by_key(|(p, _)| p.y)
            .collect()
    }

    pub fn line_starting_from<'a>(
        &'a self,
        p: Point2,
        dir: Direction,
    ) -> impl Iterator<Item = &V> + 'a {
        let offset = dir.offset();
        (0..)
            .into_iter()
            .map(move |n| p + (offset * n))
            .map(|coord| self.map.get(&coord))
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
    }
}

impl<V> FromIterator<(Point2, V)> for Grid<V> {
    fn from_iter<T: IntoIterator<Item = (Point2, V)>>(iter: T) -> Self {
        let mut map = HashMap::new();
        let mut bounds = Bounds2::new();

        for (p, i) in iter.into_iter() {
            map.insert(p, i);
            bounds = bounds.union_point(p);
        }

        Self { map, bounds }
    }
}

// impl<V, I> FromIterator<I> for Grid<V>
// where
//     I: IntoIterator<Item = V>,
// {
//     fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
//         todo!()
//     }
// }

// pub struct Line;

// impl FromIterator<

impl<V: Display> Display for Grid<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = &self.map;
        if map.is_empty() {
            writeln!(f, "map is empty")?;
            return Ok(());
        }

        for y in self.bounds.min.y..self.bounds.max.y {
            for x in self.bounds.min.x..self.bounds.max.x {
                let p = Point2 { x, y };
                if let Some(v) = self.map.get(&p) {
                    write!(f, "{}", v)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
