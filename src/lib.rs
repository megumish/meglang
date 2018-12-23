use std::fs;
use std::io::Write;

const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const ELF_HEADER_SIZE: u16 = 0x40;
const ELF_FLAGS: u32 = 0x00;
const BITS_FLAG: u8 = 2; // 64bit
const ENDIAN_FLAG: u8 = 1; // little endian
const ELF_HEADER_VERSION: u8 = 1;
const OSABI_FLAG: u8 = 3; // Linux is 0x03
const MACHINE_ARCH: u16 = 0x3e; // x86-64 is 0x3e
const ELF_VERSION: u32 = 0;
pub fn write_elf_file(file_name: String) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(file_name)?;
    file.write_all(ELF_MAGIC)?; // magic
    file.write_all(&u8_to_bytes(BITS_FLAG))?; // class (32bit or 64bit)
    file.write_all(&u8_to_bytes(ENDIAN_FLAG))?; // data (endian)
    file.write_all(&u8_to_bytes(ELF_HEADER_VERSION))?; // version
    file.write_all(&u8_to_bytes(OSABI_FLAG))?; // osabi (os: Linux is 0x03)
    file.write_all(&u64_to_bytes(0))?; // padding

    let elf_type = 2; // EXEC is 0x02
    file.write_all(&u16_to_bytes(elf_type))?; // type (ELF type: EXEC is 0x02)
    file.write_all(&u16_to_bytes(MACHINE_ARCH))?; // machine (machine architecture: x86-64 is 0x3e)
    file.write_all(&u32_to_bytes(ELF_VERSION))?; // version

    let entry_point = 0x403e0;
    let program_header_offset = 0x40;
    let section_header_offset = 0x00;
    let program_headers_total_size = 0x00;
    let number_of_program_headers = 0x00;
    let section_headers_total_size = 0x00;
    let number_of_section_headers = 0x00;
    let section_header_string_table_index = 0x00;
    file.write_all(&u64_to_bytes(entry_point))?; // entry point
    file.write_all(&u64_to_bytes(program_header_offset))?; // program header offset
    file.write_all(&u64_to_bytes(section_header_offset))?; // section header offset
    file.write_all(&u32_to_bytes(ELF_FLAGS))?; // flags
    file.write_all(&u16_to_bytes(ELF_HEADER_SIZE))?; // elf header size
    file.write_all(&u16_to_bytes(program_headers_total_size))?; // program headers total size
    file.write_all(&u16_to_bytes(number_of_program_headers))?; // number of program headers
    file.write_all(&u16_to_bytes(section_headers_total_size))?; // section headers total size
    file.write_all(&u16_to_bytes(number_of_section_headers))?; // number of section headers
    file.write_all(&u16_to_bytes(section_header_string_table_index))?; // section header string table index
    Ok(())
}

fn u8_to_bytes(n: u8) -> [u8; 1] {
    [n]
}

fn u16_to_bytes(n: u16) -> [u8; 2] {
    [((n) & 0xff) as u8, ((n >> (1 * 8)) & 0xff) as u8]
}

fn u32_to_bytes(n: u32) -> [u8; 4] {
    [
        ((n) & 0xff) as u8,
        ((n >> (1 * 8)) & 0xff) as u8,
        ((n >> (2 * 8)) & 0xff) as u8,
        ((n >> (3 * 8)) & 0xff) as u8,
    ]
}

fn u64_to_bytes(n: u64) -> [u8; 8] {
    [
        ((n) & 0xff) as u8,
        ((n >> (1 * 8)) & 0xff) as u8,
        ((n >> (2 * 8)) & 0xff) as u8,
        ((n >> (3 * 8)) & 0xff) as u8,
        ((n >> (4 * 8)) & 0xff) as u8,
        ((n >> (5 * 8)) & 0xff) as u8,
        ((n >> (6 * 8)) & 0xff) as u8,
        ((n >> (7 * 8)) & 0xff) as u8,
    ]
}
