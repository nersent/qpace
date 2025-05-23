import * as _deepMerge from "deepmerge";

export const deepMerge = <T = any, K = any, R = any>(
  target: T,
  source: K,
): R => {
  return _deepMerge.all([target as any, source as any]) as R;
};
