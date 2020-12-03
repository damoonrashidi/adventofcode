import { input } from './input/day1-input';

const puzzleOne = (values: number[]) => {
  let answer = 0;

  values.forEach(cursor => {
    const value = values.find(value => cursor + value === 2020);
    if (value) {
      answer = value * cursor;
      return;
    }
  });
  return answer;
};

const puzzleTwo = (values: number[]) => {
  for (let i = 0; i < values.length; i++) {
    for (let j = 1; j < values.length; j++) {
      const answer = values.find(
        candidate => candidate + values[i] + values[j] === 2020
      );

      if (answer) {
        return answer * values[i] * values[j];
      }
    }
  }
  return 0;
};

console.log('Puzzle one', puzzleOne(input));
console.log('Puzzle two', puzzleTwo(input));
