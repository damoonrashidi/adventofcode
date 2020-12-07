import { Bag, input } from './input/day7-input';

const canContainGoldBag = (contains: string[], bags: Bag[]): boolean => {
  if (contains.length === 0) {
    return false;
  } else if (contains.includes('shiny gold')) {
    return true;
  }

  return contains.reduce((truth: boolean, color: string) => {
    const bag = bags.find(s => s.color === color) as Bag;
    return truth || canContainGoldBag(bag.contains, bags);
  }, false);
};

function puzzleOne(bags: Bag[]): number {
  return bags.filter(bag => canContainGoldBag(bag.contains, bags)).length;
}
function puzzleTwo(_: any): any {}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
