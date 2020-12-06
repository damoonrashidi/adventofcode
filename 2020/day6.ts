import { input } from './input/day6-input';

const uniqueYesPerGroup = (group: string[][]): number => {
  const y = group.reduce((a, b) => a.concat(b));

  return new Set(y).size;
};

const countAnswer = (s: string, g: string[]): number =>
  g.filter(x => x === s).length;

const sharedYesPerGroup = (group: string[][]): number => {
  const people = group.length;

  const allAnswers = group.reduce((a, b) => a.concat(b), []);

  const uniqueAnswers = [...new Set(allAnswers)];

  return uniqueAnswers.filter(a => countAnswer(a, allAnswers) === people)
    .length;
};

function puzzleOne(groups: string[][][]): number {
  return groups.map(group => uniqueYesPerGroup(group)).reduce((a, b) => a + b);
}

function puzzleTwo(groups: string[][][]): number {
  return groups.map(group => sharedYesPerGroup(group)).reduce((a, b) => a + b);
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
