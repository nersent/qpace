import { Command } from "commander";
import { mkdir, readdir, readFile, writeFile } from "fs/promises";
import { basename, relative, resolve } from "path";
import {
  Config,
  getInitConfig,
  tryGetJsPackageName,
  QPC_CONFIG_FILENAME,
} from "~/compiler/config";
import { exists, writeJson } from "~/base/node/fs";
import {
  isValidNpmPackageName,
  isValidPipPackageName,
  loadQpcConfig,
  tryLoadQpcConfig,
} from "./compiler";
import { EXAMPLES_DIR, QPACE_BG_PREFIX } from "./common";
import { input, confirm, select, checkbox } from "@inquirer/prompts";
import chalk from "chalk";

interface Example {
  id: string;
  name: string;
}

const EXAMPLES: Example[] = [
  {
    id: "library",
    name: "library",
  },
  {
    id: "indicator",
    name: "indicator",
  },
];

const initProject = async ({ cwd }: { cwd: string }): Promise<void> => {
  cwd ??= process.cwd();
  await mkdir(cwd, { recursive: true });
  const configPath = resolve(cwd, QPC_CONFIG_FILENAME);
  let config: Config | undefined;
  let firstInitialize = false;
  if (await exists(configPath)) {
    config = await loadQpcConfig(configPath);
    console.log(`${QPACE_BG_PREFIX}Project already exists at ${configPath}`);
    firstInitialize = false;
  } else {
    console.log(`${QPACE_BG_PREFIX}Creating new project at ${configPath}`);
    config = getInitConfig();
    firstInitialize = true;
  }
  let newConfig: Config = { ...config };

  const usePython = await confirm({
    message: "Do you want to use Python?",
    default: true,
  });
  if (usePython) {
    const packageName = await input({
      message: `
Enter Python package name.
${chalk.yellowBright(`
import <PACKAGE_NAME> as pine
pine.my_indicator.custom_ma()
`)}
`,
      default: newConfig.python?.package ?? "qpace_artifact",
      required: false,
      validate: (x) => {
        if (!isValidPipPackageName(x)) {
          return "Invalid Pip package name";
        }
        return true;
      },
    });
    newConfig.python = {
      ...newConfig.python,
      package: packageName,
    };
    console.log();
  }

  const useJs = await confirm({
    message: "Do you want to use JavaScript?",
    default: true,
  });
  if (useJs) {
    const packageName = await input({
      message: `
Enter JavaScript package name.
${chalk.yellowBright(`
import * as pine from "<PACKAGE_NAME>"; 
pine.my_indicator.customMa()
`)}
`,
      default: tryGetJsPackageName(newConfig.js) ?? "qpace_artifact",
      required: false,
      validate: (x) => {
        if (!isValidNpmPackageName(x)) {
          return "Invalid NPM package name";
        }
        return true;
      },
    });
    newConfig.js = {
      ...newConfig.js,
      package: packageName,
    };
    console.log();
  }

  const useExample = await select({
    message: `Do you want to setup an example?`,
    default: true,
    choices: [
      {
        name: "yes",
        value: true,
      },
      {
        name: "no",
        value: false,
      },
    ],
  });
  if (useExample) {
    const exampleIds = await checkbox({
      message: "Select examples",
      choices: EXAMPLES.map((r) => ({
        value: r.id,
        name: r.name,
      })),
    });
    for (const exampleId of exampleIds) {
      const exampleDir = resolve(EXAMPLES_DIR, exampleId);
      const srcFiles = await readdir(exampleDir).then((files) =>
        files.map((r) => resolve(exampleDir, r)),
      );
      for (const srcFile of srcFiles) {
        const srcPath = resolve(exampleDir, srcFile);

        if (srcPath.endsWith(".py") && !usePython) {
          continue;
        }
        if (srcPath.endsWith(".js") && !useJs) {
          continue;
        }

        let destPath = resolve(cwd, basename(srcFile));
        if (await exists(destPath)) {
          const overwrite = await confirm({
            message: `File ${destPath} already exists. Do you want to overwrite it?`,
            default: true,
          });
          if (!overwrite) {
            destPath = resolve(cwd, `${Date.now()}_${srcFile}`);
          }
        }
        let content = await readFile(srcPath, "utf8");
        content = content.replaceAll(
          "__QPC_PYTHON_PACKAGE__",
          newConfig.python?.package ?? "",
        );
        content = content.replaceAll(
          "__QPC_JS_PACKAGE__",
          tryGetJsPackageName(newConfig.js) ?? "",
        );
        await writeFile(destPath, content, "utf8");
        console.log(chalk.cyanBright(`→ ${relative(process.cwd(), destPath)}`));
      }
    }
    console.log();
  }

  await writeJson(configPath, newConfig, true);

  if (firstInitialize) {
    console.log(`${QPACE_BG_PREFIX}Project initialized`);
  } else {
    console.log(`${QPACE_BG_PREFIX}Project updated`);
  }
  console.log();
};

export const getCommands = (): Command[] => {
  return [
    new Command("init")
      .description("Interactive project initialization")
      .option("--cwd <path>", `Project root directory`)
      .action(initProject),
  ];
};
