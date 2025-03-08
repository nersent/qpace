import { Config } from "./config";

export interface FileSystem {
  exists: (path: string) => Promise<boolean>;
  read(path: string): Promise<Buffer>;
  read(path: string, encoding: "utf8"): Promise<string>;
  write(path: string, data: Buffer): Promise<void>;
  write(path: string, data: string, encoding: "utf8"): Promise<void>;
}

export interface Driver {
  build(): Promise<void>;
  check(): Promise<void>;
}

export interface Program {
  getRootDir: () => string;
  getConfig: () => Config;
  getFileSystem: () => FileSystem;
}

// export interface DriverBuildOptions {
//   targetSpec?: TargetSpec;
//   logger?: Logger;
//   checkOnly?: boolean;
// }

// export abstract class Driver {
//   constructor(public readonly program: Program) {}

//   public abstract build(opts?: DriverBuildOptions): Promise<boolean>;
//   public async check(
//     opts?: Omit<DriverBuildOptions, "checkOnly">,
//   ): Promise<boolean> {
//     return await this.build({ ...opts, checkOnly: true });
//   }
// }

// export type DriverFc = (program: Program) => Driver;
