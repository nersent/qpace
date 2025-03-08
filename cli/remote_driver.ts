import { resolve } from "path";
import { Driver, Program } from "~/compiler/driver";
import { CompilerClient } from "~/compiler/proto/compiler_grpc_pb";
import { exists } from "~/base/node/fs";
import { readFile, writeFile } from "fs/promises";
import { FileSystem } from "~/compiler/driver";

export class OsFileSystem implements FileSystem {
  constructor(public readonly rootDir: string) {}

  public resolve(path: string): string {
    return resolve(this.rootDir, path);
  }

  public async exists(path: string): Promise<boolean> {
    path = this.resolve(path);
    return exists(path);
  }

  public read(path: string): Promise<Buffer>;
  public read(path: string, encoding: "utf8"): Promise<string>;
  public read(path: string, encoding?: "utf8"): Promise<Buffer | string> {
    path = this.resolve(path);
    return readFile(path, encoding);
  }

  public write(
    path: string,
    data: Buffer | string,
    encoding?: "utf8",
  ): Promise<void> {
    path = this.resolve(path);
    return writeFile(path, data, encoding);
  }
}

export class RemoteDriver implements Driver {
  constructor(
    public readonly program: Program,
    public readonly client: CompilerClient,
  ) {}

  public async build(): Promise<void> {}

  public async check(): Promise<void> {}
}
