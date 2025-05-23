import pkg from "../package.json";

// export * from "../core/pkg/qpace_core";
// export * from "./client";

export const getVersion = (): string => {
  return pkg.version;
};
