use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction { Left, Right, Up, Down }

const MAX: usize = 4;

type TABLE = [[i32; MAX]; MAX];
const RANDOM_TRYS:i32 = 150;

const DIRECTIONS_LIST:[Direction;4] = [Direction::Up,Direction::Down,Direction::Left,Direction::Right];
struct RowData {
    row :Vec<i32>,
    counter : i32
}
fn main() {
    play_game();
}

fn play_game() {
    let mut table: TABLE = [[0; MAX]; MAX];
    let mut counter: i32 = 0;
    let mut pos: [i32; 2] = [0, 0];

    while pos[0] != -1 {
        let direction = get_best_next_move(table);
        println!("Direction {:?}", direction.0);
        let result = compact_table_by_direction(table, direction.0);
        table = result.0;


        counter += 1;
        pos = generate_position(table);
        if pos[0] != -1 {
            table[pos[0] as usize][pos[1] as usize] = 2;
            print_table(table);
        }
    }
    println!("Game over after moves {}", counter);
}

fn print_table(table: TABLE) {
    print!("[");
    for i in 0..MAX {
        print!("[");
        for j in 0..MAX {
            print!("{:6}", table[i][j]);
            if j < MAX - 1 {
                print!(", ");
            }
        }
        print!("]");
        if i < MAX - 1 {
            println!(", ");
        }
    }
    println!("]");
    println!("--------------------------");
}

fn generate_position(table: TABLE) -> [i32; 2] {
    let mut list: Vec<[i32; 2]> = Vec::new();
    for i in 0..MAX {
        for j in 0..MAX {
            if table[i][j] == 0 {
                list.push([i as i32, j as i32]);
            }
        }
    }
    if list.len() == 0 {
        return [-1; 2];
    }
    let position = rand::thread_rng().gen_range(0..list.len());
    return list[position];
}

fn compact_table_by_direction(table: TABLE, direction: Direction) -> (TABLE, i32) {
    let mut return_table: TABLE = [[0; MAX]; MAX];
    let mut vec: RowData;
    let mut max_reductions: i32 = 0;
    if let Direction::Right = direction {
        for x in 0..MAX {
            vec = RowData {row:Vec::new(), counter:0};
            for y in (0..MAX).rev() {
                vec.row.push(table[x][y]);
            }
            vec = prepare_vector(vec);
            max_reductions += vec.counter;
            if vec.row.len() > 0 {
                for i in 0..vec.row.len() {
                    return_table[x][MAX - i - 1] = vec.row[i];
                }
            }
        }
    }
    if let Direction::Left = direction {
        for x in 0..MAX {
            vec = RowData {row:Vec::new(), counter:0};
            for y in 0..MAX {
                vec.row.push(table[x][y]);
            }
            vec = prepare_vector(vec);
            max_reductions += vec.counter;
            if vec.row.len() > 0 {
                for i in 0..vec.row.len() {
                    return_table[x][i] = vec.row[i];
                }
            }
        }
    }
    if let Direction::Up = direction {
        for y in 0..MAX {
            vec = RowData {row:Vec::new(), counter:0};
            for x in 0..MAX {
                vec.row.push(table[x][y]);
            }
            vec = prepare_vector(vec);
            max_reductions += vec.counter;
            if vec.row.len() > 0 {
                for i in 0..vec.row.len() {
                    return_table[i][y] = vec.row[i];
                }
            }
        }
    }
    if let Direction::Down = direction {
        for y in 0..MAX {
            vec = RowData {row:Vec::new(), counter:0};
            for x in (0..MAX).rev() {
                vec.row.push(table[x][y]);
            }
            vec = prepare_vector(vec);
            max_reductions += vec.counter;
            if vec.row.len() > 0 {
                for i in 0..vec.row.len() {
                    return_table[MAX - i - 1][y] = vec.row[i];
                }
            }
        }
    }
    return (return_table, max_reductions);
}

fn get_best_next_move(table: TABLE) -> (Direction, i32) {
    let mut direction: Direction = Direction::Left;
    let mut max_reductions: i32 = 0;
    for current_direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
        let first_compact = compact_table_by_direction(table.clone(), current_direction);
        for _ in 0..RANDOM_TRYS {
            let mut curr_table = first_compact.0;
            let mut reductions :i32 = first_compact.1;
            let mut new_pos:[i32;2] = generate_position(curr_table);
            while new_pos[0] != -1 {
                curr_table[new_pos[0] as usize][new_pos[1] as usize] = 2;
                let next_direction:Direction = get_next_random_direction();
                let result = compact_table_by_direction(curr_table, next_direction);
                curr_table = result.0;
                reductions = reductions + result.1;
                new_pos = generate_position(curr_table);
            }

            if reductions > max_reductions{
                max_reductions = reductions;
                direction= current_direction;
            }
        }
    }
    return (direction, max_reductions);
}

fn get_next_random_direction() -> Direction {
  return DIRECTIONS_LIST[rand::thread_rng().gen_range(0..DIRECTIONS_LIST.len())];
}

fn prepare_vector(data: RowData) -> RowData {
    assert_eq!(data.row.len(), MAX);
    let mut vec: Vec<i32> = data.row.clone();
    let mut max_reductions: i32 = 0;
    vec.retain(|&x| x > 0);
    let mut return_vec: Vec<i32> = Vec::new();
    let mut prev: i32 = -1;
    for x in &vec {
        if prev > -1 {
            if *x == prev {
                return_vec.push((*x) + prev);
                max_reductions += prev;
                prev = -1;
            } else {
                return_vec.push(prev);
                prev = *x;
            }
        } else {
            prev = *x;
        }
    }
    if prev > -1 {
        return_vec.push(prev);
    }
    return RowData { row:return_vec,counter: max_reductions};
}
