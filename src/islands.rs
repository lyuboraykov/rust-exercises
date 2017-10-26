const N: i32 = 10;

pub struct Territory {
    territory: [[u32; N as usize]; N as usize],
    islands_count: i32,
    id: u32
}

const OFFSETS: [(i32, i32); 4] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0)
];

const SEA: u32 = 0;

impl Territory {

    pub fn new() -> Territory {
        return Territory {
            islands_count: 0,
            id: 0,
            territory: [[0; N as usize]; N as usize]
        };
    }

    pub fn add_land(& mut self, r: i32, c: i32) -> i32 {
        let mut different_neighbours: i32 = 0;

        for offset in OFFSETS.iter() {
            // relies on undeflow
            let new_coordinates = ((r + offset.0) as usize, (c + offset.1) as usize);
            let upper_bound = (N - 1) as usize;
            let are_coordinates_valid = new_coordinates.0 < upper_bound && new_coordinates.1 < upper_bound;
            if are_coordinates_valid && self.territory[new_coordinates.0][new_coordinates.1] != SEA {
                different_neighbours += 1;
            }
        }
        let islands_difference = -1 * (different_neighbours - 1);
        self.islands_count += islands_difference;

        // always give the new indexed ground a new id
        self.id += 1;
        self.reindex_islands(r, c, true);

        return self.islands_count;
    }

    fn reindex_islands(&mut self, r: i32, c: i32, initial: bool) {
        let current_el = self.territory[r as usize][c as usize];
        if (current_el == SEA || current_el == self.id) && !initial {
            return;
        }

        self.territory[r as usize][c as usize] = self.id;

        // TODO: make some sort of iterator to avoid code duplication
        for offset in OFFSETS.iter() {
            let new_coordinates = ((r + offset.0) as usize, (c + offset.1) as usize);
            let upper_bound = (N - 1) as usize;
            let are_coordinates_valid = new_coordinates.0 < upper_bound && new_coordinates.1 < upper_bound;
            if are_coordinates_valid {
                self.reindex_islands(new_coordinates.0 as i32, new_coordinates.1 as i32, false);
            }
        }
    }
}
