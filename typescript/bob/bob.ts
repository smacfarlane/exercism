export function hey(message: string): string {
  if(isSilence(message)) {
     return "Fine. Be that way!";
  }
  if(isShouting(message) && isQuestion(message)) {
      return "Calm down, I know what I'm doing!";
  }
  if(isShouting(message)) {
      return "Whoa, chill out!"
  }
  if(isQuestion(message)) {
    return "Sure.";
  }
  return "Whatever."
}

function isQuestion(message: string): boolean {
  return (message.trim().slice(-1) === '?');
}

function isShouting(message: string): boolean {
  return  (/[a-zA-Z]/.test(message)) && 
          (message === message.toUpperCase())
}

function isSilence(message: string): boolean {
  return (message.trim() === "")
}