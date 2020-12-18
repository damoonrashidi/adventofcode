import {
  input,
  testInput,
  Instruction,
  Direction,
  Turn,
  Move,
} from './input/day12-input';

function turnToDirection(
  [turn, degrees]: Instruction,
  facing: Direction
): Direction {
  if (turn === Turn.F) {
    return facing;
  }

  const lefties = {
    [Direction.E]: Direction.N,
    [Direction.N]: Direction.W,
    [Direction.W]: Direction.S,
    [Direction.S]: Direction.E,
  };

  const righties = {
    [Direction.E]: Direction.N,
    [Direction.N]: Direction.W,
    [Direction.W]: Direction.S,
    [Direction.S]: Direction.E,
  };

  for (let turned = 0; turned <= degrees; turned += 90) {
    if (turn === Turn.L) {
      facing = lefties[facing];
    } else {
      facing = righties[facing];
    }
  }

  return facing;
}

function signDistance(facing: Direction, distance: number): number {
  if ([Direction.S, Direction.W].includes(facing)) {
    return -distance;
  }
  return distance;
}

const isTurn = ([move]: Instruction): boolean =>
  [Turn.L, Turn.R].includes(move as Turn);

function normalizeTurn(
  [move, distance]: Instruction,
  facing: Direction
): Direction {
  return isTurn([move, distance])
    ? turnToDirection([move, distance], facing)
    : (move as Direction);
}

function puzzleOne(instructions: Instruction[]): number {
  let { x, y } = { x: 0, y: 0 };
  let facing = Direction.E;

  instructions.forEach(instruction => {
    let [move, distance] = instruction;
    if (isTurn(instruction)) {
      facing = normalizeTurn(instruction, facing);
    }

    distance = signDistance(facing, distance);

    if (
      [Direction.W, Direction.E].includes(
        isTurn(instruction) ? facing : (move as Direction)
      )
    ) {
      x += distance;
    } else {
      y += distance;
    }

    console.log(facing, { x, y });
  });

  return Math.abs(x) + Math.abs(y);
}

function puzzleTwo(data: Instruction[]): number {
  return data.length;
}

console.log('Puzzle one:', puzzleOne(testInput));
console.log('Puzzle two:', puzzleTwo(testInput));
