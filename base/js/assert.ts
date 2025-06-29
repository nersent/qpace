export const assert = (
  condition: boolean,
  message?: string | Error,
  ...data: any[]
): void => {
  if (!condition) {
    if (data.length > 0) {
      console.trace(data);
    }
    if (message instanceof Error) throw message;
    throw new Error(message || "Assertion failed");
  }
};

export const assertNonNull = <T>(
  value: T | undefined,
  message?: string | Error,
  ...data: any[]
): value is T => {
  if (value == null) {
    if (data.length > 0) {
      console.trace(data);
    }
    if (message instanceof Error) throw message;
    throw new Error(message || "Assertion failed");
  }
  return value != null;
};

export const unwrap = <T = any>(
  val?: T,
  message?: any,
  ...data: any[]
): NonNullable<T> => {
  if (val == null) {
    if (data.length > 0) {
      console.trace(data);
    }
    if (message instanceof Error) throw message;
    throw new Error(message || "Value is empty");
  }
  return val;
};
