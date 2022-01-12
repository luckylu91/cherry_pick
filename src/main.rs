use std::collections::HashMap;
use std::iter;
use cherrypick::grid::*;
use cherrypick::score::*;



type Scores = HashMap<PointPair, PairScore>;

fn cherry_pickup(grid: Vec<Vec<i32>>) -> PairScore {
    let grid = Grid::new(grid);

    let mut scores = Scores::new();
    scores.insert(PointPair::zero(), PairScore::start());

    for i in grid.steps_i() {
        for pair @ PointPair(p1, p2) in grid.pairs_iter_step_i(i) {
            println!("pair: {}", pair);
            let predecessors = pair.predecessors(&grid);
            if predecessors.len() == 0 {
                scores.insert(pair, PairScore::new(-1));
                continue;
            }
            println!("{:?}", predecessors);
            let (_, best_pairscore) = predecessors.iter()
                .map(|ppair| {
                    println!("Accessing {}", ppair);
                    (ppair, &scores[&ppair])
                })
                .max_by_key(|(_, pairscore)| pairscore.score)
                .unwrap();
            // let score = best_pairscore.clone();
            // score.append
            let score = best_pairscore.score + if p1 != p2 {
                p1.score(&grid) + p2.score(&grid)
            } else {
                p1.score(&grid)
            };
            let mut path1 = best_pairscore.path1.clone();
            let mut path2 = best_pairscore.path2.clone();
            path1.push(p1.clone());
            path2.push(p2.clone());
            scores.insert(PointPair (p1, p2), PairScore{score, path1, path2});
        }
        println!("scores: {:#?}\n", scores);
    }
    scores[&PointPair::end(&grid)].clone()
}


fn main() {
    let grid = vec![
        vec![0, 0, 0, 0],
        vec![0, -1, 1, 0],
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 0],
    ];
    let score = cherry_pickup(grid);
    println!("END:\n{:?}", score);
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

