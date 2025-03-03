import { readdir, readFile, writeFile } from "fs/promises";
import { basename, dirname, resolve } from "path";

import chalk from "chalk";
import { Argument, Command, Option } from "commander";
import { glob } from "glob";

import { wait } from "../../base/js/time";
import { getWorkspaceRoot } from "../../base/node/path";

import { CliModule } from "./cli_service";

import { unwrap } from "~/base/js/assert";
import { exec, ExecOptions } from "~/base/node/exec";
import { createDir, exists, readJson } from "~/base/node/fs";
import { createLoggerFromStdIo } from "~/base/node/logger";
import {
  DriverFc,
  ProgramOptions,
  Program,
  QPC_CONFIG_FILENAME,
  CompilerHost,
} from "~/pace/compiler/schema/driver";

export class CompilerCli implements CliModule {
  constructor(private readonly driverFc: DriverFc) {}

  public build(): Command[] {
    return [this.checkCommand, this.buildCommand];
    // .addCommand(this.fixCommand)
    // .addCommand(this.buildCommand);
  }

  private createProgram({
    options,
    cwd,
    rootNames,
    onWriteFile,
  }: {
    options: ProgramOptions;
    cwd: string;
    rootNames: string[];
    onWriteFile?: (path: string) => Promise<void> | void;
  }): Program {
    return {
      options,
      host: {
        rootNames,
        fileExists: (filename) => exists(resolve(cwd, filename)),
        readFile: (filename, encoding) => {
          return readFile(resolve(cwd, filename), encoding);
        },
        writeFile: async (filename, data, encoding) => {
          const dir =
            filename.includes("/") || filename.includes(`\\`)
              ? dirname(filename)
              : undefined;
          if (dir != null) await createDir(resolve(cwd, dir));
          const path = resolve(cwd, filename);
          await writeFile(path, data, encoding);
          await onWriteFile?.(path);
        },
      } as CompilerHost,
    };
  }

  private async readProgramConfig(
    path: string,
    filenames?: string[],
  ): Promise<{ path: string; data: ProgramOptions }> {
    filenames ??= await readdir(path);
    if (filenames.includes(QPC_CONFIG_FILENAME)) {
      const configPath = resolve(path, QPC_CONFIG_FILENAME);
      const data: ProgramOptions = await readJson(configPath);
      data.include ??= ["./**/*.qp", "./**/*.pine"];
      data.exclude ??= ["node_modules", "__pycache__"];
      data.compilerOptions ??= {};
      data.compilerOptions.lib ??= ["stdlib"];
      data.buildOptions ??= {};
      data.buildOptions.dir = "build";
      return { path: configPath, data };
    }
    console.log(
      chalk.red(
        `qpScript config file \`${chalk.bold(
          QPC_CONFIG_FILENAME,
        )}\` not found in dir ${path}\nUse: qp script init --config`,
      ),
    );
    process.exit(1);
  }

  private shouldInstallWheel(options: ProgramOptions): boolean {
    return !!options.buildOptions?.installWheel;
  }

  private async collectRootNames({
    cwd,
    options,
  }: {
    cwd: string;
    options: ProgramOptions;
  }): Promise<string[]> {
    const excludes: string[] = [];
    const includes: string[] = [];

    const srcPaths: string[] = [];

    if (options != null) {
      excludes.push(...(options.exclude ?? []));
      includes.push(...(options.include ?? []));
    }

    srcPaths.push(
      ...(await Promise.all(
        includes.map((r) => glob(r, { cwd, ignore: excludes })),
      ).then((r) => r.flat())),
    );

    return srcPaths.map((r) => basename(r, cwd));
  }

  public checkCommand = new Command("check").action(async () => {
    // const cwd = process.cwd()
    // const cwd = `C:\\projects\\nersent\\prompusie\\pace\\playground`;
    const cwd = `C:\\projects\\nersent\\qpace\\examples\\script\\python_indicator`;
    const { data: options } = await this.readProgramConfig(cwd);
    const driver = this.driverFc(
      this.createProgram({
        rootNames: await this.collectRootNames({ cwd, options }),
        options,
        cwd,
      }),
    );
    const ok = await driver.check({ logger: createLoggerFromStdIo() });
    if (ok) {
      console.log(chalk.greenBright.bold(`ok`));
    } else {
      process.exit(1);
    }
  });

  public buildCommand = new Command("build")
    .option("--dir <path>", "working dir")
    .action(async (opts) => {
      // const cwd = process.cwd()
      // const cwd = `C:\\projects\\nersent\\prompusie\\pace\\playground`;
      const cwd = resolve(process.cwd(), opts.dir);
      const logger = createLoggerFromStdIo();
      // const cwd = `C:\\projects\\nersent\\qpace\\examples\\script\\python_indicator`;
      // const cwd = "/Users/xnerhu/Desktop/mikolaj/prompusie/pace/playground";
      const { data: options } = await this.readProgramConfig(cwd);
      const driver = this.driverFc(
        this.createProgram({
          rootNames: await this.collectRootNames({ cwd, options }),
          options,
          cwd,
          onWriteFile: async (path) => {
            if (
              this.shouldInstallWheel(driver.program.options) &&
              path.endsWith(".whl")
            ) {
              logger.log(chalk.blackBright(`Installing wheel`));
              await exec({
                command: `pip install "${path}" --force-reinstall`,
                io: true,
              });
            }
          },
        }),
      );
      const ok = await driver.build({
        logger,
        targetSpec: { arch: "x86_64", os: "windows", target: "py" },
        cwd,
      });
      if (ok) {
        console.log(chalk.greenBright.bold(`ok`));
      } else {
        process.exit(1);
      }
    });
}
