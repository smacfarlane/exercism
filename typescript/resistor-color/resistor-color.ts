export const colorCode = (color:  string) => {
  return Colors[color as ColorKey];
}


export enum Colors  { 
  "black" = 0,
  "brown",
  "red",
  "orange",
  "yellow",
  "green", 
  "blue",
  "violet",
  "grey",
  "white",
};

export const COLORS = Object.values(Colors).filter((c) => typeof c === "string");

type ColorKey = keyof typeof Colors;

export function decodedValue(input: string[]): number {
  // Assumption, input is at least length 2
  return Colors[input[0].toLowerCase() as ColorKey] * 10 + 
         Colors[input[1].toLowerCase() as ColorKey];
}
