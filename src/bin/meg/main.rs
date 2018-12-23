fn main() -> Result<(), std::io::Error> {
    meglang::write_elf_file("main".to_string())?;
    Ok(())
}
