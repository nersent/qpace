import { resolve } from "path";

import * as Lib from "../core/qpace_core";

export const getLib = async (): Promise<typeof Lib> => {
  return (await import(resolve(__dirname, "../core/pkg/qpace_core.js"))) as any;
};

// export * from "../core/qpace_core";

// export { Lib };
