export const tryParseInt = (
  value: number | string | undefined,
): number | undefined => {
  if (value == null) return;
  if (typeof value === "string") value = parseInt(value.trim());
  return Number.isSafeInteger(value) ? value : undefined;
};
