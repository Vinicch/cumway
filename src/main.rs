use std::{process::Command, thread, time};

fn main() {
    let mut universe = generate_universe(50, 50);
    let generations = 500;
    for _ in 0..=generations {
        thread::sleep(time::Duration::from_millis(100));

        print(&universe);

        universe = game_of_life(universe);
    }
}

fn generate_universe(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut universe = vec![vec![0_u8; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            if rand::random() {
                universe[row][col] = 1;
            }
        }
    }

    universe
}

fn game_of_life(current_gen: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = current_gen.len();
    let cols = current_gen[0].len();

    let mut next_gen = vec![vec![0_u8; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            let current_cell = current_gen[row][col];
            let mut live_neighbors = 0;

            // count live neighbors
            for x in -1i8..=1 {
                for y in -1i8..=1 {
                    let neighbor_row = (row as i8) + x;
                    let neighbor_col = (col as i8) + y;

                    if neighbor_row > 0
                        && neighbor_col > 0
                        && neighbor_row < rows as i8
                        && neighbor_col < cols as i8
                    {
                        live_neighbors += current_gen[neighbor_row as usize][neighbor_col as usize];
                    }
                }
            }

            if live_neighbors > 0 {
                live_neighbors -= current_cell;
            }

            // apply rules
            next_gen[row][col] = if current_cell == 1 && !(2..=3).contains(&live_neighbors) {
                0
            } else if current_cell == 0 && live_neighbors == 3 {
                1
            } else {
                current_cell
            }
        }
    }

    next_gen
}

fn print(universe: &[Vec<u8>]) {
    clear_terminal();

    universe.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            let pretty_cell = if *cell == 1 { '■' } else { '□' };
            print!("{pretty_cell} ");
        });

        println!();
    });
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
        return;
    }

    Command::new("clear").status().unwrap();
}
