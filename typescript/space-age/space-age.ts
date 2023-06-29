const planets: Map<string,number> = new Map([
  ["mercury", 0.2408467],
  ["venus", 0.61519726],
  ["earth", 1],
  ["mars", 1.8808158],
  ["jupiter", 11.862615],
  ["saturn", 29.447498],
  ["uranus", 84.016846],
  ["neptune", 164.79132]
]);
const earthSeconds = 31557600;

export function age(planet: string, seconds: number): number{
  let mult = planets.get(planet)!;
  return parseFloat((seconds / (mult * earthSeconds)).toFixed(2))
}
