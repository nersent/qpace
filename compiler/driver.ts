export const QPC_CONFIG_FILENAME = ".qpc.json";

export interface FileSystem {
  exists: (path: string) => Promise<boolean> | boolean;
  read(filename: string): Promise<Buffer | undefined> | Buffer | undefined;
  read(
    path: string,
    encoding: "utf8",
  ): Promise<string | undefined> | string | undefined;
  write(path: string, data: Buffer): Promise<void> | void;
  write(
    path: string,
    data: string,
    encoding: "utf8",
  ): Promise<void> | void;
}

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

export interface TargetTriple {
  os: Os;
  arch: Arch;
  target: Target;
}

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


export interface CompilerHost {
  rootNames: string[];
}

export interface DriverBuildOptions {
  targetSpec?: TargetSpec;
  logger?: Logger;
  checkOnly?: boolean;
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
