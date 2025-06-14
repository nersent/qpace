import { MD5 as md5 } from "object-hash";

export type Os = "macos" | "linux" | "windows" | "unknown";
export type Arch = "x86_64" | "arm64" | "unknown";

export const TARGETS = [
  "python-x86_64-linux",
  "python-x86_64-windows",
  "python-x86_64-macos",
  "python-arm64-macos",
  "python-arm64-linux",
  "wasm-unknown-unknown",
  "node-x86_64-linux",
  "node-x86_64-windows",
  "node-x86_64-macos",
  "node-arm64-macos",
  "node-arm64-linux",
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
  // noDeadCode?: boolean;
  /* Rust target config */
  rust?: RustConfig;
  /* Python target config */
  python?: PythonConfig;
  /* JS/Wasm target config */
  wasm?: WasmConfig;
  /* Node target config */
  node?: NodeConfig;
  /* Emits compiled code to directory. default: `false` */
  emit?: boolean;
  /* Directory to emit compiled code and artifacts. default: `build` */
  out?: string;
  /* Files included in the build. default: `["**\/*.pine"]` */
  include?: string[];
  /* Files excluded from the build. default: `["node_modules", "build", "dist", "target", "__pycache__"]` */
  exclude?: string[];
}

export interface RustConfig {
  /* Name of qpace core cargo crate. default: `qpace_core` */
  qpaceCoreCrate?: string;
}

export interface PythonConfig {
  /* Name of python package being built. default: `qpace_artifact` */
  package?: string;
  /* Installs wheel everytime after building using `pip install`. default: `true` */
  install?: boolean;
  /* Tests the wheel after building by running test python code. default: `false` */
  test?: boolean;
  /* Name of qpace python package. default: `qpace` */
  qpacePackage?: string;
}

export type NodePackageManager = "auto" | "npm" | "yarn" | "pnpm" | "bun";

export interface NodeConfig {
  /* Name of node package being built. default: `qpace_artifact` */
  package?: string;
  /* Installs the package to the node_modules directory. default: `false` */
  install?: boolean;
  test?: boolean;
  packageManager?: NodePackageManager;
  qpacePackage?: string;
}

export interface WasmConfig {
  bindings?: boolean;
}

export const getDefaultConfig = (): Config => {
  return {
    rust: {
      qpaceCoreCrate: "qpace_core",
    },
    python: {
      package: "qpace_artifact",
      qpacePackage: "qpace",
    },
    node: {
      package: "qpace_artifact",
      qpacePackage: "qpace",
    },
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
    python: {
      package: pckgName,
      install: true,
      test: true,
    },
    node: {
      package: pckgName,
      install: true,
      test: true,
      packageManager: "auto",
    },
    buildDir: "build",
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
    node: {
      ...config.node,
      ...newConfig.node,
    },
    wasm: {
      ...config.wasm,
      ...newConfig.wasm,
    },
  };
};
