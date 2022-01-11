use std::collections::HashMap;
use std::iter;
use std::ops::Deref;

// type Grid = Vec<Vec<i32>>;
#[derive(Debug)]
struct Grid {
    data: Vec<Vec<i32>>,
    size: i32,
}

impl Deref for Grid {
    type Target = Vec<Vec<i32>>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}

fn pairs_of<'a, T>(v: Vec<T>) -> Option<Box<dyn Iterator<Item = (T, T)> + 'a>>
where T: Copy + 'a
{
    let n = v.len();
    if n <= 1 {
        return None;
    }
    let it = (0..=n - 2)
        .map(move |k| iter::repeat(k).zip(k..=n - 1))
        .flat_map(|r| r)
        .map(move |(k1, k2)| (v[k1], v[k2]));
    Some(Box::new(it))
}

impl Grid {
    fn new(data: Vec<Vec<i32>>) -> Grid {
        if data.len() == 0 {
            panic!("Invalid grid shape: empty grid")
        }
        let height = data.len() as i32;
        let width = data[0].len() as i32;
        if !data.iter().all(|row| row.len() as i32 == width) {
            panic!("Invalid grid shape: not all rows have the same length")
        }
        if width != height {
            panic!("Invalid grid shape: must be a square")
        }
        Grid { data, size: width }
    }

    fn pairs_iter_step_i<'a>(&'a self, i: i32) -> Box<dyn Iterator<Item = PointPair> + 'a> {

        if i == 0  || i >= 2 * self.size - 2 {
            panic!("begin at step 1, stop at step 2 * size - 3")
        }
        let k_max = if i <= self.size - 1 {
            i
        } else {
            2 * self.size - 2 - i
        };

        let diag_coords = (0..=k_max).map(move |k| (k, k_max - k));
        let closure: Box<dyn Fn((i32, i32)) -> Point> =
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

    fn steps_i(&self) ->  Box<dyn Iterator<Item = i32>> {
        Box::new(1 .. 2 * self.size - 2)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn from_tuple((x, y): (i32, i32)) -> Self {
        Point(x, y)
    }

    fn from_tuple_sym((x, y): (i32, i32), size: i32) -> Self {
        Point(size - 1 - x, size - 1 - y)
    }

    fn score(&self, grid: &Grid) -> i32 {
        (*grid)[self.0 as usize][self.1 as usize]
    }

    fn is_valid(&self, grid: &Grid) -> bool {
        self.score(grid) >= 0
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point(x, y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PointPair(Point, Point);

impl PointPair {
    // fn from_tuples(x: (i32, i32), y: (i32, i32)) -> Self {
    //     PointPair(Point::from_tuple(x), Point::from_tuple(y))
    // }
    // fn from_tuples_sym(x: (i32, i32), y: (i32, i32), size: i32) -> Self {
    //     PointPair(Point::from_tuple_sym(x, size), Point::from_tuple_sym(y, size))
    // }

    fn zero() -> Self {
        PointPair(Point(0, 0), Point(0, 0))
    }

    fn end(grid: &Grid) -> Self {
        let s = grid.size;
        PointPair(Point(s - 1, s - 1), Point(s - 1, s - 1))
    }

    fn predecessors(&self, grid: &Grid) -> Option<Box<dyn Iterator<Item = PointPair>>> {
        let (p1, p2) = if self.0.0 < self.1.0 {
            (self.0, self.1)
        } else {
            (self.1, self.0)
        };
        let predecessors = if self.1.0 - self.0.0 == 1 {
            vec![(p1.0 - 1, p1.1), (p1.0, p1.1 - 1), (p2.0, p2.1 - 1)]
        } else {
            vec![(p1.0 - 1, p1.1), (p1.0, p1.1 - 1), (p2.0 - 1, p2.1), (p2.0, p2.1 - 1)]
        }.into_iter()
        .map(|t| Point::from_tuple(t))
        .filter(|p @ Point(x, y)| *x >= 0 && *y >= 0 && p.is_valid(grid))
        .collect::<Vec<_>>();
        let predecessors = pairs_of(predecessors);
        if let Some(predecessors) = predecessors {
            let predecessors = predecessors.map(|(p1, p2)| PointPair(p1, p2));
            Some(Box::new(predecessors))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct PairScore {
    score: i32,
    path1: Vec<Point>,
    path2: Vec<Point>,
}

impl PairScore {
    fn start() -> PairScore {
        PairScore {
            score: 0,
            path1: vec![Point(0, 0)],
            path2: vec![Point(0, 0)],
        }
    }

    fn new(score: i32) -> PairScore {
        PairScore {
            score,
            path1: Vec::new(),
            path2: Vec::new(),
        }
    }

    fn append(&self, grid: Grid, p1: Point, p2: Point) -> PairScore {
        let delta = p1.score(&grid) + p2.score(&grid);
        let mut pscore = PairScore::new(self.score + delta);
        pscore.path1.extend(self.path1.iter().chain(iter::once(&p1)));
        pscore.path2.extend(self.path2.iter().chain(iter::once(&p2)));
        pscore
    }
}

type Scores = HashMap<PointPair, PairScore>;

fn cherry_pickup(grid: Vec<Vec<i32>>) -> PairScore {
    let grid = Grid::new(grid);

    let mut scores = Scores::new();
    scores.insert(PointPair::zero(), PairScore::start());

    for i in grid.steps_i() {
        for pair @ PointPair(p1, p2) in grid.pairs_iter_step_i(i) {
            let predecessors = pair.predecessors(&grid);
            if predecessors.is_none() {
                scores.insert(PointPair (p1, p2), PairScore::new(-1));
                continue;
            }
            let predecessors = predecessors.unwrap();
            let predecessors = predecessors.collect::<Vec<_>>();
            println!("{:?}", predecessors);
            let (best_predecessors, best_pairscore) = predecessors.iter()
                .map(|ppair| (ppair, &scores[&ppair]))
                .max_by_key(|(ppair, pairscore)| pairscore.score)
                .unwrap();
            let score = best_pairscore.score + p1.score(&grid) + p2.score(&grid);
            let mut path1 = best_pairscore.path1.clone();
            let mut path2 = best_pairscore.path2.clone();
            path1.push(p1.clone());
            path2.push(p2.clone());
            scores.insert(PointPair (p1, p2), PairScore{score, path1, path2});
        }
    }
    scores[&PointPair::end(&grid)].clone()
}


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diag_3() {
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let grid = Grid::new(grid);
        println!("Grid size is 3\n");
        for i in grid.steps_i() {
            println!("i = {}", i);
            grid.pairs_iter_step_i(i).for_each(|x| println!("{:?}", x));
        }
    }

    #[test]
    fn diag_4() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let grid = Grid::new(grid);
        println!("Grid size is 4\n");
        for i in grid.steps_i() {
            println!("i = {}", i);
            grid.pairs_iter_step_i(i).for_each(|x| println!("{:?}", x));
        }
    }

    #[test]
    fn solve_4() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
        ];
        let grid = Grid::new(grid);
        println!("Grid size is 4\n");
        println!("{:?}", cherry_pickup(grid.data));
    }
}

