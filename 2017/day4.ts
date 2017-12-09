import { Set } from 'core-js';
import { input } from './input/day4';

const calc = (list: string[]) => 
  list
    .map(row => row.split(' ').reduce((_, __, ___, array) => new Set(array).size === array.length, false))
    .reduce((acc: number, valid: boolean) => acc + (valid ? 1 : 0), 0);


console.log(calc(input.split('\n')));