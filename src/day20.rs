#![allow(unused)]

use anyhow::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const SIZE_TILES: usize = 12;

pub fn solve() {
    let input = include_str!("../input/day20");

    let tiles: Vec<Tile> = input
        .trim()
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(tiles.len(), SIZE_TILES * SIZE_TILES);

    println!("Part 2: {}", part2(&tiles));
}

fn part2(tiles: &[Tile]) -> u64 {
    let mut border_to_tile_map: HashMap<BorderId, Vec<TileId>> = HashMap::new();

    for tile in tiles {
        for &border in &tile.borders {
            border_to_tile_map.entry(border).or_default().push(tile.id);
        }
    }

    let is_edge = |border| {
        border_to_tile_map[&border].len() == 1
    };

    let mut grid = Vec::new();
    let mut row = Vec::new();

    //------------------------------------------------------------------------
    // Find a corner and put it at the start of the row

    let mut corner = None;

    for tile in tiles {
        let mut edges = 0;

        for &border in &tile.borders {
            if is_edge(border) {
                edges += 1;
            }
        }

        assert!(edges <= 2);

        if edges == 2 {
            corner = Some(tile);
            break;
        }
    }

    let mut corner = TransformedTile::from_tile(corner.expect("Could not find a corner").clone());

    corner.flip_h();

    while !is_edge(corner.left_border()) || !is_edge(corner.top_border()) {
        corner.rotate();
    }

    row.push(corner);

    //------------------------------------------------------------------------
    // Connect up the pieces in the first row

    for c in 1..SIZE_TILES {
        let border = row[c - 1].right_border();
        let prev_id = row[c - 1].tile.id;

        let mut piece = None;

        for tile in tiles {
            if tile.borders.contains(&border) && tile.id != prev_id {
                piece = Some(tile);
                break;
            }
        }

        let mut piece = TransformedTile::from_tile(piece.expect("Could not find connecting piece").clone());

        while piece.left_border() != border {
            piece.rotate();
        }

        if !is_edge(piece.top_border()) {
            piece.flip_v();
        }

        row.push(piece);
    }

    grid.push(row);

    //------------------------------------------------------------------------
    // Connect up the pieces in the remaining rows

    for r in 1..SIZE_TILES {
        row = Vec::new();

        for c in 0..SIZE_TILES {
            let border = grid[r - 1][c].bottom_border();
            let prev_id = grid[r - 1][c].tile.id;

            let mut piece = None;

            for tile in tiles {
                if tile.borders.contains(&border) && tile.id != prev_id {
                    piece = Some(tile);
                    break;
                }
            }

            let mut piece = TransformedTile::from_tile(piece.expect("Could not find connecting piece").clone());

            while piece.top_border() != border {
                piece.rotate();
            }

            if c == 0 {
                if !is_edge(piece.left_border()) {
                    piece.flip_h();
                    assert!(is_edge(piece.left_border()));
                }
            } else {
                let border = row[c - 1].right_border();
                if piece.left_border() != border {
                    piece.flip_h();
                    assert!(piece.left_border() == border);
                }
            }

            row.push(piece);
        }

        grid.push(row);
    }

    //------------------------------------------------------------------------
    // Check that it is assembled correctly

    for i in 0..SIZE_TILES {
        assert!(is_edge(grid[0][i].top_border()));
        assert!(is_edge(grid[i][SIZE_TILES - 1].right_border()));
        assert!(is_edge(grid[SIZE_TILES - 1][i].bottom_border()));
        assert!(is_edge(grid[i][0].left_border()));
    }

    for r in 0..(SIZE_TILES - 1) {
        for c in 0..(SIZE_TILES - 1) {
            assert_eq!(grid[r][c].right_border(), grid[r][c + 1].left_border());
            assert_eq!(grid[r][c].bottom_border(), grid[r + 1][c].top_border());
        }
    }

    //------------------------------------------------------------------------
    // Make the full image

    let mut image = vec![vec![false; 8 * SIZE_TILES]; 8 * SIZE_TILES];

    for r in 0..(8*SIZE_TILES) {
        for c in 0..(8*SIZE_TILES) {
            image[r][c] = grid[r / 8][c / 8].sample(r % 8, c % 8);
        }
    }

    //------------------------------------------------------------------------
    // Search for monsters

    //           1111111111
    // 01234567890123456789
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #

    let monster = vec![
        (0, 18),
        (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
        (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
    ];

    for &flipped in &[true, false] {
        for rotation in 0..4 {
            let mut monster_count = 0;
            let new_image = transform_image(&image, rotation, flipped);
            let mut monster_coords = HashSet::new();

            for mut r in 0..(8 * SIZE_TILES - 2) {
                for mut c in 0..(8 * SIZE_TILES - 19) {
                    let mut is_monster = true;

                    for (dr, dc) in &monster {
                        if !new_image[r + dr][c + dc] {
                            is_monster = false;
                            break;
                        }
                    }

                    if is_monster {
                        for (dr, dc) in &monster {
                            monster_coords.insert((r + dr, c + dc));
                        }
                    }
                }
            }

            if !monster_coords.is_empty() {
                let mut roughness = 0;

                for mut r in 0..(8 * SIZE_TILES) {
                    for mut c in 0..(8 * SIZE_TILES) {
                        if new_image[r][c] && !monster_coords.contains(&(r, c)) {
                            roughness += 1;
                        }
                    }
                }

                return roughness;
            }
        }
    }

    0
}

fn transform_image(image: &Vec<Vec<bool>>, rotation: u32, flipped: bool) -> Vec<Vec<bool>> {
    let mut new_image = vec![vec![false; 8 * SIZE_TILES]; 8 * SIZE_TILES];

    for r in 0..(8 * SIZE_TILES) {
        for c in 0..(8 * SIZE_TILES) {
            let sample = image[r][c];

            let mut r = r;
            let mut c = c;

            for _ in 0..rotation {
                std::mem::swap(&mut r, &mut c);
                r = 8 * SIZE_TILES - 1 - r;
            }

            if flipped {
                c = 8 * SIZE_TILES - 1 - c;
            }

            new_image[r][c] = sample;
        }
    }

    new_image
}

type BorderId = u32;
type TileId = u64;

#[derive(Debug, Clone)]
struct Tile {
    id: TileId,
    borders: [BorderId; 4],
    image: Vec<Vec<bool>>,
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Tile, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Tile (\d+):").unwrap();
        }

        let id = RE.captures(s).unwrap().get(1).unwrap().as_str().parse()?;

        let data: Vec<Vec<char>> = s
            .trim()
            .lines()
            .skip(1)
            .map(|line| line.chars().collect())
            .collect();

        let size = data.len();

        let mut borders = [0; 4];

        fn to_bit(c: char) -> BorderId {
            match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid square"),
            }
        }

        for i in 0..size {
            let top_square = to_bit(data[0][i]);
            let right_square = to_bit(data[i][size - 1]);
            let bottom_square = to_bit(data[size - 1][size - 1 - i]);
            let left_square = to_bit(data[size - 1 - i][0]);

            borders[0] = (borders[0] << 1) | top_square;
            borders[1] = (borders[1] << 1) | right_square;
            borders[2] = (borders[2] << 1) | bottom_square;
            borders[3] = (borders[3] << 1) | left_square;
        }

        let flip_bits = |mut n| {
            let mut b = 0;

            for _ in 0..size {
                b = (b << 1) | (n & 1);
                n >>= 1;
            }

            b
        };

        // The borders are unique even under reflection, so we can normalise
        // the IDs
        for i in 0..4 {
            borders[i] = BorderId::min(borders[i], flip_bits(borders[i]));
        }

        let mut image = vec![vec![false; size - 2]; size - 2];

        for r in 0..(size - 2) {
            for c in 0..(size - 2) {
                image[r][c] = data[r + 1][c + 1] == '#';
            }
        }

        Ok(Tile {
            id,
            borders,
            image,
        })
    }
}

#[derive(Debug)]
struct TransformedTile {
    tile: Tile,
    flipped: bool,
    rotation: usize,
}

impl TransformedTile {
    fn from_tile(tile: Tile) -> TransformedTile {
        TransformedTile {
            tile, flipped: false, rotation: 0,
        }
    }

    fn flip_h(&mut self) {
        self.flipped = !self.flipped;
    }

    fn flip_v(&mut self) {
        self.flip_h();
        self.rotate();
        self.rotate();
    }

    fn rotate(&mut self) {
        self.rotation += 1;
    }

    fn has_border(&self, border: BorderId) -> bool {
        self.tile.borders.contains(&border)
    }

    fn top_border(&self) -> BorderId {
        self.tile.borders[(0 + self.rotation) % 4]
    }

    fn right_border(&self) -> BorderId {
        if self.flipped {
            self.tile.borders[(3 + self.rotation) % 4]
        } else {
            self.tile.borders[(1 + self.rotation) % 4]
        }
    }

    fn bottom_border(&self) -> BorderId {
        self.tile.borders[(2 + self.rotation) % 4]
    }

    fn left_border(&self) -> BorderId {
        if self.flipped {
            self.tile.borders[(1 + self.rotation) % 4]
        } else {
            self.tile.borders[(3 + self.rotation) % 4]
        }
    }

    fn sample(&self, r: usize, c: usize) -> bool {
        let (mut r, mut c) = (r, c);

        if self.flipped {
            c = 7 - c;
        }

        for _ in 0..self.rotation {
            std::mem::swap(&mut r, &mut c);
            c = 7 - c;
        }

        self.tile.image[r][c]
    }
}

#[test]
fn test_transforms() {
    let tile = r"Tile 3461:
#.##.#....
...#......
#..##.#...
#...##.#..
..#.####.#
.....#....
##..#....#
#....#....
##.###.#..
..#.#.#.#.".parse().unwrap();


    let mut tt = TransformedTile::from_tile(tile);

    // Front view:          Back view:
    //
    //     +---45----+            +---45----+
    //     |         |            |         |
    //     461      40            40      461
    //     |         |            |         |
    //     +---170---+            +---170---+

    assert_eq!(tt.top_border(), 45);
    assert_eq!(tt.right_border(), 40);
    assert_eq!(tt.bottom_border(), 170);
    assert_eq!(tt.left_border(), 461);

    tt.flip_h();

    assert_eq!(tt.top_border(), 45);
    assert_eq!(tt.right_border(), 461);
    assert_eq!(tt.bottom_border(), 170);
    assert_eq!(tt.left_border(), 40);

    tt.flip_h();
    tt.rotate();

    // Front view:          Back view:
    //
    //     +---40----+            +---40----+
    //     |         |            |         |
    //     45      170            170      45
    //     |         |            |         |
    //     +---461---+            +---461---+

    assert_eq!(tt.top_border(), 40);
    assert_eq!(tt.right_border(), 170);
    assert_eq!(tt.bottom_border(), 461);
    assert_eq!(tt.left_border(), 45);

    tt.flip_h();

    assert_eq!(tt.top_border(), 40);
    assert_eq!(tt.right_border(), 45);
    assert_eq!(tt.bottom_border(), 461);
    assert_eq!(tt.left_border(), 170);

    tt.flip_h();
    tt.rotate();

    // Front view:          Back view:
    //
    //     +---170---+            +---170---+
    //     |         |            |         |
    //     40      461            461      40
    //     |         |            |         |
    //     +---45----+            +---45----+

    assert_eq!(tt.top_border(), 170);
    assert_eq!(tt.right_border(), 461);
    assert_eq!(tt.bottom_border(), 45);
    assert_eq!(tt.left_border(), 40);

    tt.flip_h();

    assert_eq!(tt.top_border(), 170);
    assert_eq!(tt.right_border(), 40);
    assert_eq!(tt.bottom_border(), 45);
    assert_eq!(tt.left_border(), 461);

    tt.flip_h();
    tt.rotate();

    // Front view:          Back view:
    //
    //     +---461---+            +---461---+
    //     |         |            |         |
    //     170      45            45      170
    //     |         |            |         |
    //     +---40----+            +---40----+

    assert_eq!(tt.top_border(), 461);
    assert_eq!(tt.right_border(), 45);
    assert_eq!(tt.bottom_border(), 40);
    assert_eq!(tt.left_border(), 170);

    tt.flip_h();

    assert_eq!(tt.top_border(), 461);
    assert_eq!(tt.right_border(), 170);
    assert_eq!(tt.bottom_border(), 40);
    assert_eq!(tt.left_border(), 45);
}
