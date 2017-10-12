extern crate rand;

use rand::Rng;


const SEA: u8 = 0;
const LAND: u8 = 1;

const VISITED: u8 = 1;

const SIZE: usize = 7;

fn main() {
    // TODO: make the array initializations better, not literals
    // probably some vectors

    let mut map: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    map = initalize_map(map);

    let mut visited: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    let mut islands_count = 0;

    for (i, &row) in map.iter().enumerate() {
        for (j, &el) in row.iter().enumerate() {
            if el == LAND && visited[i][j] != VISITED {
                islands_count += 1;
                traverse_figure(&map, &mut visited, i, j);
            }
        }
    }

    println!("There are {} islands in the field", islands_count);
}

fn initalize_map(mut map: [[u8; SIZE]; SIZE]) -> [[u8; SIZE]; SIZE] {
    for row in map.iter_mut() {
        for el in row.iter_mut() {
            *el = rand::thread_rng().gen_range(0, 2);
        }
    }
    map
}

fn traverse_figure(field: &[[u8; SIZE]], visited: &mut [[u8; SIZE]], r: usize, c: usize) {
    if visited[r][c] == VISITED || field[r][c] == SEA {
        return;
    }
    visited[r][c] = VISITED;

    if r > 0 {
        traverse_figure(field, visited, r - 1, c);
    }
    if r + 1 < SIZE {
        traverse_figure(field, visited, r + 1, c);
    }
    if c > 0 {
        traverse_figure(field, visited, r, c - 1);
    }
    if c + 1 < SIZE {
        traverse_figure(field, visited, r, c + 1);
    }
}
