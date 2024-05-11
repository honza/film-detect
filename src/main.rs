use std::io;

fn load_string(data: &[u8], offset: usize, length: usize) -> &str {
    std::str::from_utf8(&data[offset..(offset + length)]).unwrap()
    // println!("parsed {}", x);
}

fn load_u16(data: &[u8], offset: usize) -> u16 {
    let mut num = [0u8; 2];
    num.copy_from_slice(&data[offset..offset + 2]);
    u16::from_le_bytes(num)
}

fn load_u32(data: &[u8], offset: usize) -> u32 {
    let mut num = [0u8; 4];
    num.copy_from_slice(&data[offset..offset + 4]);
    u32::from_le_bytes(num)
}

fn run() -> io::Result<()> {
    println!("Hello, world!");

    let path = "test-portra.jpg";

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    for f in exif.fields() {
        if f.tag.number() == 37500 {
            println!(
                "{:#01x} {} --- {} {} {}",
                f.tag.number(),
                f.tag.number(),
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );

            match f.value {
                exif::Value::Undefined(ref v, index) => {
                    let mut offset = 0;

                    let fujifilm = load_string(v, offset, 8);
                    offset += 8;

                    println!("header {}", fujifilm);

                    while offset < v.len() {
                        let tag = load_u16(v, offset);
                        offset += 2;

                        if tag == 0xc {
                            offset += 16;
                            continue;
                        }

                        let data_type = load_u16(v, offset);
                        offset += 2;

                        let comp_count = load_u32(v, offset);
                        offset += 4;
                        let data_value = load_u32(v, offset);
                        offset += 4;

                        println!(" tag --- {} {:#01x}", tag, tag);
                        println!(" data --- {}", data_type);
                        println!(" comp --- {}", comp_count);
                        println!(" data --- {}", data_value);

                        let serial = load_string(v, data_value as usize, comp_count as usize);
                        println!(" value --- {}", serial);

                        match tag {
                            0xc => {
                                panic!("RAF data shouldn't be here");
                                continue;
                            }
                            0x0010 => {
                                println! {"serial"}
                            }
                            _ => {
                                println!("no {} {:#01x}", tag, tag);
                            }
                        }

                        break;
                    }

                    // after the FUJIFILM, we should skip RAF (16 bytes)

                    for (i, byte) in v.iter().enumerate() {
                        if *byte == 0 {
                            println!("{} -", i);
                        } else {
                            if *byte < 33 || *byte > 127 {
                                println!("{} - {:#x} {}", i, byte, byte);
                            } else {
                                println!("{} - {:#x} {} '{}'", i, byte, byte, *byte as char);
                            }
                        }

                        // 722 - 0x47 71 'G'
                        // 723 - 0x14 20
                        // 724 - 0x2 2
                        // 725 -
                        // 726 - 0x20 32
                        // 727 -
                        // 728 -
                        // 729 -
                        // 730 - 0x6 6
                        // 731 - 0x4 4
                        // 732 -
                        // 733 -
                        //
                        //
                        //    47 14      | 02 00  | 20 00 00 00      | 06 04 00 00
                        //    Fuji Model | string | data byte length | index
                    }
                }
                _ => (),
            }
        }
    }

    Ok(())
}

fn main() {
    run();
}
