use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    a: Direction,
    b: Direction,
}

#[derive(Debug, PartialEq, Eq)]
enum TileItem {
    Pipe(Pipe),
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
struct Tile {
    position: (u32, u32),
    item: TileItem,
}

type Grid = BTreeMap<(u32, u32), Tile>;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

fn parse_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe {
            a: Direction::Up,
            b: Direction::Down,
        },
        '-' => Pipe {
            a: Direction::Left,
            b: Direction::Right,
        },
        'L' => Pipe {
            a: Direction::Up,
            b: Direction::Right,
        },
        'J' => Pipe {
            a: Direction::Up,
            b: Direction::Left,
        },
        '7' => Pipe {
            a: Direction::Down,
            b: Direction::Left,
        },
        'F' => Pipe {
            a: Direction::Down,
            b: Direction::Right,
        },
        _ => panic!("Not a pipe character: {}", c),
    }
}

fn parse_tile_item(c: char) -> TileItem {
    match c {
        '|' | '-' | 'L' | 'J' | '7' | 'F' => TileItem::Pipe(parse_pipe(c)),
        '.' => TileItem::Ground,
        'S' => TileItem::Start,
        _ => panic!("Not a valid tile character: {}", c),
    }
}

fn main() {
    println!("Hello, world!");
}

fn part1(input: &str) -> u32 {
    let grid = parse_grid(input);

    grid.iter().for_each(|((y, x), item)| {
        println!("({},{}) => {:?}", y, x, item);
    });

    todo!();
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = BTreeMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let position = (y as u32, x as u32);
            grid.insert(
                position,
                Tile {
                    position,
                    item: parse_tile_item(c),
                },
            );
        })
    });

    grid
}

fn adjacent_tiles(tile: &Tile, grid: &Grid) -> Vec<&Tile> {
    let mut adjacent = Vec::new();

    let (y, x) = tile.position;

    if y > 0 {
        let position = (y - 1, x);
        if let Some(tile) = grid.get(&position) {
            adjacent.push(tile);
        }
    }

    if y < 4 {
        let position = (y + 1, x);
        if let Some(tile) = grid.get(&position) {
            adjacent.push(tile);
        }
    }

    if x > 0 {
        let position = (y, x - 1);
        if let Some(tile) = grid.get(&position) {
            adjacent.push(tile);
        }
    }

    if x < 4 {
        let position = (y, x + 1);
        if let Some(tile) = grid.get(&position) {
            adjacent.push(tile);
        }
    }

    adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grid_works() {
        let input = ".F-";
        // 7.S";
        let grid = parse_grid(input);
        assert_eq!(
            grid,
            BTreeMap::from([
                (
                    (0, 0),
                    Tile {
                        position: (0, 0),
                        item: parse_tile_item('.')
                    }
                ),
                (
                    (0, 1),
                    Tile {
                        position: (0, 1),
                        item: parse_tile_item('F')
                    }
                ),
                (
                    (0, 2),
                    Tile {
                        position: (0, 2),
                        item: parse_tile_item('-')
                    }
                ),
            ])
        )
    }

    #[test]
    fn parse_pipe_works() {
        assert_eq!(
            parse_pipe('|'),
            Pipe {
                a: Direction::Up,
                b: Direction::Down
            }
        );
    }

    #[test]
    fn it_works() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let result = part1(input);
        assert_eq!(result, 8);
    }
}
