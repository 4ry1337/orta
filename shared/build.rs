extern crate tonic_build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // tonic_build::configure().build_server(false).compile(
    //     &[
    //         "./protos/common.proto",
    //         "./protos/user.proto",
    //         "./protos/common.proto",
    //         "./protos/auth.proto",
    //         "./protos/user.proto",
    //         "./protos/article.proto",
    //         "./protos/list.proto",
    //         "./protos/series.proto",
    //         "./protos/comment.proto",
    //         "./protos/tag.proto",
    //         "./protos/storage.proto",
    //     ],
    //     &["protos"],
    // )?;

    tonic_build::compile_protos("./protos/common.proto")?;
    tonic_build::compile_protos("./protos/auth.proto")?;
    tonic_build::compile_protos("./protos/user.proto")?;
    tonic_build::compile_protos("./protos/article.proto")?;
    tonic_build::compile_protos("./protos/list.proto")?;
    tonic_build::compile_protos("./protos/series.proto")?;
    tonic_build::compile_protos("./protos/comment.proto")?;
    tonic_build::compile_protos("./protos/tag.proto")?;
    tonic_build::compile_protos("./protos/storage.proto")?;

    Ok(())
}
