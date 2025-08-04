import { exists, readJson, writeJson } from "~/base/node/fs";
import { Client } from "~/lib/node/client";
import { dirname, resolve } from "path";
import { homedir } from "os";
import { mkdir } from "fs/promises";
import axios from "axios";
import { CliError } from "./exceptions";
import chalk from "chalk";
import { ChannelCredentials } from "@grpc/grpc-js";

export type ProfileData = {
  apiKey?: string;
};

const getProfileDataPath = (id: string): string => {
  return resolve(homedir(), ".qpace", `profile_${id}.json`);
};

export class Profile {
  private client: Client | undefined;
  private static instance: Profile | undefined;

  constructor(public readonly id: string, public readonly data: ProfileData) {}

  public static async load(id?: string): Promise<Profile> {
    if (process.env["DEV"]) {
      id ??= "dev";
    }
    id ??= "prod";
    const path = getProfileDataPath(id);
    let data: ProfileData = {};
    if (await exists(path)) {
      data = await readJson(path);
    }
    if (Profile.instance == null) {
      Profile.instance = new Profile(id, data);
    }
    return Profile.instance;
  }

  public async save(): Promise<void> {
    const path = getProfileDataPath(this.id);
    await mkdir(dirname(path), { recursive: true });
    await writeJson(path, this.data);
  }

  public async getClient(cache = true): Promise<Client> {
    if (this.client == null || !cache) {
      let apiBase = "https://api.qpace.dev/v1";
      let grpcApiBase = "grpc.qpace.dev";
      let apiKey = this.data.apiKey;
      let grpcCredentials = ChannelCredentials.createSsl();

      if (process.env["DEV"]) {
        apiBase = "http://0.0.0.0:3000/v1";
        grpcApiBase = "0.0.0.0:3001";
        grpcCredentials = ChannelCredentials.createInsecure();
      }

      if (process.env["QPACE_API_KEY"]?.length) {
        apiKey = process.env["QPACE_API_KEY"];
      }
      if (process.env["QPACE_API_BASE"]?.length) {
        apiBase = process.env["QPACE_API_BASE"];
      }
      if (process.env["QPACE_GRPC_API_BASE"]?.length) {
        grpcApiBase = process.env["QPACE_GRPC_API_BASE"];
      }

      if (!apiKey?.length) {
        throw new CliError(`API key is required`);
      }

      this.client = new Client({
        apiKey,
        apiBase,
        grpcApiBase,
        grpcCredentials,
      });
    }
    return this.client;
  }

  public async ping(): Promise<void> {
    const client = await this.getClient();
    try {
      await client.user.me();
    } catch (e) {
      if (axios.isAxiosError(e) && e.response?.status == 403) {
        throw new CliError(
          `Invalid API key. Try authenticating again using "${chalk.yellowBright(
            `qpace login`,
          )}"`,
        );
      }
      throw e;
    }
  }
}
