export const prettifyTime = (ms?: number): string => {
  if (ms == null) return "?";
  const hours = Math.floor(ms / 1000 / 60 / 60);
  const minutes = Math.floor((ms / 1000 / 60 / 60 - hours) * 60);
  const seconds = Math.floor(
    ((ms / 1000 / 60 / 60 - hours) * 60 - minutes) * 60,
  );

  let str = "";

  if (hours > 0) {
    str += `${hours}h `;
  }

  if (minutes > 0) {
    str += `${minutes}m `;
  }

  if (seconds > 0) {
    str += `${seconds}s`;
  }

  if (str.length === 0) {
    str = `${Math.round(ms)}ms`;
  }

  return str.trim();
};

export const secondsToMs = (seconds: number): number => {
  return seconds * 1000;
};

export const minutesToMs = (minutes: number): number => {
  return secondsToMs(minutes * 60);
};

export const hoursToMs = (hours: number): number => {
  return minutesToMs(hours * 60);
};

export const daysToMs = (days: number): number => {
  return hoursToMs(days * 24);
};

export const monthsToMs = (months: number): number => {
  return daysToMs(months * 30);
};

export const yearsToMs = (years: number): number => {
  return monthsToMs(years * 12);
};
