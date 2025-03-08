export type Os = "macos" | "linux" | "windows" | "unknown";
export type Arch = "x86_64" | "arm64" | "unknown";
export type Target = `py` | "unknown";

export interface TargetTriple {
  os: Os;
  arch: Arch;
  target: Target;
}

export const QPC_CONFIG_FILENAME = ".qpace.json";

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
  package?: string
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
    },
    wasm: {
      bindings: false,
    },
    emit: false,
    emitDir: ".qpc",
    include: ["**/*.pine"],
    exclude: ["node_modules", "build", "dist", "target", "__pycache__"],
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
