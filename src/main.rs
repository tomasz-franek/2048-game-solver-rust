mod lib;


use crate::lib::{Direction, Solver, TABLE,TableReduction};
use crate::lib::MAX;


fn print_table(table: TABLE) {
    for y in 0..MAX {
        print!("[");
        for x in 0..MAX {
            print!("{:6}", table[y][x]);
            if x < MAX - 1 {
                print!(", ");
            }
        }
        print!("]");
        if y < MAX - 1 {
            println!(", ");
        }
    }
    println!();
    println!("--------------------------");
}

fn main() {

    let mut table: TABLE = [[0; MAX]; MAX];
    let mut counter: i32 = 0;
    let mut pos: [i32; 2] = [0, 0];

    while pos[0] != -1 {
        let direction: (Direction,i32) = Solver::best_next_move(table);
        println!("Direction {:?}", direction.0);
        let result:TableReduction = Solver::compact_table_by_direction(table, direction.0);
        table = result.table;


        counter += 1;
        let result = Solver::add_next_number(table);
        table = result.0;
        pos = result.1;
        print_table(table);
    }
    println!("Game over after moves {}", counter);
}
