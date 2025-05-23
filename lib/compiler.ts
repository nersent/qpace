import { MD5 as md5 } from "object-hash";

export type Os = "macos" | "linux" | "windows" | "unknown";
export type Arch = "x86_64" | "arm64" | "unknown";

export const TARGETS = [
  "python-x86_64-linux",
  "python-x86_64-windows",
  "python-x86_64-macos",
  "python-arm64-macos",
  "python-arm64-linux",
] as const;

export type Target = typeof TARGETS[number];

export const extractArchFromTarget = (target: Target): Arch => {
  if (target.includes("x86_64")) {
    return "x86_64";
  } else if (target.includes("arm64")) {
    return "arm64";
  }
  return "unknown";
};

export const extractOsFromTarget = (target: Target): Os => {
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
  /* Compiler-specific config */
  compiler?: CompilerConfig;
  /* Rust target config */
  rust?: RustConfig;
  /* Python target config */
  python?: PythonConfig;
  /* JS/Wasm target config */
  wasm?: WasmConfig;
  /* Emits compiled code to directory. default: `false` */
  emit?: boolean;
  /* Directory to emit compiled code. default: `.qpc` */
  emitDir?: string;
  /* Directory to output compilation artifacts. default: `build` */
  buildDir?: string;
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
  /* Generates python bindings. default: `true` */
  bindings?: boolean;
  /* Installs wheel everytime after building using `pip install`. default: `true` */
  installWheel?: boolean;
  /* Name of qpace python package. default: `qpace` */
  qpacePackage?: string;
  /* Name of python package being built. default: `qpace_artifact` */
  package?: string;
  /* Tests the wheel after building by running test python code. default: `false` */
  testWheel?: boolean;
}

export interface WasmConfig {
  bindings?: boolean;
}

export interface CompilerConfig {
  bindings?: boolean;
  /* Removes unused functions, variables, etc. default: `true` */
  noDeadCode?: boolean;
}

export const getDefaultConfig = (): Config => {
  return {
    compiler: {
      noDeadCode: true,
    },
    rust: {
      qpaceCoreCrate: "qpace_core",
    },
    python: {
      bindings: true,
      installWheel: true,
      qpacePackage: "qpace",
      package: "qpace_artifact",
      testWheel: true,
    },
    wasm: {
      bindings: false,
    },
    emit: false,
    emitDir: ".",
    buildDir: "build",
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
  return {
    python: {
      package: `qpace_script_${md5(Date.now()).slice(0, 6)}`,
      installWheel: true,
      testWheel: true,
    },
    buildDir: "build",
    include: ["**/*.pine"],
  };
};

export const mergeConfigs = (config: Config, newConfig: Config): Config => {
  return {
    ...config,
    ...newConfig,
    compiler: {
      ...config.compiler,
      ...newConfig.compiler,
    },
    rust: {
      ...config.rust,
      ...newConfig.rust,
    },
    python: {
      ...config.python,
      ...newConfig.python,
    },
    wasm: {
      ...config.wasm,
      ...newConfig.wasm,
    },
  };
};
