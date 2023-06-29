export function score(x: number, y: number): number {
  let distanceFromCenter = Math.sqrt((x * x) + (y * y));

  switch(true) {
    case distanceFromCenter <= 1:
      return 10;
      break;
    case distanceFromCenter > 1 && distanceFromCenter <= 5:
      return 5;
      break;
    case distanceFromCenter > 5 && distanceFromCenter <= 10:
      return 1;
      break;
    default: 
      return 0;
  }
}
