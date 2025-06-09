import { which } from "./os";

export const locatePython = async (): Promise<string | undefined> => {
  let path = await which("python3");
  path ??= await which("python");
  return path;
};
