const RESULT = Symbol("result");
const ERR = Symbol("err");

export type Result<T, E = Error> =
  | {
      [RESULT]: true;
      value: T;
    }
  | {
      [RESULT]: false;
      [ERR]: E;
    };

export namespace Result {
  export const unwrap = <T, E = any>(result: Result<T, E>): T => {
    if (result[RESULT]) {
      return result.value;
    } else {
      throw result[ERR];
    }
  };

  export const isOk = <T, E = Error>(
    result: Result<T, E>,
  ): result is Extract<Result<T, E>, { [RESULT]: true }> => {
    return result[RESULT] === true;
  };

  export const isErr = <T, E = Error>(
    result: Result<T, E>,
  ): result is Extract<Result<T, E>, { [RESULT]: false }> => {
    return result[RESULT] === false;
  };

  export const ok = <T, E = Error>(value: T): Result<T, E> => {
    return {
      [RESULT]: true,
      value,
    };
  };

  export const err = <E = Error, T = any>(error: E): Result<T, E> => {
    return {
      [RESULT]: false,
      [ERR]: error,
    };
  };
}
