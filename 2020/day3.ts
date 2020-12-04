import { input } from './input/day3-input';

const puzzleOne = (rows: string[], stepX = 3, stepY = 1): number => {
  const w = rows[0].length;
  let x = 0;
  let trees = 0;

  for (let y = 0; y < rows.length; y += stepY) {
    if (!rows[y]) {
      return trees;
    }

    const charAtSpace = rows[y].charAt(x);
    if (charAtSpace === '#') {
      trees++;
    }
    x = (x + stepX) % w;
  }

  return trees;
};

const puzzleTwo = (rows: string[]): number => {
  return [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]].reduce(
    (product, [stepX, stepY]) => product * puzzleOne(rows, stepX, stepY),
    1
  );
};

console.log('Puzzle one: ', puzzleOne(input));
console.log('Puzzle two: ', puzzleTwo(input));
