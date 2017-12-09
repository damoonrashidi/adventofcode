const day3 = 368078;

const getCoord = (value: number): number[] => {
  let cursor = 1;
  let acc = 2;
  let moves = 0;
  let circle = 1;
  let corners = [];
  while (cursor <= value) {
    corners.push(cursor);
    cursor += acc;
    moves++;
    if (moves === 4) {
      moves = 0;
      acc += 2;
      circle++;
    }
  }
  return [corners[corners.length - 1], circle];
}

console.log(getCoord(15));
