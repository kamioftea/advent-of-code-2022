---
day: 8
tags: post
header: 'Day 8: Treetop Tree House'
---
Today I'm tasked with analysing trees planted in a perfect grid by a previous expedition. Now they're grown, the elves
would like to assess their suitability to host a tree house.

## Representing a grid

Grid based puzzles are a staple of advent of code, so I already have an implementation I used multiple times last year.
I'm going to bring that into this year's code and add the extra methods I need. 

The [grid]({{'/advent_of_code_2022/util/grid/struct.Grid.html' | url}}) stores its contents in a `Vec<u8>`. So far I've 
not yet needed to store anything other than `u8`s in the cells, so I'll stick with that for now.  

```rust
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    /// Store the numbers in a 1D list...
    pub numbers: Vec<u8>,
    /// ...and use the width to determine the 1D offset as a 2D co-ordinate
    pub width: usize,
}
```

I've copied the parsing implementation, as usually the grid comes from puzzle input laid out as a grid of single digits.

```rust
impl From<String> for Grid {
    fn from(string: String) -> Self {
        let width: usize = string.lines().next().unwrap_or("").len();
  
        let numbers = string
            .lines()
            .flat_map(|line| {
              return line.chars().map(|c| {
                c.to_digit(10)
                 .expect(format!("{} is not a digit", c).as_str()) as u8
              });
            })
            .collect();
  
        Grid { numbers, width }
    }
}
```

I've also copied over the getters and setters that map from `x`, `y` co-ords to the index in the internal `Vec<u8>`, 
and a helper function to print the state of the grid, which is useful for testing and debugging. This needs to be 
flagged as `dead_code` to prevent warnings when not being compiled for tests.

```rust
impl Grid {
   pub fn get(&self, y: usize, x: usize) -> Option<u8> {
      self.pos_of(y, x)
          .and_then(|p| self.numbers.get(p))
          .map(|&v| v)
   }

   /// Update the value in a given cell
   pub fn set(&mut self, y: usize, x: usize, val: u8) -> bool {
      match self.pos_of(y, x) {
         Some(pos) => {
            self.numbers[pos] = val;
            true
         }
         None => false,
      }
   }

   /// Turn (y, x) coordinates into a position in the underlying array
   pub fn pos_of(&self, y: usize, x: usize) -> Option<usize> {
      if x >= self.width {
         return None;
      }

      let pos = x + y * self.width;

      if pos >= self.numbers.len() {
         return None;
      }

      return Some(pos);
   }
    
   pub fn height(&self) -> usize {
       self.numbers.len() / self.width
   }

   #[allow(dead_code)]
   pub fn print(&self) -> String {
       let (_, out) = self
           .iter()
           .fold((0usize, "".to_string()), |(prev_y, out), ((y, _), v)| {
               (
                   y,
                   format!(
                       "{}{}{}",
                       out,
                       if y != prev_y { "\n" } else { "" },
                       if v <= 9 {
                           v.to_string()
                       } else {
                           "#".to_string()
                       },
                   ),
               )
           });

       out.to_string()
   }
}
```

And copy over some tests.

```rust
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

    #[test]
    fn set_ignores_out_of_bounds() {
        let mut grid = Grid::from(sample_input());

        assert_eq!(grid.set(5, 0, 9), false);
        assert_eq!(grid.set(0, 5, 9), false);
        assert_eq!(grid.set(5, 5, 9), false);
        // unchanged
        assert_eq!(grid.print(), sample_input());
    }
}
```

## Part 1 - Finding visible trees 

I've decided to do this by creating a second grid to track the visible trees, then walking each row and column in 
both directions marking each tree that is taller than those seen before. First I need a way to initialise the 
visibility grid. The wierd signature for the initialising function is to allow closures that capture variables to be 
used, which is needed because I need to know the upper boundaries of the grid in the `Grid::init` function.

```rust
impl Grid {
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
    // ...
}
// ...
#[test]
fn can_build_grid() {
    let grid = Grid::new(
        3, 3, 
        |x, y| u8::try_from(x).unwrap() + u8::try_from(y).unwrap()
    );
    
    assert_eq!(grid.width, 3);
    assert_eq!(grid.height(), 3);
    assert_eq!(grid.print(), "012\n123\n234");
}
```

I then need to be able from the start and end of each row, a function that marks taller trees. To make this generic it
should take a starting point, a direction expressed as a delta for each step, and both grids - the source to look up 
tree heights, and the visibility grid, which needs to be mutable.

```rust
fn mark_visible_trees(
    (origin_y, origin_x): (usize, usize),
    (dy, dx): (isize, isize),
    trees_grid: &Grid,
    visibility_grid: &mut Grid,
) {
    let mut max_height = trees_grid.get(origin_x, origin_y).unwrap();

    (1..)
        .map(
            |pos| {
                with_delta(origin_y, dy, pos)
                    .zip(with_delta(origin_x, dx, pos))
                    .and_then(|(y, x)| trees_grid.get(y, x).map(|h| (y, x, h)))
            })
        .while_some()
        .for_each(|(y, x, h)| {
            if max_height < h {
                max_height = h;
                visibility_grid.set(y, x, 1);
            }
        });
}

fn with_delta(init: usize, delta: isize, multiplier: isize) -> Option<usize> {
    isize::try_from(init)
        .map(|init_i| init_i + delta * multiplier)
        .and_then(|result| usize::try_from(result))
        .ok()
}
```

Applying a possibly negative delta to unsigned integers is awkward due to the type switching involved. I pull that out
to its own function to reduce the noise. 

* I can start with an infinitely incrementing range to generate the list of positions. 
* Use the extracted `with_delta` for both `x` and `y` to give the current co-ordinates as an option. This will be 
  `None` if either have become negative.
* Use flat_map (Rust calls flat_map `and_then` for Options) to also get the tree height at the current position, 
  which will also be `None` if I've walked off the Grid.
* `while_some` stops the iterator when I do walk off the grid
* Then I'm keeping track of the current tallest tree, and if the tree at the current position is higher, mark it as 
  visible and bump the max height.

The last sub-task is to count the visible trees. I'll add a sum method to the grid implementation. Using `1`s to mark 
the visible trees, means that summing them will give the count I want.

```rust
impl Grid {
    // ...
    pub fn sum(&self) -> usize {
        self.numbers.iter().map(|&v| usize::from(v)).sum()
    }
    // ...
}
// ...
#[test]
fn can_sum_grid() {
    let grid = Grid::new(
      3, 3, 
      |x, y| u8::try_from(x).unwrap() + u8::try_from(y).unwrap()
    );
    assert_eq!(grid.sum(), 18)
}
```

Putting these all together and I can count the trees, and test with the example input.

```rust
fn find_visible_count(grid: &Grid) -> usize {
    let mut visible = Grid::new(
        grid.width,
        grid.height(),
        |x, y| u8::from(
          x == 0 
              || y == 0 
              || x == grid.width - 1 
              || y == grid.height() - 1
        ),
    );

    for y in 1..(grid.height() - 1) {
        mark_visible_trees((y, 0), (0, 1), &grid, &mut visible);
        mark_visible_trees((y, grid.width - 1), (0, -1), &grid, &mut visible);
    }

    for x in 1..(grid.width - 1) {
        mark_visible_trees((0, x), (1, 0), &grid, &mut visible);
        mark_visible_trees((grid.height() - 1, x), (-1, 0), &grid, &mut visible);
    }

    visible.sum()
}
// ...
#[test]
fn sample_grid() -> Grid {
    let input = "30373
25512
65332
33549
35390".to_string();
    
    Grid::from(input)
}

#[test]
fn can_count_visible() {
    assert_eq!(find_visible_count(&sample_grid()), 21);
}
```

Now this is passing I can apply the puzzle input.

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-8-input").expect("Failed to read file");
    let grid = Grid::from(contents);

    println!(
        "The count of visible trees is: {}",
        find_visible_count(&grid)
    );
}
// The count of visible trees is: 1538
```

## Part 2 - Survey the scenery

Having determined there is enough tree cover, the elves now want to know which tree is best so that they can see as many
trees as possible. They only plan to have windows aligned to the grid, so that's what I need to calculate.
Specifically I need to multiply the numbers of trees within the grid they can see in each of the four directions for 
each tree, and find the maximum value.

First to get the count for one direction. I can use a version of the stepping function from the rows/columns, but 
break as soon as I find a taller tree. I have to use `itertools::fold_while` for the count here instead of merging 
it with the other `Options`, as when a taller tree is encountered I need to increment the count one last time for 
that tree, whereas I need to stop on the current count if I walk off the grid.

```rust
fn count_visible_with_delta(
  (origin_y, origin_x): (usize, usize), 
  (dy, dx): (isize, isize), 
  origin_height: u8, 
  grid: &Grid
) -> usize {
    (1..)
        .map(
            |pos| {
                with_delta(origin_y, dy, pos)
                    .zip(with_delta(origin_x, dx, pos))
                    .and_then(|(y, x)| grid.get(y, x))
            })
        .while_some()
        .fold_while(
            0,
            |count, h|
                if h >= origin_height {
                  Done(count + 1) 
                } else { 
                  Continue(count + 1) 
                },
        )
        .into_inner()
}
```

I already have code from last year that allows iterating over each point in the grid, so I'll copy that over.

```rust
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
    // ...
    pub fn iter(&self) -> GridCoords {
        GridCoords { grid: self, pos: 0 }
    }
    // ...
}
```

Now to iterate over the grid, call `count_visible_with_delta` for each direction, and find the maximum multiple. 

```rust
fn find_best_scenery_score(grid: &Grid) -> usize {
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    grid.iter().map(
        |(pos, height)| {
            deltas.iter()
                  .map(|&delta| count_visible_with_delta(pos, delta, height, grid))
                  .reduce(|a, b| a * b).unwrap_or(0)
        }
    ).max().unwrap_or(0)
}
// ...
#[test]
fn can_find_max_score() {
    assert_eq!(find_best_scenery_score(&sample_grid()), 8);
}
// ...
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-8-input").expect("Failed to read file");
    let grid = Grid::from(contents);

    println!(
        "The count of visible trees is: {}",
        find_visible_count(&grid)
    );

    println!(
        "The tree with the highest scenery score is: {}",
        find_best_scenery_score(&grid)
    );
}
// The count of visible trees is: 1548
// The tree with the highest scenery score is: 496125
// 
// Finished in 3.98ms
```
