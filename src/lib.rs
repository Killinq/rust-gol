use colored::*;
use rand::Rng;
use std::{thread, time};

#[derive(Clone, Copy)]
struct Cell {
    state: usize,
    neighbors: usize,
}

impl Cell {
    fn new() -> Cell {
        let mut rng = rand::thread_rng();
        Cell {
            state: rng.gen_range(0..=1),
            neighbors: 0,
        }
    }
}

pub fn create_grid(x: usize, y: usize, time: u64) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for i in 0..x {
        grid.push(vec![]);
        for _ in 0..y {
            grid[i].push(Cell::new());
        }
    }
    display_grid(&grid);

    grid = calc_neighbors(&mut grid);
    loop {
        thread::sleep(time::Duration::from_millis(time));
        display_grid(&grid);
        grid = calc_neighbors(&mut grid);
    }
}

fn display_grid(grid: &Vec<Vec<Cell>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j].state {
                0 => print!("{} ", "#".red()),
                1 => print!("{} ", "#".green()),
                _ => print!(""),
            };
        }
        println!("");
    }
    println!("");
    println!("");
}

fn calc_neighbors(grid: &mut Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut next: Vec<Vec<Cell>> = grid.clone().to_vec();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            for i in 0..=2 {
                for j in 0..=2 {
                    grid[x][y].neighbors += grid[(x + i + grid.len() - 1) % grid.len()]
                        [(y + j + grid.len() - 1) % grid[x].len()]
                    .state;
                }
            }
            grid[x][y].neighbors -= grid[x][y].state;
            let neighbors = grid[x][y].neighbors;
            if grid[x][y].state == 1 && neighbors < 2 {
                next[x][y].state = 0;
            } else if grid[x][y].state == 1 && neighbors > 3 {
                next[x][y].state = 0;
            } else if grid[x][y].state == 0 && neighbors == 3 {
                next[x][y].state = 1;
            }
        }
    }
    next.to_vec()
}
