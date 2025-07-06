import { MD5 as md5 } from "object-hash";

export type Os = "macos" | "linux" | "windows" | "unknown";
export type Arch = "x86_64" | "arm64" | "unknown";

export const TARGETS = [
  "python-x86_64-linux",
  "python-x86_64-windows",
  "python-x86_64-macos",
  "python-arm64-macos",
  "python-arm64-linux",
  // "python-universal",
  //
  "node-x86_64-linux",
  "node-x86_64-windows",
  "node-x86_64-macos",
  "node-arm64-macos",
  "node-arm64-linux",
  "node-universal", // all platforms
  //
  "js-universal", // WASM + Node.js all platforms
  //
  "wasm-unknown-unknown",
] as const;

export type Target = typeof TARGETS[number];

export const archFromTarget = (target: Target): Arch => {
  if (target.includes("x86_64")) {
    return "x86_64";
  } else if (target.includes("arm64")) {
    return "arm64";
  }
  return "unknown";
};

export const osFromTarget = (target: Target): Os => {
  if (target.includes("linux")) {
    return "linux";
  } else if (target.includes("windows")) {
    return "windows";
  } else if (target.includes("macos")) {
    return "macos";
  }
  return "unknown";
};

export const QPC_DIR = ".qpace";
export const QPC_CONFIG_FILENAME = ".qpace.json";

export enum FileTag {
  QPC_PYTHON_WHEEL = "qpc_python_wheel",
}

export interface Config {
  [key: string]: any;
  // bindings?: boolean;
  /* Removes unused functions, variables, etc. default: `true` */
  noDeadCode?: boolean;
  /* Rust target config */
  rust?: RustConfig;
  /* Python target config */
  python?: PythonConfig;
  /* Node.js target config */
  node?: NodeConfig;
  /* JS/WASM/Browser target config */
  web?: WebConfig;
  // Universal JS target config
  js?: JsConfig;
  /* Emits compiled code to directory. default: `false` */
  emit?: boolean;
  /* Directory to emit compiled code and artifacts. default: `build` */
  outDir?: string;
  /* Files included in the build. default: `["**\/*.pine"]` */
  include?: string[];
  /* Files excluded from the build. default: `["node_modules", "build", "dist", "target", "__pycache__"]` */
  exclude?: string[];
  /* Installs artifact everytime after building using `pip install` or `npm install` ect. default: `true` */
  install?: boolean;
  /* Tests the artifact after building by running test code. default: `false` */
  test?: boolean;
}

export type RustConfig = Record<string, any> & {
  /* Name of qpace core cargo crate. default: `qpace_core` */
  qpaceCoreCrate?: string;
};

export type PythonConfig = Record<string, any> & {
  /* Name of python package being built. default: `qpace_artifact` */
  package?: string;
  /* Name of qpace python package. default: `qpace` */
  qpacePackage?: string;
};

export type NodePackageManager = "auto" | "npm" | "yarn" | "pnpm" | "bun";

export type NodeConfig = Record<string, any> & {};

export type WebConfig = Record<string, any> & {};

export type JsConfig = Record<string, any> & {
  /* Name of node package being built. default: `qpace_artifact` */
  package?:
    | string
    | {
        name: string;
        version?: string;
        description?: string;
        homepage?: string;
        author?: string;
        repository?: {
          type?: string;
          url?: string;
        };
      };
  packageManager?: NodePackageManager;
  qpacePackage?: string;
};

export const getDefaultConfig = (): Config => {
  return {
    noDeadCode: true,
    rust: {
      qpaceCoreCrate: "qpace_core",
    },
    python: {
      package: "qpace_artifact",
      qpacePackage: "qpace",
    },
    js: {
      package: "qpace_artifact",
      qpacePackage: "qpace",
    },
    node: {},
    web: {},
    emit: false,
    out: "build",
    include: ["**/*.pine"],
    exclude: [
      "node_modules",
      "build",
      "dist",
      "target",
      "__pycache__",
      QPC_DIR,
    ],
  };
};

export const getInitConfig = (): Config => {
  const pckgName = `qpace_script_${md5(Date.now()).slice(0, 6)}`;
  return {
    install: true,
    test: true,
    python: {
      package: pckgName,
    },
    js: {
      package: pckgName,
      packageManager: "auto",
    },
    outDir: "build",
    include: ["**/*.pine"],
  };
};

export const mergeConfigs = (config: Config, newConfig: Config): Config => {
  return {
    ...config,
    ...newConfig,
    rust: {
      ...config.rust,
      ...newConfig.rust,
    },
    python: {
      ...config.python,
      ...newConfig.python,
    },
    js: {
      ...config.js,
      ...newConfig.js,
    },
    node: {
      ...config.node,
      ...newConfig.node,
    },
    web: {
      ...config.web,
      ...newConfig.web,
    },
  };
};
