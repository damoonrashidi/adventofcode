import { input } from './input/day3-input';

const w = input[0].length;
let x = 0;
let trees = 0;

for (let y = 0; y < input.length; y++) {
  const charAtSpace = input[y].charAt(x);
  if (charAtSpace === '#') {
    trees++;
  }
  x = (x + 3) % w;
}

console.log(trees);
