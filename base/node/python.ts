import { exec } from "./exec";
import { which } from "./os";

export const locatePython = async (): Promise<string | undefined> => {
  let path = await which("python3");
  path ??= await which("python");
  return path;
};

export const getPythonVersion = async (
  pythonPath?: string,
): Promise<string | undefined> => {
  try {
    const res = await exec({
      command: `${pythonPath ?? "python3"} --version`,
    });
    if (res.exitCode === 0) return res.stdout.trim();
  } catch (err) {}
  return;
};
