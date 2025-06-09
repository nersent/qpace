import { exists, readJson, writeJson } from "~/base/node/fs";
import { Client } from "~/lib/node/client";
import { unwrap } from "../base/js/assert";
import { dirname, resolve } from "path";
import { homedir } from "os";
import { mkdir } from "fs/promises";
import axios from "axios";
import { CliError } from "./exceptions";
import chalk from "chalk";
import { QPACE_BG_PREFIX } from "./common";

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

  public static async load(id = "prod"): Promise<Profile> {
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
      this.client = new Client({
        apiKey: unwrap(this.data.apiKey, "API key is required"),
        apiBase: "http://0.0.0.0:3000/v1",
        grpcApiBase: "0.0.0.0:3001",
      });
    }
    return this.client;
  }

  public async ping(): Promise<void> {
    const client = await this.getClient();
    try {
      await client.me();
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
