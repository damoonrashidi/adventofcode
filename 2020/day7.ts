import { BagMap, BagContent, testInput, input } from './input/day7-input';

function canContainGoldBag(bags: BagContent[], map: BagMap): boolean {
  if (bags.length === 0) {
    return false;
  } else if (bags.map(bag => bag.color).includes('shiny gold')) {
    return true;
  }

  return bags.reduce((truth: boolean, bag: BagContent) => {
    const content = map.get(bag.color) as BagContent[];
    return truth || canContainGoldBag(content, map);
  }, false);
}

function costForBag({ color, count }: BagContent, map: BagMap): number {
  const children = map.get(color) as BagContent[];

  if (children.length === 0) {
    return count;
  }

  return children.reduce(
    (total, bag) => total + bag.count * costForBag(bag, map),
    0
  );
}

function puzzleOne(bags: BagMap): number {
  return [...bags.keys()].filter(color => {
    return canContainGoldBag(bags.get(color) as BagContent[], bags);
  }).length;
}

function puzzleTwo(map: BagMap): number {
  const bags = map.get('shiny gold') as BagContent[];

  return bags.reduce(
    (total, bag) => total + bag.count * costForBag(bag, map),
    0
  );
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(testInput));
