---
day: 12
type: post,
header: 'Day 12: Hill Climbing Algorithm'
---
I now need to get to the high ground so that I can find some signal for my poor comms device. This is a pathfinding 
challenge as I can only climb up where it's not too steep.

I'm lucky in that I have a couple of existing things that will help me here. I have the existing `Grid` 
implementation, though I will need a custom parser for the letter based input. I also have an implementation of 
Dijkstra's shortest path algorithm for the `Grid` from last year - which in turn was adapted from the example in Rust's 
`BinaryHeap` implementation.

## Map reading

First to ingest the puzzle input into a `Grid`. I can continue to represent the cells with `u8` if I map the letters 
to their position in the alphabet. First I'll move the existing `From<String>` to a more generic version, which the 
`From<String>` will now call.

```rust
impl Grid {
    // ...
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
    // ...
}
// ...
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
```

I also want to be able to print and test letter based grids. I will do the same generify and use refactor.

```rust
impl Grid {
    // ...
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
    // ...
}
//...
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
```

I can then import the puzzle input using that. I also need to capture the start `S` and end `E`. It would be 
slightly more efficient to get these as the parsing is done, but the `Grid` would be over-complicated by allowing 
that, and it's a very minor cost.

```rust
fn parse_input(input: &String) -> (Grid, Position, Position) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start = (y, x)
            } else if chr == 'E' {
                end = (y, x)
            }
        }
    }

    let grid = Grid::from_string_with_mapping(
        input,
        |chr| match chr {
            'S' => 1,
            'E' => 26,
            c => u8::try_from(c).unwrap() & 0b11111
        },
    );

    (grid, start, end)
}
// ...
fn sample_data() -> (Grid, Position, Position) {
    (
        Grid {
            width: 8,
            numbers: vec![
                1, 1, 2, 17, 16, 15, 14, 13,
                1, 2, 3, 18, 25, 24, 24, 12,
                1, 3, 3, 19, 26, 26, 24, 11,
                1, 3, 3, 20, 21, 22, 23, 10,
                1, 2, 4,  5,  6,  7,  8,  9,
            ],
        },
        (0, 0),
        (2, 5)
    )
}

//noinspection SpellCheckingInspection
#[test]
fn can_parse() {
    let input =
        "Sabqponm\n\
         abcryxxl\n\
         accszExk\n\
         acctuvwj\n\
         abdefghi".to_string();

    assert_eq!(parse_input(&input), sample_data())
}
```

## Running up that hill

First I need to copy over some methods from last year's grid that the shortest path algorithm uses.

```rust
impl Grid {
    // ...
    pub fn get_orthogonal_surrounds(
        &self, 
        (y, x): (usize, usize)
    ) -> Vec<((usize, usize), u8)> {
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
    // ...
}
```

I also need to copy over the `Cell` struct, used to hold the current costs for co-ordinates in the Binary Heap, and 
define an ordering so that the cheapest unprocessed `Cell` bubbles to the top of the heap. I've added in a separate 
value for the dist, so I can optimise the algorithm by including the direct path to the goal as an underestimate of 
the total cost for a node.

```rust
#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    dist: usize,
    cost: usize,
    coords: Position,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

I can then copy over the [`find_shortest_path`](
https://kamioftea.github.io/advent-of-code-2021/advent_of_code_2021/day_15/fn.find_shortest_path.html) from last year.
There are a few tweaks needed. The cost is now constant `1` per cell visited, but there are limits on which cells I 
can visit based on the height. I can also optimise it by including the manhatten distance to the goal in the cost 
turning it into `A*` pathfinding.

To walk through this
1. Initialise a list of cells with their distance usize::MAX, except the start cell which is `0`.
2. Initialise a `BinaryHeap` with the start cell, its distance from itself(`0`), and the minimum possible distance 
   to the goal as the cost.
3. Pop items off the heap (which gives the lowest cost first) until the goal is found - adding all their adjacent 
   cells to the heap if:
    * Visiting them via this cell is cheaper than previously calculated.
    * The cell is at most one height greater than the current value.
4. Once the goal is found, the distance to it is included in the data stored in the heap, so return it.

```rust
fn manhatten_distance((y_a, x_a): Position, (y_b, x_b): Position) -> usize {
   y_a.abs_diff(y_b) + x_a.abs_diff(x_b)
}

fn find_shortest_path(
   grid: &Grid,
   start: (usize, usize),
   goal: (usize, usize),
) -> Option<usize> {
   let mut heap: BinaryHeap<Cell> = BinaryHeap::new();
   let mut dists: Vec<usize> = (0..grid.numbers.len()).map(|_| usize::MAX).collect();

   dists[grid.pos_of(start).unwrap()] = 0;
   heap.push(Cell {
      dist: 0,
      cost: manhatten_distance(start, goal),
      coords: start,
   });

   while let Some(Cell { dist, coords, .. }) = heap.pop() {
      if coords == goal {
         return Some(dist);
      }

      if dist > dists[grid.pos_of(coords).unwrap()] {
         continue;
      }

      let current_height = grid.get(coords.0, coords.1).unwrap();

      for (next_coords, next_height) in grid.get_orthogonal_surrounds(coords) {
         let next_pos = grid.pos_of(next_coords).unwrap();

         if (current_height + 1 >= next_height) && (dist + 1 < dists[next_pos]) {
            heap.push(Cell {
               dist: dist + 1,
               cost: dist + 1 + manhatten_distance(next_coords, goal),
               coords: next_coords,
            });
            dists[next_pos] = dist + 1
         }
      }
   }

   None
}
```

Add a test from the puzzle example, and when that passes, also find the shortest path in the puzzle input.

```rust
#[test]
fn can_find_shortest_path() {
   let (grid, start, goal) = sample_data();
   assert_eq!(
      find_shortest_path_from_start(&grid, start, goal),
      Some(31)
   )
}
// ...
pub fn run() {
   let contents =
       fs::read_to_string("res/day-12-input").expect("Failed to read file");
   let (grid, start, goal) = parse_input(&contents);

   println!(
      "The shortest path to the goal is: {}",
      find_shortest_path_from_start(&grid, start, goal).unwrap()
   );

   println!(
      "The shortest trail to the goal is: {}",
      find_shortest_trail(&grid, goal).unwrap()
   )
}
// The shortest path to the goal is: 420
```

## Taking the scenic route

