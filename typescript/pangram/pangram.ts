export function isPangram(input: string): boolean {
  if(input.length < 26) {  return false }

  let notSeen = new Map<String,boolean>();
  [..."abcdefghijklmnopqrstuvwxyz"].forEach((c,_) => { notSeen.set(c, false)});
  
  [...input].forEach((c,_) => {
      notSeen.delete(c.toLowerCase());
  });

  if(notSeen.size != 0) { return false }
  return true;
  
}
