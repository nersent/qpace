export const QPC_CONFIG_FILENAME = ".qpc.json";

export interface CompilerOptions {
  [key: string]: any;
  rs?: {
    qpCrate?: string;
  };
  py?:
    | boolean
    | {
        module?: string;
        qpaceModule?: string;
        scriptBindings?: boolean;
      };
  lib?: string[];
  optimization?: {
    removeUnusedFunctions?: boolean;
    removeUnusedGlobals?: boolean;
  };
  obfuscation?: {
    symbols?: boolean;
  };
  emitOnly?: boolean;
}

export type Os = "macos" | "linux" | "windows";
export type Arch = "x86_64" | "arm64";
export type Target = `py`;

export interface ProgramOptions {
  [key: string]: any;
  compilerOptions?: CompilerOptions;
  buildOptions?: BuildOptions;
  include?: string[];
  exclude?: string[];
}

export interface BuildOptions {
  [key: string]: any;
  installWheel?: boolean;
  dir?: string;
}

export interface Program {
  options: ProgramOptions;
  host: CompilerHost;
}

export type Encoding = "binary" | "utf8";

export interface CompilerHost {
  rootNames: string[];
  fileExists: (filename: string) => Promise<boolean> | boolean;
  readFile(filename: string): Promise<Buffer | undefined> | Buffer | undefined;
  readFile(
    filename: string,
    encoding: "utf8",
  ): Promise<string | undefined> | string | undefined;
  writeFile(filename: string, data: Buffer): Promise<void> | void;
  writeFile(
    filename: string,
    data: string,
    encoding: "utf8",
  ): Promise<void> | void;
}

export interface DriverBuildOptions {
  targetSpec?: TargetSpec;
  logger?: Logger;
  checkOnly?: boolean;
}

export interface TargetSpec {
  os?: Os;
  arch?: Arch;
  target?: Target;
}

export abstract class Driver {
  constructor(public readonly program: Program) {}

  public abstract build(opts?: DriverBuildOptions): Promise<boolean>;
  public async check(
    opts?: Omit<DriverBuildOptions, "checkOnly">,
  ): Promise<boolean> {
    return await this.build({ ...opts, checkOnly: true });
  }
}

export type DriverFc = (program: Program) => Driver;
