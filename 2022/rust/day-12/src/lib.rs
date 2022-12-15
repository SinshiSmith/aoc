use std::{convert::Infallible, fs::File, io::Write, str::FromStr};

use grid::Grid;
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
    prelude::{DiGraphMap, UnGraphMap},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    row: usize,
    col: usize,
}

struct PossibleMove {
    row: isize,
    col: isize,
}

impl PossibleMove {
    fn try_into_point(&self, max_rows: usize, max_cols: usize) -> Option<Point> {
        match self.row >= 0
            && self.col >= 0
            && self.col <= max_cols as isize
            && self.row <= max_rows as isize
        {
            true => Some(Point {
                col: self.col as usize,
                row: self.row as usize,
            }),
            false => None,
        }
    }
}

struct MapSetup {
    start: Point,
    end: Point,
    line_width: usize,
}

impl FromStr for MapSetup {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        Ok(MapSetup {
            start: get_coord(s, 'S'),
            end: get_coord(s, 'E'),
            line_width: s.lines().next().unwrap().len(),
        })
    }
}

fn get_coord(input: &str, point: char) -> Point {
    let (row, point_line) = input
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains(point))
        .unwrap();
    let col = point_line.find(|char| char == point).unwrap();
    Point { col, row }
}

pub fn part_1(input: &str) -> usize {
    let setup = input.parse::<MapSetup>().unwrap();
    let grid = Grid::from_vec(
        input
            .lines()
            .flat_map(|line| {
                line.chars().map(|char| {
                    let mut char = char;
                    if char == 'S' {
                        char = 'a';
                    }
                    if char == 'E' {
                        char = 'z';
                    }
                    char as u8
                })
            })
            .collect::<Vec<u8>>(),
        setup.line_width,
    );
    let mut edges = vec![];
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let current_node = grid.get(row, col).unwrap();
            [
                PossibleMove {
                    col: col as isize - 1,
                    row: row as isize,
                },
                PossibleMove {
                    col: col as isize + 1,
                    row: row as isize,
                },
                PossibleMove {
                    col: col as isize,
                    row: row as isize - 1,
                },
                PossibleMove {
                    col: col as isize,
                    row: row as isize + 1,
                },
            ]
            .iter()
            .filter_map(|possible_move| {
                possible_move.try_into_point(grid.rows() - 1, grid.cols() - 1)
            })
            .filter_map(|point| {
                let point_node = grid.get(point.row, point.col)?;
                if current_node + 1 >= *point_node {
                    Some(((col, row), (point.col, point.row)))
                } else {
                    None
                }
            })
            .for_each(|link| {
                edges.push(link);
            });
        }
    }
    let gr = DiGraphMap::<_, ()>::from_edges(&edges);
    let path = dijkstra(
        &gr,
        (setup.start.col, setup.start.row),
        Some((setup.end.col, setup.end.row)),
        |_| 1,
    );
    path[&(setup.end.col, setup.end.row)]
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn shortest_path() {
        assert_eq!(part_1(INPUT), 31);
    }
}
