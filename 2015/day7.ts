import { input } from './input/day7-input';

const evalute = (wires: Map<string, number>, value: string) => {
  if (Number.isNaN(parseInt(value))) {
    return wires.get(value) as number;
  }
  return parseInt(value);
};

function puzzleOne(instructions: string[][]): number {
  const wires = new Map<string, number>();

  instructions.forEach(instruction => {
    const [operation, destination] = instruction;
    const tokenized = operation.split(' ');

    if (tokenized.length === 1) {
      const value = evalute(wires, tokenized[0]);
      wires.set(destination, value);
    }

    else if (tokenized.length === 2) {

      

      wires.set(destination. 0);
    }

    else if (tokenized.length === 3) {
      wires.set(destination. 0);
    }


  });
}

console.log('Puzzle one: ', puzzleOne(input));
