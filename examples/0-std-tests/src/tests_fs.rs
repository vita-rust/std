use super::assert;
use anyhow::Context;
use std::fs;

pub fn test_fs_read_dir() -> anyhow::Result<()> {
    let mut data_found = false;

    for file in fs::read_dir("ux0:/").context("read_dir ux0 failed")? {
        let file = file.context("unable to get direntry of file")?;

        if file.file_name().to_str() == Some("data") {
            data_found = true;
            let meta = file.metadata().context("unable to stat ux0:/data")?;
            assert(meta.is_dir(), "ux:0/data is not a directory")?;
            assert(!meta.is_file(), "ux:0/data is a file")?;
            assert(!meta.is_symlink(), "ux:0/data is a symlink")?;
            assert(meta.created().is_ok(), "created time is error")?;
            assert(meta.modified().is_ok(), "modified time is error")?;
            assert(meta.accessed().is_ok(), "access time is error")?;
            break;
        }
    }

    assert(data_found, "ux0:/data not found")?;

    Ok(())
}

#[inline(never)]
pub fn test_fs_creation() -> anyhow::Result<()> {
    fs::create_dir("ux0:/data/.rust_test").context("unable to create ux0:/data/.rust_test")?;
    let meta =
        fs::metadata("ux0:/data/.rust_test").context("ux0:/data/.rust_test does not exist")?;
    assert(meta.is_dir(), "ux0:/data/.rust_test is not a directory")?;

    fs::write("ux0:/data/.rust_test/file", "contents").context("unable to create a test file")?;
    let data = fs::read("ux0:/data/.rust_test/file").context("unable to read file")?;
    assert(&data == "contents".as_bytes(), "invalid file contents")?;

    let data = fs::read_to_string("ux0:/data/.rust_test/file").context("unable to read file")?;
    assert(&data == "contents", "invalid file contents")?;

    assert(
        fs::try_exists("ux0:/data/.rust_test/file")?,
        "file does not exist",
    )?;

    fs::copy(
        "ux0:/data/.rust_test/file",
        "ux0:/data/.rust_test/file_copy",
    )
    .context("unable to copy file to file_copy")?;
    let data =
        fs::read_to_string("ux0:/data/.rust_test/file_copy").context("unable to read file_copy")?;
    assert(&data == "contents", "invalid file_copy contents")?;

    fs::remove_file("ux0:/data/.rust_test/file").context("unable to delete file")?;
    assert(
        !fs::try_exists("ux0:/data/.rust_test/file")?,
        "file exists, but should not",
    )?;

    assert(
        fs::remove_dir("ux0:/data/.rust_test").is_err(),
        "remove_dir should fail, because directory is not empty",
    )?;

    assert(
        fs::remove_dir_all("ux0:/data/.rust_test").is_ok(),
        "remove_dir_all should succeed",
    )?;

    Ok(())
}

pub fn fs_cleanup() {
    if fs::try_exists("ux0:/data/.rust_test").unwrap_or(false) {
        fs::remove_dir_all("ux0:/data/.rust_test").expect("unable to cleanup");
    }
}
