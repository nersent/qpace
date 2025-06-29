import { Command } from "commander";
import * as compiler from "./compiler";
import {
  version as VERSION,
  coreVersion as CORE_VERSION,
} from "~/package.json";
import { handleExceptions } from "./exceptions";
import * as user from "./user";

const main = async (): Promise<void> => {
  handleExceptions();
  const program = new Command();
  program
    .option("-v, --version", "Show version")
    .option("--skip-remote", "Skip remote checks", false)
    .action(async ({ skipRemote }: { skipRemote?: boolean }) => {
      let data: any = {
        qpace: VERSION,
        qpaceCore: CORE_VERSION,
      };
      if (!skipRemote) {
        try {
          const [compilerClient] = await compiler.getCompilerClient();
          const compilerInfo = await compiler.fetchInfo(compilerClient);
          data = {
            ...data,
            compiler: compilerInfo.version,
            compilerDate: compilerInfo.buildDate.toLocaleString(),
          };
        } catch (e) {
          console.log(e);
        }
      }
      console.table(data);
    });
  user.getCommands().forEach((r) => program.addCommand(r));
  compiler.getCommands().forEach((r) => program.addCommand(r));
  program.parse();
};

main();
