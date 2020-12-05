import { input } from './input/day5-input';

const ROWS = 128;
const COLUMNS = 8;

const seatId = (row: number, column: number) => row * 8 + column;

const seatIdByInstruction = (aile: string[]): number => {
  const [minR, maxR, minC, maxC] = aile.reduce(
    ([minR, maxR, minC, maxC], instruction) => {
      if (instruction === 'F') {
        return [minR, (minR + maxR) / 2, minC, maxC];
      } else if (instruction === 'B') {
        return [(minR + maxR) / 2, maxR, minC, maxC];
      } else if (instruction === 'R') {
        return [minR, maxR, (minC + maxC) / 2, maxC];
      }
      return [minR, maxR, minC, (minC + maxC) / 2];
    },
    [0, ROWS, 0, COLUMNS] as number[]
  );

  const row = Math.min(minR, maxR);
  const column = Math.min(minC, maxC);

  return seatId(row, column);
};

function puzzleOne(instructions: string[][]): number {
  return Math.max(...instructions.map(seatIdByInstruction));
}

function puzzleTwo(instructions: string[][]): number {
  const seats = new Set(instructions.map(seatIdByInstruction));
  for (let i = 7; i < 816; i++) {
    if (!seats.has(i)) {
      return i;
    }
  }
  return -1;
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
