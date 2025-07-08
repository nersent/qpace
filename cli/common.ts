import chalk from "chalk";
import { resolve } from "path";

export const QPACE_BG_PREFIX = `${chalk
  .hex("#000")
  .bgHex("#7fee64")
  .bold("qPACE")} `;

export const EXAMPLES_DIR = resolve(__dirname, "examples");
