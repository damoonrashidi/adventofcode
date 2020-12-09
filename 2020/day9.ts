import { testInput, input } from './input/day9-input';

function isValid(number: number, preamble: number[]): boolean {
  for (let i = 0; i < preamble.length; i++) {
    for (let j = 1; j < preamble.length; j++) {
      const candidate = preamble[i] + preamble[j];

      if (candidate === number) {
        return true;
      }
    }
  }

  return false;
}

function seriesForInvalidNumber(
  target: number,
  list: number[],
  depth = 2
): number[] {
  for (let i = 0; i < list.length; i++) {
    const candidates = list.slice(i, depth);
    const sum = candidates.reduce((s, n) => s + n, 0);
    if (sum === target) {
      return candidates;
    }
  }

  return seriesForInvalidNumber(target, list, depth + 1);
}

function puzzleOne(list: number[]): number {
  for (let i = 25; i < list.length; i++) {
    const number = list[i];
    const preamble = list.slice(i - 25, i);
    if (!isValid(number, preamble)) {
      return number;
    }
  }
  return -1;
}

function puzzleTwo(list: number[]): number {
  const target = puzzleOne(list);
  const candidates = list.filter(x => x < target);

  const series = seriesForInvalidNumber(target, candidates);

  return Math.min(...series) + Math.max(...series);
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
