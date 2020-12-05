import input from './input/day1-input';

const simpleCost = (mass: number): number => Math.floor(mass / 3) - 2;

const complexCost = (mass: number, mem = 0): number => {
  if (simpleCost(mass) < 2) {
    return mem;
  }
  const cost = simpleCost(mass);

  return complexCost(cost, mem + cost);
};

function puzzleOne(modules: number[]): number {
  return modules.map(simpleCost).reduce((a, b) => a + b);
}

function puzzleTwo(modules: number[]): number {
  return modules.map(n => complexCost(n)).reduce((a, b) => a + b);
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
