extern crate tonic_build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("./proto/auth.proto")?;
    tonic_build::compile_protos("./proto/resource.proto")?;
    tonic_build::compile_protos("./proto/storage.proto")?;
    Ok(())
}
