import { Command } from "commander";

// import { ApiClient } from "./api_client";
// import { CliService } from "./cli_service";
// import { CompilerCli } from "./compiler_cli";
// import { RemoteDriver } from "./remote_driver";

// const apiClient = new ApiClient({});
// const cliService = new CliService({
//   compilerModule: new CompilerCli(
//     (program) => new RemoteDriver(program, { apiClient }),
//   ),
// });
const program = new Command();
program.addCommand(new Command('build').action(() => console.log('build')));
// cliService.build().forEach((r) => program.addCommand(r));
program.parse();
