---
header: Jeff's Advent of Code 2022
---
[Advent of Code](https://adventofcode.com/2022) Is a yearly challenge with one coding puzzle a day from 1st of December
until Christmas Day. The challenges are language agnostic, providing the input as a text file, and expecting a number or
a string as the result of each part.

This year I've chosen to use [Rust](https://www.rust-lang.org/), and I've used the built-in Rust `cargo doc` tool to 
build the [documentation for the code](./advent_of_code_2022/). In parallel, I've used 11ty to 
build a static site for write-ups of how I've tackled each section. I've then used GitHub Actions to publish the outputs 
from both of these, and publish them to this site on GitHub Pages.

## My Solutions

<div class="solutions-list">
{% for solution in solutions %}
  <div class="solution">
    <p class="solution-title">{{solution.title}}</p>
    <div class="solution-links">
      {%- for label, href in solution.links -%}
        <a href="{{ href | url }}" class="solution-link">{{ label }}</a>
      {%- endfor -%}
    </div>
  </div>
{% endfor %}
</div>
