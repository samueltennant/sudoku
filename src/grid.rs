pub struct Grid {
    nums: [u8; 81],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            nums: Grid::generate_board(),
        }
    }

    fn generate_board() -> [u8; 81] {
        let mut nums = [0u8; 81];

        // for n in nums {
        //     let available = Grid::find_available_values(n, &nums);
        //     println!("{:?}", available);
        // }

        nums[8] = 1;
        nums[10] = 2;
        nums[36] = 3;
        nums[80] = 4;
        let available = Grid::find_available_values(0, &nums);
        println!("{:?}", available);

        nums
    }

    /*
        Returns all available numbers for a given cell of index n.
        0 is a placeholder when a value has been removed.
    */
    fn find_available_values(n: u8, nums: &[u8; 81]) -> [u8; 9] {
        let mut available = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];

        let row = n % 9;
        let col = row % 9;

        for x in 0..9 {
            for y in 0..9 {
                // Ignore self.
                if x == row && y == col {
                    continue;
                }

                // If on same row or column.
                if x == col || y == row {
                    let index = y * 9 + x;
                    let val = nums[index as usize];
                    if val != 0 {
                        available[val as usize - 1] = 0;
                    }
                }
            }
        }

        // Check square.
        let indices = Grid::get_square_indices(row, col);

        for i in indices {
            println!("{}", i);
            let val = nums[i as usize];
            if val != 0 {
                available[val as usize - 1] = 0;
            }
        }

        available
    }

    fn get_square_indices(x: u8, y: u8) -> [u8; 9] {
        let indices;

        if x <= 2 {
            if y <= 2 {
                // top left
                indices = [0, 1, 2, 9, 10, 11, 18, 19, 20];
            } else if y <= 5 {
                // middle left
                indices = [27, 28, 29, 36, 37, 38, 45, 46, 47];
            } else {
                // bottom left
                indices = [54, 55, 56, 63, 64, 65, 72, 73, 74];
            }
        } else if x <= 5 {
            if y <= 2 {
                // top middle
                indices = [3, 4, 5, 12, 13, 14, 21, 22, 23];
            } else if y <= 5 {
                // middle middle
                indices = [30, 31, 32, 39, 40, 41, 48, 49, 50];
            } else {
                // bottom middle
                indices = [57, 58, 59, 66, 67, 68, 75, 76, 77];
            }
        } else {
            if y <= 2 {
                // top right
                indices = [6, 7, 8, 15, 16, 17, 24, 25, 26];
            } else if y <= 5 {
                // middle right
                indices = [33, 34, 35, 42, 43, 44, 51, 52, 53];
            } else {
                // bottom right
                indices = [60, 61, 62, 69, 70, 71, 78, 79, 80];
            }
        }

        indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_values() {
        let mut nums = [0u8; 81];
        nums[8] = 1;
        nums[10] = 2;
        nums[36] = 3;
        nums[80] = 4;
        let available = Grid::find_available_values(0, &nums);

        assert_eq!(available, [0, 0, 0, 4, 5u8, 6u8, 7u8, 8u8, 9u8]);
    }
}
