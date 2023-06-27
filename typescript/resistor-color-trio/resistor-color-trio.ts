enum Colors  { 
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

type ColorKey = keyof typeof Colors;

export function decodedResistorValue(input: string[]): string {
  let base = (Colors[input[0] as ColorKey] * 10 +
              Colors[input[1] as ColorKey]) *
             Math.pow(10, Colors[input[2] as ColorKey]);

  if(base > (Math.pow(10,9))) {
    return `${base / Math.pow(10,9)} gigaohms`;
  }
  if(base > (Math.pow(10,6))) {
    return `${base / Math.pow(10,6)} megaohms`;
  }
  if(base > (Math.pow(10,3))) {
    return `${base / Math.pow(10,3)} kiloohms`;
  }

  return `${base} ohms`;
}


