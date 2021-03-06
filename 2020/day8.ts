import {
  Instruction,
  InstructionSet,
  testInput,
  input,
} from './input/day8-input';

function canTerminate(instructions: InstructionSet): boolean {
  let i = 0;
  const visited = new Set<number>();
  while (true) {
    if (visited.has(i)) {
      return false;
    }

    const [instruction, value] = instructions[i];
    visited.add(i);

    i += instruction === Instruction.JMP ? value : 1;

    if (i >= instructions.length) {
      return true;
    }
  }
}

function accForRun(instructions: InstructionSet): number {
  let i = 0;
  let acc = 0;
  const visited = new Set<number>();

  while (!visited.has(i)) {
    if (!instructions[i]) {
      return acc;
    }
    const [instruction, value] = instructions[i];
    visited.add(i);
    if (instruction === Instruction.ACC) {
      acc += value;
      i++;
    } else if (instruction === Instruction.JMP) {
      i += value;
    } else if (instruction === Instruction.NOOP) {
      i++;
    }
  }
  return acc;
}

function puzzleOne(instructions: InstructionSet): number {
  return accForRun(instructions);
}

function puzzleTwo(instructions: InstructionSet): number {
  for (let i = 0; i < instructions.length; i++) {
    const [command, value] = instructions[i];
    if (command === Instruction.NOOP || command === Instruction.JMP) {
      const ammendment =
        command === Instruction.NOOP ? Instruction.JMP : Instruction.NOOP;
      const ammendedInstructions = instructions
        .slice(0, i)
        .concat([[ammendment, value]])
        .concat(instructions.slice(i + 1, instructions.length));

      if (canTerminate(ammendedInstructions)) {
        return accForRun(ammendedInstructions);
      }
    }
  }
  return -1;
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
