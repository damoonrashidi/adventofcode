import { input } from './input/day4-input';

const validPuzzleOne = (credentials: string[][]): boolean => {
  return (
    credentials.length === 8 ||
    (credentials.length === 7 && !credentials.find(items => items[0] === 'cid'))
  );
};

const byr = (key: string, value: string) =>
  key === 'byr' ? value.length === 4 && +value >= 1920 && +value <= 2002 : true;

const iyr = (key: string, value: string) =>
  key === 'iyr' ? value.length === 4 && +value >= 2010 && +value <= 2020 : true;

const eyr = (key: string, value: string) =>
  key === 'eyr' ? +value >= 2020 && +value <= 2030 : true;

const hcl = (key: string, value: string) =>
  key === 'hcl' ? value.length === 7 && !!value.match(/#[0-9a-f]+/) : true;

const ecl = (key: string, value: string) =>
  key === 'ecl' ? 'ambblubrngrygrnhzloth'.includes(value) : true;

const pid = (key: string, value: string) =>
  key === 'pid' ? value.length === 9 && !!value.match(/[0-9]{9,9}/) : true;

const hgt = (key: string, value: string) => {
  if (key !== 'hgt') {
    return true;
  }

  if (!!!value.match(/[\d]cm|in/)) {
    return false;
  }

  const height = Number(value.replace(/cm|in/, ''));
  const [min, max] = value.includes('cm') ? [150, 194] : [59, 76];
  return min <= height && height <= max;
};

const validPuzzleTwo = (credentials: string[][]): boolean => {
  if (!validPuzzleOne(credentials)) {
    return false;
  }

  return credentials.reduce(
    (valid, [key, value]) =>
      valid &&
      (byr(key, value) &&
        iyr(key, value) &&
        eyr(key, value) &&
        hgt(key, value) &&
        hcl(key, value) &&
        ecl(key, value) &&
        pid(key, value)),
    true as boolean
  );
};

function puzzleOne(input: string[][][]): number {
  return input.filter(validPuzzleOne).length;
}

function puzzleTwo(input: string[][][]): number {
  return input.filter(validPuzzleTwo).length;
}

console.log('Puzzle one: ', puzzleOne(input));
console.log('Puzzle two: ', puzzleTwo(input));
