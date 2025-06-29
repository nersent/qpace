fn main() {
    #[cfg(feature = "bindings_py")]
    {
        pyo3_build_config::add_extension_module_link_args();
    }
}
