import { BagMap, BagContent, testInput, input } from './input/day7-input';

const canContainGoldBag = (bags: BagContent[], map: BagMap): boolean => {
  if (bags.length === 0) {
    return false;
  } else if (bags.map(bag => bag.color).includes('shiny gold')) {
    return true;
  }

  return bags.reduce((truth: boolean, bag: BagContent) => {
    const content = map.get(bag.color) as BagContent[];
    return truth || canContainGoldBag(content, map);
  }, false);
};

const costForBag = (color: string, map: BagMap, mem = 0): number => {
  const children = map.get(color) as BagContent[];

  if (children.length === 0) {
    return mem;
  }

  return children.reduce((total, content) => {
    const cost = content.count * costForBag(content.color, map, content.count);
    console.log(content, cost);
    return total + cost;
  }, 0);
};

function puzzleOne(bags: BagMap): number {
  return [...bags.keys()].filter(color => {
    return canContainGoldBag(bags.get(color) as BagContent[], bags);
  }).length;
}

function puzzleTwo(bags: BagMap): number {
  const start = bags.get('shiny gold') as BagContent[];

  const startCost = start.map(({ count }) => count).reduce((a, b) => a + b, 0);

  const totalCost = start.reduce(
    (total, bag) => total + bag.count * costForBag(bag.color, bags),
    0
  );

  return totalCost - startCost;
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(input));
