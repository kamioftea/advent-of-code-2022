const fs = require('fs/promises');
const path = require('node:path');

function injectWriteUpUrl(day, posts) {
    console.log(day, posts, posts[day])

    return posts[day] ? {'Write Up': posts[day]} : {};
}
async function buildDay(file, day, posts) {
    const contents = await fs.readFile(file, 'utf-8')
    const line = contents.split(/[\n\r]+/)[0]
    // This is my solution for [Advent of Code - Day 1 - _Sonar Sweep_](https://adventofcode.com/2021/day/1)
    const [,title, puzzleURL] = line.match(/\[Advent of Code - Day \d+ - _([^_]+)_]\(([^)]+)\)/) ?? []

    const links = {
        Puzzle: puzzleURL,
        ...(injectWriteUpUrl(day, posts)),
        Documentation: `./advent_of_code_2022/day_${day}/index.html`
    }

    return {day, title, links};
}

async function buildSolutionData(posts) {
    const solutions = [];
    const dir = await fs.opendir(path.join('..', 'src'));
    for await (const entry of dir) {
        const matches = entry.name.match(/day_(\d+)\.rs/)
        if(entry.isFile() && matches) {
            solutions.push(await buildDay(path.join(dir.path, entry.name), parseInt(matches[1]), posts))
        }
    }
    return solutions
}

module.exports = function() {
    return {
        eleventyComputed: {
            solutions: async (data) => {
                const postsCollection = data.collections.post;
                const posts = Object.fromEntries(
                    [...(postsCollection ?? [])].map(post => [post.data.day, post.url])
                );
                return [...(await buildSolutionData(posts))].sort((a, b) => a.day - b.day)
            }
        }
    }
}