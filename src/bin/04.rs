advent_of_code::solution!(4);

type Grid = Vec<Vec<char>>;

fn create_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn check_horizontally(
    i: usize,
    j: usize,
    grid: &Grid,
    amount_cols: usize,
) -> u8 {
    let mut count: u8 = 0;
    // Check forward
    if j <= amount_cols - 4 {
        let mas = format!("{}{}{}", grid[i][j+1], grid[i][j+2], grid[i][j+3]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    // Check backwards
    if j >= 3 {
        let mas = format!("{}{}{}", grid[i][j-1], grid[i][j-2], grid[i][j-3]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    count
}
fn check_vertically(
    i: usize,
    j: usize,
    grid: &Grid,
    amount_rows: usize,
) -> u8 {
    let mut count: u8 = 0;
    // Check upwards
    if i >= 3 {
        let mas = format!("{}{}{}", grid[i-1][j], grid[i-2][j], grid[i-3][j]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    // Check downwards
    if i <= amount_rows - 4 {
        let mas = format!("{}{}{}", grid[i+1][j], grid[i+2][j], grid[i+3][j]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    count

}
fn check_diagonally(
    i: usize,
    j: usize,
    grid: &Grid,
    amount_rows: usize,
    amount_cols: usize,
) -> u8 {
    let mut count: u8 = 0;
    // CHeck up right
    if i >= 3 && j <= amount_cols - 4 {
        let mas = format!("{}{}{}", grid[i-1][j+1], grid[i-2][j+2], grid[i-3][j+3]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    // Check up left
    if i >= 3 && j >= 3 {
        let mas = format!("{}{}{}", grid[i-1][j-1], grid[i-2][j-2], grid[i-3][j-3]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    // Check down left
    if i <= amount_rows - 4 && j >= 3 {
        let mas = format!("{}{}{}", grid[i+1][j-1], grid[i+2][j-2], grid[i+3][j-3]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    // Check down right
    if i <= amount_rows - 4 && j <= amount_cols - 4 {
        let mas = format!("{}{}{}", grid[i+1][j+1], grid[i+2][j+2], grid[i+3][j+3]);
        count += if mas == "MAS" { 1 } else { 0 };
    }
    count
}

// Assumes 'X' was found at row i, column j.
fn count_finds(i: usize, j: usize, grid: &Grid) -> u32 {
    let mut finds: u32 = 0;
    let amount_rows = grid.len();
    let amount_cols = grid[0].len();
    finds += check_horizontally(i, j, grid, amount_cols) as u32;
    finds += check_vertically(i, j, grid, amount_rows) as u32;
    finds += check_diagonally(i, j, grid, amount_rows, amount_cols) as u32;
    finds
}

fn count_crosses(i: usize, j: usize, grid: &Grid) -> u32 {
    let mut count = 0;
    let mut found_one_diagonal = false;
    let amount_rows = grid.len();
    let amount_cols = grid[0].len();
    // CHeck up from left to right
    if i > 0 && i <= amount_rows - 2 && j <= amount_cols - 2 && j > 0 {
        let mut temp: String = String::from(grid[i+1][j-1]);
        if temp == "M" || temp == "S" {
            temp.push(grid[i-1][j+1]);
            match temp.as_str() {
                "SM" | "MS" => found_one_diagonal = true,
                _ => {}
            }
        }
    }
    // Check down from left to right
    if i > 0 && i <= amount_rows - 2 && j <= amount_cols - 2 && j > 0 {
        let mut temp: String = String::from(grid[i-1][j-1]);
        if temp == "M" || temp == "S" {
            temp.push(grid[i+1][j+1]);
            match temp.as_str() {
                "SM" | "MS" if found_one_diagonal => count += 1,
                _ => {}
            }
        }
    }
    count
}

// I am sorry for this code D-:
pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid = create_grid(&input);
    let mut count: u32 = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'X' {
                count += count_finds(i, j, &grid);
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid = create_grid(&input);
    let mut count: u32 = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'A' {
                count += count_crosses(i, j, &grid);
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
