---
header: Jeff's Advent of Code 2022
title: 'Solutions List | Advent of Code 2022 | Jeff Horton'
description: |
  Once again I'm attempting Advent of Code 2022 in Rust. This page links to my solutions and write-ups for each day's 
  puzzle.
---
[Advent of Code](https://adventofcode.com/2022) Is a yearly challenge with one coding puzzle a day from 1st of December
until Christmas Day. The challenges are language agnostic, providing the input as a text file, and expecting a number or
a string as the result of each part.

This year I've chosen to use [Rust](https://www.rust-lang.org/), and I've used the built-in Rust `cargo doc` tool to 
build the [documentation for the code](./advent_of_code_2022/). In parallel, I've used 11ty to build a static site for 
walk-throughs of how I've tackled each section. I've then used GitHub Actions to bundle the outputs from both of these, 
and publish them to this site on GitHub Pages.

## My Solutions

<div class="solutions-list">
{% for solution in solutions %}
  <section class="solution" aria-labelledby="{{ solution.title | slugify }}">
    <h3 class="solution-title" id="{{ solution.title | slugify }}">
      Day {{ solution.day }}: {{solution.title}}
    </h3>
    <div class="solution-links">
      {%- for label, href in solution.links -%}
        <a href="{{ href | url }}" class="solution-link">{{ label }}</a>
      {%- endfor -%}
    </div>
  </section>
{% endfor %}
</div>
