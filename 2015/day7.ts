import { input } from './input/day7-input';

const rshift = (value: number, amount: number) => (+value || 0) >> +amount;
const lshift = (value: number, amount: number) => (+value || 0) << +amount;
const not = (value: number) => ~(value || 0);
const or = (a: number, b: number) => (+a || 0) | (+b || 0);
const and = (a: number, b: number) => (+a || 0) & (+b || 0);
const ops: { [k: string]: (...args: any[]) => number } = {
  LSHIFT: lshift,
  RSHIFT: rshift,
  NOT: not,
  AND: and,
  OR: or,
};

function puzzleOne(instructions: string[][]): number {
  const wires = new Map<string, number>();
  const g = (x: string) =>
    (Number.isNaN(parseInt(x)) ? wires.get(x) || 0 : +x) as number;

  instructions.forEach(instruction => {
    const [operation, destination] = instruction;
    const tokenized = operation.split(' ');
    let value = 0;

    if (tokenized.length === 1) {
      value = g(tokenized[0]);
    } else if (tokenized.length === 2) {
      const operation: string = tokenized[0];

      value = ops[operation](g(tokenized[1]));
    } else if (tokenized.length === 3) {
      const operation: string = tokenized[1];

      if (operation.includes('SHIFT')) {
        value = ops[operation](g(tokenized[0]), g(tokenized[2]));
      }

      value = Math.abs(ops[operation](g(tokenized[0]), g(tokenized[2])));
    }

    if (value < 0) {
      value = 65535 + value + 1;
    }

    wires.set(destination, value);
  });

  console.log(wires);
  return wires.get('a') || 0;
}

console.log('Puzzle one: ', puzzleOne(input));
