export function toRna(dna: string): string {
  let rna:string = "";

  [...dna].forEach((c,_) => {
    switch(c) {
      case 'G': { rna = rna + "C"; break; }
      case 'C': { rna = rna + "G"; break; }
      case 'T': { rna = rna + "A"; break; }
      case 'A': { rna = rna + "U"; break; }
      default: throw new Error(`Invalid input DNA. ${c}`)
    };
  });

  return rna
}
