cfg_if::cfg_if! {
if #[cfg(feature = "bindings_py")]
{
use qpace_core::stub_info;
}
}

#[cfg(feature = "bindings_py")]
fn py_main() -> pyo3_stub_gen::Result<()> {
    println!("Generating types");
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info")).init();
    let stub = stub_info()?;
    println!("{:#?}", stub);
    stub.generate()?;
    Ok(())
}

fn main() -> Result<(), ()> {
    #[cfg(feature = "bindings_py")]
    {
        py_main().unwrap();
    }
    Ok(())
}
