use game2048::{Direction, MAX, Solver, TABLE, TableReduction};

#[test]
fn compact_direction_down() {
    let table: TABLE = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 4, 2],
        [0, 2, 4, 2]];
    let result: TableReduction = Solver::compact_table_by_direction(table, Direction::Down);
    assert_eq!(result.table,
               [
                   [0, 0, 0, 0],
                   [0, 0, 0, 0],
                   [0, 0, 0, 0],
                   [0, 2, 8, 4]
               ]
    );
    assert_eq!(result.max_reduction, 6);
}

#[test]
fn compact_direction_up() {
    let table: TABLE = [
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [0, 2, 4, 2],
        [0, 0, 4, 2]
    ];
    let result: TableReduction = Solver::compact_table_by_direction(table, Direction::Up);
    assert_eq!(result.table,
               [
                   [0, 2, 8, 4],
                   [0, 0, 4, 0],
                   [0, 0, 0, 0],
                   [0, 0, 0, 0]
               ]
    );
    assert_eq!(result.max_reduction, 6);
}

#[test]
fn compact_direction_left() {
    let table: TABLE = [
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [0, 0, 4, 4],
        [0, 0, 2, 2]];
    let result: TableReduction = Solver::compact_table_by_direction(table, Direction::Left);
    assert_eq!(result.table,
               [
                   [0, 0, 0, 0],
                   [4, 0, 0, 0],
                   [8, 0, 0, 0],
                   [4, 0, 0, 0]
               ]
    );
    assert_eq!(result.max_reduction, 6);
}

#[test]
fn compact_direction_right() {
    let table: TABLE = [
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [0, 0, 4, 4],
        [0, 0, 2, 2]];
    let result: TableReduction = Solver::compact_table_by_direction(table, Direction::Right);
    assert_eq!(result.table,
               [
                   [0, 0, 0, 0],
                   [0, 0, 0, 4],
                   [0, 0, 0, 8],
                   [0, 0, 0, 4]
               ]
    );
    assert_eq!(result.max_reduction, 6);
}

#[test]
fn generate_position_last_element() {
    let table: TABLE = [
        [2, 2, 2, 2],
        [2, 2, 2, 2],
        [2, 2, 0, 2],
        [2, 2, 2, 2]];
    let result = Solver::generate_position(table);
    assert_eq!(result[0], 2);
    assert_eq!(result[1], 2);
}

#[test]
fn generate_position_empty_table() {
    let table: TABLE = [[0;4];4];
    let result = Solver::generate_position(table);
    assert!(result[0]>=0 && result[0]< MAX as i32);
    assert!(result[0]>=0 && result[0]< MAX as i32);
}

#[test]
fn generate_position_full_table() {
    let table: TABLE = [
        [2, 2, 2, 2],
        [2, 2, 2, 2],
        [2, 2, 2, 2],
        [2, 2, 2, 2]];
    let result = Solver::generate_position(table);
    assert_eq!(result[0], -1);
    assert_eq!(result[1], -1);
}
