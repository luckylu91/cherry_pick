use std::ops::Deref;
use std::iter;
use crate::pair_iter::pairs_of;
use itertools::Itertools;

#[derive(Debug)]
pub struct Grid {
    pub data: Vec<Vec<i32>>,
    pub size: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub usize, pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PointPair(pub Point, pub Point);


impl Deref for Grid {
    type Target = Vec<Vec<i32>>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}

impl Grid {
    pub fn new(data: Vec<Vec<i32>>) -> Grid {
        if data.len() == 0 {
            panic!("Invalid grid shape: empty grid")
        }
        let height = data.len();
        let width = data[0].len();
        if !data.iter().all(|row| row.len() == width) {
            panic!("Invalid grid shape: not all rows have the same length")
        }
        if width != height {
            panic!("Invalid grid shape: must be a square")
        }
        Grid { data, size: width }
    }

    pub fn pairs_iter_step_i<'a>(&'a self, i: usize) -> Box<dyn Iterator<Item = PointPair> + 'a> {

        if i == 0  || i >= 2 * self.size - 2 {
            panic!("begin at step 1, stop at step 2 * size - 3")
        }
        let k_max = if i <= self.size - 1 {
            i
        } else {
            2 * self.size - 2 - i
        };

        let diag_coords = (0..=k_max).map(move |k| (k, k_max - k));
        let closure: Box<dyn Fn((usize, usize)) -> Point> =
            if i <= self.size - 1 {
                Box::new(|p| Point::from_tuple(p))
            } else {
                Box::new(|p| Point::from_tuple_sym(p, self.size))
            };
        let diag_coords = diag_coords.map(closure)
            .filter(|p| p.is_valid(&self))
            .collect::<Vec<Point>>();
        let n_valid = diag_coords.len();
        let diag_pairs_coords = pairs_of(diag_coords).unwrap()
            .map(|(p1, p2)| PointPair(p1, p2));
        Box::new(diag_pairs_coords)
    }

    pub fn steps_i(&self) ->  Box<dyn Iterator<Item = usize>> {
        Box::new(1 .. 2 * self.size - 2)
    }
}


impl Point {
    pub fn from_tuple((x, y): (usize, usize)) -> Self {
        Point(x, y)
    }

    pub fn from_tuple_sym((x, y): (usize, usize), size: usize) -> Self {
        Point(size - 1 - x, size - 1 - y)
    }

    pub fn score(&self, grid: &Grid) -> i32 {
        (*grid)[self.0][self.1]
    }

    pub fn is_valid(&self, grid: &Grid) -> bool {
        self.score(grid) >= 0
    }

    pub fn valid_predecessors(&self, grid: &Grid) -> Vec<Point> {
        let mut result = Vec::new();
        result.reserve(2);
        if self.0 > 0 && grid[self.0 - 1][self.1] != -1 {
            result.push(Point(self.0 - 1, self.1));
        }
        if self.1 > 0 && grid[self.0][self.1 - 1] != -1 {
            result.push(Point(self.0, self.1 - 1));
        }
        result
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point(x, y)
    }
}

impl PointPair {
    pub fn zero() -> Self {
        PointPair(Point(0, 0), Point(0, 0))
    }

    pub fn end(grid: &Grid) -> Self {
        let s = grid.size;
        PointPair(Point(s - 1, s - 1), Point(s - 1, s - 1))
    }

    pub fn predecessors<'a>(&self, grid: &'a Grid) -> Box<dyn Iterator<Item = PointPair> + 'a> {
        // let (p1, p2) = if self.0.0 < self.1.0 {
        //     (self.0, self.1)
        // } else {
        //     (self.1, self.0)
        // };
        let pred1 = self.0.valid_predecessors(grid);
        let pred2 = self.1.valid_predecessors(grid);
        Box::new(pred1.into_iter().cartesian_product(pred2.into_iter()).map(|(p1, p2)| PointPair(p1, p2)))
        // let predecessors = pred1.iter()
        //     .map(move |p1| iter::repeat(p1).zip(p2.valid_predecessors(grid).iter()).cloned().collect::<Vec<(Point, Point)>>())
        //     .flat_map(|x| x)
        //     .map(|(p1, p2)| PointPair(p1.clone(), p2.clone()));
        // Box::new(predecessors)
        // let predecessors = if self.1.0 - self.0.0 == 1 {
        //     vec![(p1.0 - 1, p1.1), (p1.0, p1.1 - 1), (p2.0, p2.1 - 1)]
        // } else {
        //     vec![(p1.0 - 1, p1.1), (p1.0, p1.1 - 1), (p2.0 - 1, p2.1), (p2.0, p2.1 - 1)]
        // }.into_iter()
        //     .map(|t| Point::from_tuple(t))
        //     .filter(|p @ Point(x, y)| *x >= 0 && *y >= 0 && p.is_valid(grid))
        //     .collect::<Vec<_>>();
        // let predecessors = pairs_of(predecessors);
        // if let Some(predecessors) = predecessors {
        //     let predecessors = predecessors.map(|(p1, p2)| PointPair(p1, p2));
        //     Some(Box::new(predecessors))
        // } else {
        //     None
        // }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_pred() {
        let grid = Grid::new(
            vec![
                vec![0, 0],
                vec![-1, 0],
            ]
        );
        let p = Point(0, 0);
        let v: Vec<Point> = p.valid_predecessors(&grid);
        assert_eq!(v, vec![]);
        let p = Point(0, 1);
        let v: Vec<Point> = p.valid_predecessors(&grid);
        assert_eq!(v, vec![Point(0, 0)]);
    }


    #[test]
    fn point_pair_pred() {
        let grid = Grid::new(
            vec![
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![-1, 0, 0],
            ]
        );
        let ppair = PointPair(Point(0, 2), Point(1, 1));
        let v: Vec<PointPair> = ppair.predecessors(&grid).collect();
        println!("{:?}", v);
        let ppair = PointPair(Point(1, 2), Point(2, 1));
        let v: Vec<PointPair> = ppair.predecessors(&grid).collect();
        println!("{:?}", v);
    }
}


