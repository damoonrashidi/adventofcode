import { input } from './input/day2-input';

function puzzleOne(values: string[][]) {
  return values.filter(([rule, password]) => {
    const [ranges, character] = rule.split(' ');
    const [min, max] = ranges.split('-').map(Number);

    const characterCount = password.split('').filter(x => x === character)
      .length;
    return min <= characterCount && characterCount <= max;
  }).length;
}

function puzzleTwo(values: string[][]) {
  return values.filter(([rule, password]) => {
    const [ranges, character] = rule.split(' ');
    const [x1, x2] = ranges.split('-').map(Number);

    const p1 = Number(password[x1 - 1] === character);
    const p2 = Number(password[x2 - 1] === character);

    return p1 + p2 === 1;
  }).length;
}

console.log('Puzzle one', puzzleOne(input));
console.log('Puzzle two', puzzleTwo(input));
