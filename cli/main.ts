import { Command } from "commander";
import * as compiler from "./compiler";
import { VERSION, CORE_VERSION } from "~/lib/node";
import { handleExceptions } from "./exceptions";
import * as compilerApi from "~/compiler/schema_pb";
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
        const compilerClient = compiler.getClient();
        const compilerInfo = await compiler.fetchInfo(compilerClient);
        data = {
          ...data,
          compiler: compilerInfo.version,
          compilerDate: compilerInfo.buildDate.toLocaleString(),
        };
      }
      console.table(data);
    });
  user.getCommands().forEach((r) => program.addCommand(r));
  compiler.getCommands().forEach((r) => program.addCommand(r));
  program.parse();
};

main();
