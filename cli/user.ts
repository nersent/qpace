import { Command } from "commander";
import { Profile } from "./profile";
import axios from "axios";
import { CliError } from "./exceptions";
import { QPACE_BG_PREFIX } from "./common";
import chalk from "chalk";
import { input } from "@inquirer/prompts";
import { Result } from "~/base/js/result";

export const getCommands = (): Command[] => {
  return [
    new Command("login")
      .argument("[api key]")
      .action(async (apiKey?: string) => {
        const handle = async (
          apiKey: string,
          verbose: boolean,
        ): Promise<Result<Profile, any>> => {
          const profile = await Profile.load();
          profile.data.apiKey = apiKey;
          const client = await profile.getClient(false);
          try {
            const user = await client.user.me();
            verbose &&
              console.log(
                `${QPACE_BG_PREFIX}Logged in as ${chalk.yellowBright(
                  user.firstName,
                )}`,
              );
          } catch (e) {
            if (axios.isAxiosError(e) && e.response?.status == 403) {
              return Result.err(
                new CliError("Invalid API key. Try authenticating again."),
              );
            }
            return Result.err(e);
          }
          return Result.ok(profile);
        };
        console.log(`${QPACE_BG_PREFIX}Logging into qpace.dev`);
        console.log(
          `${QPACE_BG_PREFIX}You can find your API key in your browser here: ${chalk.cyanBright(
            `https://qpace.dev/auth`,
          )}`,
        );

        if (apiKey == null) {
          console.log(
            `${QPACE_BG_PREFIX}Paste an API key here and press enter, or press ctrl+c to quit`,
          );
          apiKey = await input({
            message: "Enter your API key",
            transformer: (apiKey) => apiKey?.trim(),
            validate: async (apiKey) => {
              if (!apiKey.length) return "API key is required";
              const res = await handle(apiKey, false);
              if (Result.isErr(res)) return "API key is invalid";
              return true;
            },
          });
        }
        const profile = Result.unwrap(await handle(apiKey, true));
        await profile.save();
      }),
    new Command("logout").action(async () => {
      const profile = await Profile.load();
      profile.data.apiKey = undefined;
      await profile.save();
      console.log(`${QPACE_BG_PREFIX}Logged out successfully.`);
    }),
  ];
};
