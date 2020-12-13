import { input, testInput } from './input/day10-input';

function puzzleOne(adapters: number[]): number {
  let sortedAdapters = adapters.sort((a, b) => a - b);
  let jolt = 0;
  let jump1 = 0;
  let jump3 = 0;

  for (let adapter of sortedAdapters) {
    if (adapter - jolt === 1) {
      jump1++;
    } else if (adapter - jolt > 1 && adapter - jolt <= 3) {
      jump3++;
    }

    jolt = adapter;
  }

  return jump1 * (jump3 + 1);
}

function puzzleTwo(data: number[]): number {
  return data.length;
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(testInput));
