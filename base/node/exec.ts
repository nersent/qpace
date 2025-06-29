import { spawn } from "child_process";
import { randomUUID } from "crypto";
import { writeFile } from "fs/promises";
import { resolve } from "path";

import { tmpDir } from "./fs";
import { isWindows } from "./os";

export interface ExecOptions {
  command: string;
  args?: string[];
  env?: Record<string, any>;
  shell?: string | boolean;
  usePowershell?: boolean;
  cwd?: string;
  escapeCommand?: boolean;
  io?: boolean | string;
  returnChild?: boolean;
  detached?: boolean;
  verbose?: boolean;
}

export interface ExecResult {
  stdout: string;
  stderr: string;
  exitCode: number;
  signal?: string;
  command: string;
}

export async function exec(options: ExecOptions): Promise<ExecResult> {
  let command: string = options.command;
  let args = options.args ?? [];

  if (options.usePowershell && isWindows()) {
    args.unshift("-Command", command);
    args = [`"${args.slice(1).join(" ")}"`];
    command = "powershell.exe";
  }

  if (options.escapeCommand) {
    command = `"${command}"`;
  }

  let commandStr = command;

  // Create a temporary script file if the command contains newlines
  let tmpScriptPath: string | undefined;
  if (commandStr.includes("\n")) {
    tmpScriptPath = resolve(await tmpDir(), `${randomUUID()}.sh`);
    await writeFile(tmpScriptPath, commandStr, "utf8");
  }

  const req = {
    command: commandStr,
    args: args ?? [],
    env: options.env ?? process.env,
    shell: options.shell ?? true,
    cwd: options.cwd ?? undefined,
    stdio:
      typeof options.io === "string"
        ? options.io
        : options.io
        ? "inherit"
        : "pipe",
    returnChild: options.returnChild,
  };

  const execRes = await new Promise<ExecResult>((resolvePromise) => {
    const env = req.env;
    const cwd = req.cwd != null ? resolve(req.cwd) : undefined;

    if (tmpScriptPath) {
      commandStr = `bash "${tmpScriptPath}"`;
    }

    const child = spawn(commandStr, req.args, {
      env,
      shell: req.shell,
      cwd,
      stdio: req.stdio as any,
      detached: options.detached,
    });

    if (req.returnChild) {
      return resolvePromise(child as any);
    }

    let stdout = "";
    const stderr = "";

    let exitCode: number | null = null;
    let signal: NodeJS.Signals | null = null;
    let killTimer: NodeJS.Timeout | undefined;

    let alreadyFinished = false;

    const finish = (): void => {
      if (alreadyFinished) {
        throw new Error("Already finished. This should not happen");
      }
      alreadyFinished = true;
      clearTimeout(killTimer);
      const res: ExecResult = {
        stdout,
        stderr,
        command: commandStr,
        exitCode: exitCode ?? 0,
        signal: signal ?? undefined,
      };
      resolvePromise(res);
    };

    child.stdout?.on("data", (data) => {
      stdout += data;
      if (options.verbose) {
        process.stdout.write(data);
      }
    });
    child.stderr?.on("data", (data) => {
      stdout += data;
      if (options.verbose) {
        process.stderr.write(data);
      }
    });

    child.on("exit", (_exitCode, _signal) => {
      exitCode = _exitCode;
      signal = _signal;
      finish();
    });
  });
  execRes.command = req.command;

  return execRes;
}
