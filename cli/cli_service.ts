import { Command } from "commander";

export interface CliModule {
  build: () => Command[];
}

export class CliService implements CliModule {
  constructor(private readonly deps: { compilerModule: CliModule }) {}

  public build(): Command[] {
    const scriptCommand = new Command("script");
    this.deps.compilerModule
      .build()
      .forEach((r) => scriptCommand.addCommand(r));
    return [scriptCommand];
  }
}
