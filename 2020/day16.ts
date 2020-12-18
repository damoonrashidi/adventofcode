import {
  testRules,
  testTickets,
  inputRules,
  inputTickets,
  myTicket,
  Rule,
  Rules,
} from './input/day16-input';

const inRange = (value: number, [min, max]: number[]) =>
  min <= value && value <= max;

const isTicketValid = (ticketValues: number[], rules: Rule[]) => {
  return ticketValues.every(value =>
    rules.some(rule => rule.ranges.some(range => inRange(value, range)))
  );
};

const getColumnNames = (values: number[], rules: Rule[]): string[] => {
  const matchingRules = rules.filter(rule =>
    values.every(
      value => inRange(value, rule.ranges[0]) || inRange(value, rule.ranges[1])
    )
  );

  return matchingRules.map(rule => rule.field);
};

function puzzleOne(rules: Rules, tickets: number[][]): number {
  return tickets
    .flatMap(ticket =>
      ticket.filter(value => {
        return !rules.some(
          ([range1, range2]) => inRange(value, range1) || inRange(value, range2)
        );
      })
    )
    .reduce((a, b) => a + b);
}

/**
 * 1 2 3 4 5
 * 4 1 5 0 3
 * 8 3 5 1 4
 * 4 1 5 0 3
 */

function puzzleTwo(rules: Rule[], list: number[][]): number {
  const tickets = list.filter(ticket => isTicketValid(ticket, rules));

  const fieldMap = new Map<number, string[]>();
  const nameMap = new Map<number, string>();

  for (let y = 0; y < tickets[0].length; y++) {
    const columnValues = tickets.map(ticket => ticket[y]);

    const candidates = getColumnNames(columnValues, rules);

    fieldMap.set(y, candidates);
  }

  const mostLikely = [...fieldMap.entries()];

  mostLikely.sort(([, a], [, b]) => a.length - b.length);

  let takenFields: string[] = [];

  mostLikely.forEach(([key, candidates]) => {
    const name = candidates.find(x => !takenFields.includes(x)) as string;

    takenFields.push(name);

    nameMap.set(key, name);
  });

  const departureColumns: number[] = [];

  [...nameMap.entries()].forEach(([index, name]) => {
    if (name.startsWith('departure')) {
      departureColumns.push(index);
    }
  });

  return myTicket
    .filter((_, index) => departureColumns.includes(index))
    .reduce((a, b) => a * b, 1);
}

// console.log('Puzzle one:', puzzleOne(rules, tickets));
console.log('Puzzle two:', puzzleTwo(inputRules, inputTickets));
