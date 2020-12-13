import { Seat, Chart, input, testInput } from './input/day11-input';

const getAdjacent = (x: number, y: number, chart: Chart): Seat[] => {
  const seats: Seat[] = [];

  for (let i = y - 1; i <= y + 1; i++) {
    for (let j = x - 1; j <= x + 1; j++) {
      const isInbound =
        i >= 0 && i < chart.length && j >= 0 && j < chart[i].length;
      const isNode = i === y && j === x;
      if (isInbound && !isNode) {
        seats.push(chart[i][j]);
      }
    }
  }

  return seats.filter(seat => seat !== '.');
};

const shouldBeOccupied = (seat: Seat, adjacent: Seat[]): boolean => {
  return seat === 'L' && !adjacent.includes('#');
};

const shouldBeFree = (seat: Seat, adjacent: Seat[]): boolean => {
  return seat === '#' && adjacent.filter(x => x === '#').length >= 4;
};

const getSeat = (x: number, y: number, chart: Chart): Seat => {
  const adjacentSeats = getAdjacent(x, y, chart);
  if (shouldBeOccupied(chart[y][x], adjacentSeats)) {
    return '#';
  } else if (shouldBeFree(chart[y][x], adjacentSeats)) {
    return 'L';
  }
  return chart[y][x];
};

const simulate = (chart: Chart): Chart => {
  const simulated: Chart = [];

  for (let y = 0; y < chart.length; y++) {
    simulated[y] = [];
    for (let x = 0; x < chart[y].length; x++) {
      simulated[y][x] = getSeat(x, y, chart);
    }
  }

  return simulated;
};

const occupancy = (chart: Chart): number => {
  return chart
    .reduce((list, row) => list.concat(row), [])
    .filter(n => n === '#').length;
};

function hashChart(chart: Chart): string {
  return chart.reduce((h, r) => h + r.join(''), '');
}

function puzzleOne(chart: Chart): number {
  let hash = 'a';
  let newHash = 'b';
  while (hash !== newHash) {
    chart = simulate(chart);
    hash = newHash;
    newHash = hashChart(chart);
  }

  return occupancy(chart);
}

function puzzleTwo(data: Chart): number {
  return data.length;
}

console.log('Puzzle one:', puzzleOne(input));
console.log('Puzzle two:', puzzleTwo(testInput));
