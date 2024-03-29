/// A representation of a 2D grid of u8s.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    /// Store the numbers in a 1D list...
    pub numbers: Vec<u8>,
    /// ...and use the width to determine the 1D offset as a 2D co-ordinate
    pub width: usize,
}

impl From<String> for Grid {
    /// Turn the characters into digits and concatenate, caching the width
    fn from(string: String) -> Self {
        Grid::from_string_with_mapping(
            &string,
            |c| {
                c.to_digit(10)
                 .expect(format!("{} is not a digit", c).as_str()) as u8
            },
        )
    }
}

/// Temporary struct representing an iterator over a grid
pub struct GridCoords<'a> {
    /// Reference to the grid being iterated
    grid: &'a Grid,
    /// The current position of the iterator
    pos: usize,
}

impl<'a> Iterator for GridCoords<'a> {
    type Item = ((usize, usize), u8);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.grid.get_with_coords(self.pos);
        self.pos = self.pos + 1;

        curr
    }
}

impl Grid {
    /// Build a new grid with a mapping function to generate the date for each cell
    pub fn new<F>(width: usize, height: usize, init: F) -> Self
        where F: Fn(usize, usize) -> u8
    {
        let mut numbers = Vec::new();
        for y in 0..height {
            for x in 0..width {
                numbers.push(init(x, y))
            }
        }

        Self { width, numbers }
    }

    pub fn from_string_with_mapping(input: &String, mapping: fn(char) -> u8) -> Self {
        let width: usize = input.lines().next().unwrap_or("").len();

        let numbers = input
            .lines()
            .flat_map(|line| {
                return line.chars().map(mapping);
            })
            .collect();

        Self { numbers, width }
    }

    /// Helper to abstract iterating over the whole grid
    pub fn iter(&self) -> GridCoords {
        GridCoords { grid: self, pos: 0 }
    }

    /// Return the value at the given co-ordinates
    pub fn get(&self, y: usize, x: usize) -> Option<u8> {
        self.pos_of((y, x))
            .and_then(|p| self.numbers.get(p))
            .map(|&v| v)
    }

    /// Update the value in a given cell
    pub fn set(&mut self, y: usize, x: usize, val: u8) -> bool {
        match self.pos_of((y, x)) {
            Some(pos) => {
                self.numbers[pos] = val;
                true
            }
            None => false,
        }
    }

    /// Turn (y, x) coordinates into a position in the underlying array
    pub fn pos_of(&self, (y, x): (usize, usize)) -> Option<usize> {
        if x >= self.width {
            return None;
        }

        let pos = x + y * self.width;

        if pos >= self.numbers.len() {
            return None;
        }

        return Some(pos);
    }

    /// Calculate the height from the
    pub fn height(&self) -> usize {
        (self.numbers.len() + self.width - 1) / self.width
    }

    /// Sum the cells in the grid
    pub fn sum(&self) -> usize {
        self.numbers.iter().map(|&v| usize::from(v)).sum()
    }

    /// Used by [`GridCoords::next`]
    pub fn get_with_coords(&self, pos: usize) -> Option<((usize, usize), u8)> {
        let x = pos % self.width;
        let y = pos / self.width;

        self.numbers.get(pos).map(|&val| ((y, x), val))
    }

    pub fn get_orthogonal_surrounds(&self, (y, x): (usize, usize)) -> Vec<((usize, usize), u8)> {
        [(-1, 0), (0, 1), (1, 0), (0, -1)] // N E S W
            .iter()
            .flat_map(|&(dy, dx)| self.get_relative(y, x, dy, dx))
            .collect()
    }

    pub fn get_relative(
        &self,
        y: usize,
        x: usize,
        dy: isize,
        dx: isize,
    ) -> Option<((usize, usize), u8)> {
        let y1 = (y as isize) + dy;
        let x1 = (x as isize) + dx;

        if y1 >= 0 && x1 >= 0 {
            self.get(y1 as usize, x1 as usize)
                .map(|val| ((y1 as usize, x1 as usize), val))
        } else {
            None
        }
    }

    /// Dump the grid to stdout - useful for visualising the grid when debugging
    #[allow(dead_code)]
    pub fn print(&self) -> String {
        self.print_with(
            |v| if v <= 9 {
                v.to_string()
            } else {
                "#".to_string()
            }
        )
    }

    #[allow(dead_code)]
    pub fn print_with<F>(&self, cell_renderer: F) -> String
        where F: Fn(u8) -> String
    {
        let (_, out) = self
            .iter()
            .fold((0usize, "".to_string()), |(prev_y, out), ((y, _), v)| {
                (
                    y,
                    format!(
                        "{}{}{}",
                        out,
                        if y != prev_y { "\n" } else { "" },
                        cell_renderer(v),
                    ),
                )
            });

        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::grid::Grid;

    fn sample_input() -> String {
        "12345\n\
        23456\n\
        34567\n\
        45678\n\
        56789"
            .to_string()
    }


    #[test]
    fn can_set_and_get() {
        let mut grid = Grid::from(sample_input());

        assert_eq!(grid.get(0, 0), Some(1));
        assert_eq!(grid.get(0, 4), Some(5));
        assert_eq!(grid.get(4, 0), Some(5));
        assert_eq!(grid.get(4, 4), Some(9));
        assert_eq!(grid.get(5, 4), None);
        assert_eq!(grid.get(3, 5), None);
        assert_eq!(grid.get(17, 29), None);

        grid.set(4, 4, 17);

        assert_eq!(grid.get(4, 4), Some(17));
    }

    #[test]
    fn can_print() {
        let input = sample_input();

        let mut grid = Grid::from(input.clone());

        assert_eq!(grid.print(), input);

        grid.set(4, 4, 10);

        assert_eq!(grid.print(), input.replace("9", "#"));
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_print_with_custom_output() {
        let input = sample_input();
        let grid = Grid::from(input);

        let expected = "abcde\n\
        bcdef\n\
        cdefg\n\
        defgh\n\
        efghi"
            .to_string();

        assert_eq!(grid.print_with(|v| char::from(v + 96).to_string()), expected);
    }

    #[test]
    fn set_ignores_out_of_bounds() {
        let mut grid = Grid::from(sample_input());

        assert_eq!(grid.set(5, 0, 9), false);
        assert_eq!(grid.set(0, 5, 9), false);
        assert_eq!(grid.set(5, 5, 9), false);
        // unchanged
        assert_eq!(grid.print(), sample_input());
    }

    #[test]
    fn can_calc_height() {
        assert_eq!(Grid::from("1\n2\n3\n4".to_string()).height(), 4);
        assert_eq!(Grid::from("12\n34\n56".to_string()).height(), 3);
        assert_eq!(Grid::from("123\n34".to_string()).height(), 2);
    }

    #[test]
    fn can_build_grid() {
        let grid = Grid::new(3, 3, |x, y| u8::try_from(x).unwrap() + u8::try_from(y).unwrap());
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.print(), "012\n123\n234");
    }

    #[test]
    fn can_sum_grid() {
        let grid = Grid::new(3, 3, |x, y| u8::try_from(x).unwrap() + u8::try_from(y).unwrap());
        assert_eq!(grid.sum(), 18)
    }
}
