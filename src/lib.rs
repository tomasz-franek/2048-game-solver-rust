use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { Left, Right, Up, Down }

pub const MAX: usize = 4;

pub type TABLE = [[i32; MAX]; MAX];

const RANDOM_TRYS: i32 = 30;
pub const DIRECTIONS_LIST: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

pub struct RowData {
    pub(crate) row: Vec<i32>,
    pub(crate) counter: i32
}

pub struct TableReduction{
    pub table:TABLE,
    pub max_reduction:i32
}

pub struct Solver {}

impl Solver {
    pub fn add_next_number(mut table: TABLE) -> (TABLE,[i32; 2]) {
        let pos = Solver::generate_position(table);
        if pos[0] != -1 {
            table[pos[0] as usize][pos[1] as usize] = 2;
        }
        (table,pos)
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
        return RowData { row: return_vec, counter: max_reductions };
    }

    pub fn compact_table_by_direction(table: TABLE, direction: Direction) -> TableReduction {
        match direction {
            Direction::Left => {
                Solver::compact_direction_left(table)
            }
            Direction::Right => {
                Solver::compact_direction_right(table)
            }
            Direction::Up => {
                Solver::compact_direction_up(table)
            }
            Direction::Down => {
                Solver::compact_direction_down(table)
            }
        }
    }

    pub fn compact_direction_up(curr_table: TABLE) -> TableReduction {
        let mut vec: RowData;
        let mut max_reduction: i32 = 0;
        let mut table: TABLE = [[0; MAX]; MAX];
        for y in 0..MAX {
            vec = RowData { row: Vec::new(), counter: 0 };
            for x in 0..MAX {
                vec.row.push(curr_table[x][y]);
            }
            vec = Solver::prepare_vector(vec);
            max_reduction += vec.counter;
            for i in 0..vec.row.len() {
                table[i][y] = vec.row[i];
            }
        }
        TableReduction{table,max_reduction}
    }

    pub fn compact_direction_down(curr_table: TABLE) -> TableReduction {
        let mut vec: RowData;
        let mut max_reduction: i32 = 0;
        let mut table: TABLE = [[0; MAX]; MAX];
        for y in 0..MAX {
            vec = RowData { row: Vec::new(), counter: 0 };
            for x in (0..MAX).rev() {
                vec.row.push(curr_table[x][y]);
            }
            vec = Solver::prepare_vector(vec);
            max_reduction += vec.counter;
            for i in 0..vec.row.len() {
                table[MAX - i - 1][y] = vec.row[i];
            }
        }
        TableReduction{table, max_reduction}
    }
    pub fn compact_direction_left(curr_table: TABLE) -> TableReduction {
        let mut vec: RowData;
        let mut max_reduction: i32 = 0;
        let mut table: TABLE = [[0; MAX]; MAX];
        for x in 0..MAX {
            vec = RowData { row: Vec::new(), counter: 0 };
            for y in 0..MAX {
                vec.row.push(curr_table[x][y]);
            }
            vec = Solver::prepare_vector(vec);
            max_reduction += vec.counter;
            for i in 0..vec.row.len() {
                table[x][i] = vec.row[i];
            }
        }
        TableReduction{table,max_reduction}
    }
    pub fn compact_direction_right(curr_table: TABLE) -> TableReduction {
        let mut vec: RowData;
        let mut max_reduction: i32 = 0;
        let mut table: TABLE = [[0; MAX]; MAX];
        for x in 0..MAX {
            vec = RowData { row: Vec::new(), counter: 0 };
            for y in (0..MAX).rev() {
                vec.row.push(curr_table[x][y]);
            }
            vec = Solver::prepare_vector(vec);
            max_reduction += vec.counter;
            for i in 0..vec.row.len() {
                table[x][MAX - i - 1] = vec.row[i];
            }
        }
        TableReduction{table, max_reduction}
    }

    pub fn best_next_move(curr_table: TABLE) -> (Direction, i32) {
        let mut direction: Direction = Direction::Left;
        let mut max_reductions: i32 = 0;
        for current_direction in [Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            let first_compact:TableReduction = Solver::compact_table_by_direction(curr_table.clone(), current_direction);
            for _ in 0..RANDOM_TRYS {
                let mut table :TABLE = first_compact.table;
                let mut reductions :i32 = first_compact.max_reduction;
                let mut new_pos:[i32;2] = Solver::generate_position(table);
                while new_pos[0] != -1 {
                    table[new_pos[0] as usize][new_pos[1] as usize] = 2;
                    let next_direction:Direction = Solver::next_random_direction();
                    let result:TableReduction = Solver::compact_table_by_direction(table, next_direction);
                    table = result.table;
                    reductions = reductions + result.max_reduction;
                    new_pos = Solver::generate_position(table);
                }

                if reductions > max_reductions{
                    max_reductions = reductions;
                    direction= current_direction;
                }
            }
        }
        return (direction, max_reductions);
    }

    fn next_random_direction() -> Direction {
        return DIRECTIONS_LIST[rand::thread_rng().gen_range(0..DIRECTIONS_LIST.len())];
    }

    pub fn generate_position(table: TABLE) -> [i32; 2] {
        let mut list: Vec<[i32; 2]> = Vec::new();
        for y in 0..MAX {
            for x in 0..MAX {
                if table[y][x] == 0 {
                    list.push([y as i32, x as i32]);
                }
            }
        }
        if list.len() == 0 {
            return [-1; 2];
        }
        let position = rand::thread_rng().gen_range(0..list.len());
        return list[position];
    }


}
