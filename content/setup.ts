import { readdirSync } from "fs";
import { cp, mkdir, unlink, writeFile } from "fs/promises";
import { resolve } from "path";
import { exec } from "~/base/node/exec";
import * as tar from "tar";

const WORKSPACE_PATH = process.env["BAZED_WORKSPACE_ROOT"] ?? process.cwd();
const BUILD_DIR = resolve(WORKSPACE_PATH, "build");
const CONTENT_DIR = resolve(WORKSPACE_PATH, "content");

const main = async (): Promise<void> => {
  const args = process.argv.slice(2);
  const [target] = args;
  console.log(`Args: ${args.join(" ")}`);
  if (!target.length) {
    throw new Error("No target specified");
  }
  if (target === "init") {
    const nodeDir = resolve(CONTENT_DIR, "node");
    const webDir = resolve(CONTENT_DIR, "web");
    const pythonDir = resolve(CONTENT_DIR, "python");
    await mkdir(nodeDir, { recursive: true });
    await mkdir(webDir, { recursive: true });
    await mkdir(pythonDir, { recursive: true });
    await writeFile(resolve(nodeDir, "index.js"), "", "utf8");
    await writeFile(resolve(webDir, "index.js"), "", "utf8");
    await writeFile(resolve(pythonDir, "__init__"), "", "utf8");
  } else if (target === "node") {
    const tmpDir = resolve(CONTENT_DIR, ".tmp/node");
    const destDir = resolve(CONTENT_DIR, "node");
    await mkdir(tmpDir, { recursive: true });
    await exec({
      command: `pnpm dlx qpace build --target node-universal --out-dir ${tmpDir} --verbose`,
      verbose: true,
      cwd: CONTENT_DIR,
    });
    await mkdir(destDir, { recursive: true });
    tar.x({
      file: resolve(tmpDir, `qpace_content-1.0.0.tgz`),
      cwd: destDir,
      sync: true,
      strip: 1,
    });
    await unlink(resolve(destDir, "package.json"));
  } else if (target === "web") {
    const tmpDir = resolve(CONTENT_DIR, ".tmp/web");
    const destDir = resolve(CONTENT_DIR, "web");
    await mkdir(tmpDir, { recursive: true });
    await exec({
      command: `pnpm dlx qpace build --target web --out-dir ${tmpDir} --verbose`,
      verbose: true,
      cwd: CONTENT_DIR,
    });
    await mkdir(destDir, { recursive: true });
    tar.x({
      file: resolve(tmpDir, `qpace_content-1.0.0.tgz`),
      cwd: destDir,
      sync: true,
      strip: 1,
    });
    await unlink(resolve(destDir, "package.json"));
  } else if (target === "py") {
    const tmpDir = resolve(CONTENT_DIR, ".tmp/python");
    const destDir = resolve(CONTENT_DIR, "python");
    await mkdir(tmpDir, { recursive: true });
    await exec({
      command: `pnpm dlx qpace build --target python --out-dir ${tmpDir} --verbose`,
      verbose: true,
      cwd: CONTENT_DIR,
    });
    const wheelFilename = readdirSync(tmpDir).find((f) => f.endsWith(".whl"));
    if (wheelFilename == null)
      throw new Error("No wheel file found in tmp directory");
    await exec({
      command: `python -m wheel unpack ${resolve(
        tmpDir,
        wheelFilename,
      )} -d ${tmpDir}`,
      verbose: true,
    });
    const files = readdirSync(resolve(tmpDir), {
      withFileTypes: true,
    });
    const dir = files.find(
      (f) => f.isDirectory() && f.name.startsWith("qpace_content"),
    );
    if (!dir) {
      throw new Error("qpace_content directory not found in build/qpc");
    }
    const relPath = resolve(tmpDir, dir.name, "qpace_content");
    await cp(relPath, destDir, {
      recursive: true,
    });
    await writeFile(resolve(destDir, "py.typed"), "", "utf8");
  } else {
    throw new Error(`Unknown target: ${target}`);
  }
};

main();
