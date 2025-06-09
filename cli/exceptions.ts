import chalk from "chalk";

export class CliError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "CliError";
  }
}

const handleException = (e: Error): void => {
  if (e instanceof Error && e.name === "ExitPromptError") {
    return;
  }
  if (e instanceof CliError) {
    console.log(chalk.redBright(e.message));
    process.exit(1);
  }
  throw e;
};

export const handleExceptions = () => {
  process.on("unhandledRejection", (reason, promise) => {
    handleException(reason as Error);
  });
  process.on("uncaughtException", (reason) => {
    handleException(reason as Error);
  });
};
