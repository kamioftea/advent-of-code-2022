---
day: 6
tags: post
header: 'Day 6: Tuning Trouble'
---
The expedition is finally underway, but the communication system I've been given is broken as it can't pick up the 
signal from the noise. I need to add a way for the device to identify the communication headers in a stream of 
seemingly random characters.

## Part 1 - Start of packet header

The start of packet header will be four different characters, so I need to detect the first time that happens in the 
stream. I can use [`Itertools::tuple_windows`](
https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.tuple_windows) to get a moving window of four
characters over the stream, add an index, and return the offset of the first one I find where all the characters are 
unique.

```rust

fn find_start_of_packet(data_stream: &String) -> usize {
    let (i, _) = 
        data_stream
            .chars()
            .tuple_windows()
            .enumerate()
            .find(|(_, (a, b, c, d))| is_unique(a, b, c, d))
            .unwrap();

    i + 4
}

fn is_unique(a: &char, b: &char, c: &char, d: &char) -> bool {
    let mut set = BTreeSet::new();
    set.insert(a);
    set.insert(b);
    set.insert(c);
    set.insert(d);

    set.len() == 4
}
```

The examples in the puzzle can be used as test cases.

```rust
#[cfg(test)]
mod tests {
    use crate::day_6::find_start_of_packet;

    #[test]
    fn can_parse() {
        assert_eq!(
            find_start_of_packet(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            7
        );
        assert_eq!(
            find_start_of_packet(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            5
        );
        assert_eq!(
            find_start_of_packet(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            6
        );
        assert_eq!(
            find_start_of_packet(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            10
        );
        assert_eq!(
            find_start_of_packet(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            11
        );
    }
}
```

Those are passing, so I can apply it to the puzzle input.

```rust
pub fn run() {
    let _contents = 
        fs::read_to_string("res/day-6-input").expect("Failed to read file");

    let start_of_packet = find_start_of_packet(&_contents);
    println!(
        "The start of packet is detected after {} characters", 
        start_of_packet
    );
}

// The start of packet is detected after 1920 characters
```

## Part two - Start of message header

In the elves' protocol, the start of message header will be the first substring of 14 unique characters. The part 1 
moving window as a tuple gets a bit unwieldy at a window size of 14, so I'll refactor to use `Vec::windows`. I can
then take the window size as a parameter as the window is now a slice of arbitrary length. I also update the filter to
use a built-in way of getting the unique characters in an iterator.

```rust
fn find_non_repeating_string_of_length(
    data_stream: &String, 
    window_size: usize
) -> usize {
    let chars: Vec<char> = data_stream.chars().collect();
    
    let (i, _) =
        chars.windows(window_size)
             .enumerate()
             .find(|(_, window)| window.iter().unique().count() == window_size)
             .unwrap();

    i + window_size
}
```

I can add tests to check for the window of 14 characters examples. They also have a lot of boilerplate that makes 
them harder to read, so I'll clean that up a bit too.

```rust
#[test]
fn can_find_start_of_packet() {
    let examples = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    for (data_stream, expected) in examples {
        assert_eq!(
            find_non_repeating_string_of_length(&data_stream.to_string(), 4),
            expected
        )
    }
}

#[test]
fn can_find_start_of_message() {
    let examples = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    for (data_stream, expected) in examples {
        assert_eq!(
            find_non_repeating_string_of_length(&data_stream.to_string(), 14),
            expected
        )
    }
}
```

The run method can similarly be updated to run for both window lengths

```rust
pub fn run() {
    let _contents = 
        fs::read_to_string("res/day-6-input").expect("Failed to read file");

    let start_of_packet = find_non_repeating_string_of_length(&_contents, 4);
    println!(
        "The start of packet is detected after {} characters", 
        start_of_packet
    );

    let start_of_message = find_non_repeating_string_of_length(&_contents, 14);
    println!(
        "The start of packet is detected after {} characters", 
        start_of_message
    );
}
// The start of packet is detected after 1920 characters
// The start of packet is detected after 2334 characters
```

## Performance enhancement

I'm still not fully happy with my solution. Recalculating the unique characters for each window is quite inefficient. I 
have the idea of keeping track of the current count for each character in the window, and then for each step I can 
increment the count for the new character, and decrement the count for the one leaving the window.

I first encode this behaviour in a custom struct + implementation. It needs to:

1. Initialise its state with the first `<window_size>` characters.
2. Allow a character to be added, and another removed in one step as the window is advanced.
    * Adding needs to increment the count for that character - adding a count of one if the character is new
    * Removing should decrement the count for that character - removing that count if it goes to 0
3. Expose the number of unique characters, i.e. its length.

```rust
struct Counts {
    counts: HashMap<char, usize>,
}

impl Counts {
    fn new(init: &str) -> Self {
        Self {
            counts: init.chars().counts_by(|c| c)
        }
    }

    fn add_and_remove(&mut self, to_add: &char, to_remove: &char) {
        if to_add == to_remove {
            return;
        }

        self.add(to_add);
        self.remove(to_remove);
    }

    fn add(&mut self, to_add: &char) {
        let new_to_add_count = self.counts.get(to_add).unwrap_or(&0) + 1;
        self.counts.insert(*to_add, new_to_add_count);
    }

    fn remove(&mut self, to_remove: &char) {
        let new_to_remove_count = self.counts.get(to_remove).unwrap() - 1;
        if new_to_remove_count == 0 {
            self.counts.remove(to_remove);
        } else {
            self.counts.insert(*to_remove, new_to_remove_count);
        }
    }

    fn len(&self) -> usize {
        self.counts.len()
    }
}
```

The implementation of `find_non_repeating_string_of_length` can then be refactored to use this structure. The 
existing tests will verify `Counts` works as expected and that the behavior of `find_non_repeating_string_of_length` 
is the same.

```rust
fn find_non_repeating_string_of_length(
    data_stream: &String, 
    window_size: usize
) -> usize {
    let (init, rest) = data_stream.split_at(window_size);
    let mut counts = Counts::new(init);

    for (i, (to_add, to_remove))
    in rest.chars()
           .zip(data_stream.chars())
           .enumerate()
    {
        counts.add_and_remove(&to_add, &to_remove);

        if counts.len() == window_size {
            return i + window_size + 1;
        }
    }

    unreachable!()
}
```

The original version was running in 2.5 - 3ms, and the updated one in 1.5-2ms, so it does seem to be an improvement.
