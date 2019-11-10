import input from './input';

const transformInput = (data: string): [number, number, number, number][] => {
  return data.split('\n').map(instruction => {
    const [x, y] = (instruction.match(/[(\d),(\d)]/) as [string, string]).map(
      value => parseInt(value, 10)
    );

    const [width, height] = (instruction.match(/[(\d)x(\d)]/) as [
      string,
      string
    ]).map(value => parseInt(value, 10));

    return [x, y, width, height];
  });
};

const drawMap = (ctx: CanvasRenderingContext2D) => {
  ctx.globalCompositeOperation = 'lighten';
  ctx.fillStyle = 'rgba(255,255,255,0.05)';

  const instructions = transformInput(input);
  console.log(instructions);
  instructions.forEach(([x, y, width, height]) => {
    ctx.fillRect(x, y, width, height);
  });
};

const main = () => {
  const canvas = document.getElementById('canvas') as HTMLCanvasElement;
  const { width, height } = canvas;
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
  ctx.fillStyle = '#000';
  ctx.fillRect(0, 0, width, height);

  // drawMap(ctx);
};

document.addEventListener('DOMContentLoaded', main);
