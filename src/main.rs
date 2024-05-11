use std::io;

#[derive(Debug)]
pub enum Sharpness {
    Softest,        // -4  0x0
    VerySoft,       // -3  0x1
    Soft,           // -2  0x2
    MediumSoft,     // -1  0x82
    Normal,         //  0  0x3
    MediumHard,     // +1  0x84
    Hard,           // +2  0x4
    VeryHard,       // +3  0x5
    Hardest,        // +4  0x6
    FilmSimulation, // 0x8000
}

impl Sharpness {
    fn from_u16(n: u16) -> Self {
        match n {
            0x0 => Self::Softest,
            0x1 => Self::VerySoft,
            0x2 => Self::Soft,
            0x3 => Self::Normal,
            0x4 => Self::Hard,
            0x5 => Self::VeryHard,
            0x6 => Self::Hardest,
            0x82 => Self::MediumSoft,
            0x84 => Self::MediumHard,
            _ => panic!("from_u16"),
        }
    }
}

#[derive(Debug)]
pub enum WhiteBalance {
    Auto,                           // 0x0
    AutoWhitePriority,              // 0x1
    AutoAmbiancePriority,           // 0x2
    Daylight,                       // 0x100
    Cloudy,                         // 0x200
    DaylightFluorescent,            // 0x300
    DayWhiteFluorescent,            // 0x301
    WhiteFluorescent,               // 0x302
    WarmWhiteFluorescent,           // 0x303
    LivingRoomWarmWhiteFluorescent, // 0x304
    Incandescent,                   // 0x400
    Flash,                          // 0x500
    Underwater,                     // 0x600
    Custom,                         // 0xf00
    Custom2,                        // 0xf01
    Custom3,                        // 0xf02
    Custom4,                        // 0xf03
    Custom5,                        // 0xf04
    Kelvin,                         // 0xff0
}

impl WhiteBalance {
    fn from_u16(n: u16) -> Self {
        match n {
            0x0 => Self::Auto,
            0x1 => Self::AutoWhitePriority,
            0x2 => Self::AutoAmbiancePriority,
            0x100 => Self::Daylight,
            0x200 => Self::Cloudy,
            0x300 => Self::DaylightFluorescent,
            0x301 => Self::DayWhiteFluorescent,
            0x302 => Self::WhiteFluorescent,
            0x303 => Self::WarmWhiteFluorescent,
            0x304 => Self::LivingRoomWarmWhiteFluorescent,
            0x400 => Self::Incandescent,
            0x500 => Self::Flash,
            0x600 => Self::Underwater,
            0xf00 => Self::Custom,
            0xf01 => Self::Custom2,
            0xf02 => Self::Custom3,
            0xf03 => Self::Custom4,
            0xf04 => Self::Custom5,
            0xff0 => Self::Kelvin,
            _ => panic!("from_u16"),
        }
    }
}

#[derive(Debug)]
pub enum NoiseReduction {
    Normal,       // 0 (normal) 0x0
    Strong,       // +2 (strong) 0x100
    MediumStrong, // +1 (medium strong) 0x180
    VeryStrong,   // +3 (very strong) 0x1c0
    Strongest,    // +4 (strongest) 0x1e0
    Weak,         // -2 (weak) 0x200
    MediumWeak,   // -1 (medium weak) 0x280
    VeryWeak,     // -3 (very weak) 0x2c0
    Weakest,      // -4 (weakest) 0x2e0
}

impl NoiseReduction {
    fn from_u16(n: u16) -> Self {
        match n {
            0x0 => Self::Normal,
            0x100 => Self::Strong,
            0x180 => Self::MediumStrong,
            0x1c0 => Self::VeryStrong,
            0x1e0 => Self::Strongest,
            0x200 => Self::Weak,
            0x280 => Self::MediumWeak,
            0x2c0 => Self::VeryWeak,
            0x2e0 => Self::Weakest,
            _ => panic!("from_u16"),
        }
    }
}
#[derive(Debug)]
pub struct WhiteBalanceFineTune {
    red: i8,
    blue: i8,
}

impl WhiteBalanceFineTune {
    fn from_i32(red: i32, blue: i32) -> Self {
        let red = red / 20;
        let blue = blue / 20;
        WhiteBalanceFineTune {
            red: red as i8,
            blue: blue as i8,
        }
    }
}

#[derive(Debug)]
pub struct FujifilmSettings {
    white_balance: WhiteBalance,
    white_balance_fine_tune: WhiteBalanceFineTune,
    sharpness: Sharpness,
    noise_reduction: NoiseReduction,
}

impl FujifilmSettings {
    fn new() -> FujifilmSettings {
        FujifilmSettings {
            white_balance: WhiteBalance::Auto,
            sharpness: Sharpness::Normal,
            white_balance_fine_tune: WhiteBalanceFineTune { red: 0, blue: 0 },
            noise_reduction: NoiseReduction::Normal,
        }
    }
}

impl std::fmt::Display for WhiteBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Daylight => write!(f, "Daylight"),
            _ => write!(f, "not implemented"),
        }
    }
}

impl std::fmt::Display for FujifilmSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "White Balance: {}", self.white_balance)
    }
}

fn slurp_string<'a>(data: &'a [u8], offset: &'a mut usize, length: usize) -> &'a str {
    let s = std::str::from_utf8(&data[*offset..(*offset + length)]).unwrap();
    *offset += length;
    s
}

fn read_string(data: &[u8], offset: usize, length: usize) -> &str {
    std::str::from_utf8(&data[offset..(offset + length)]).unwrap()
}

fn read_i32(data: &[u8], offset: usize) -> i32 {
    let mut num = [0u8; 4];
    num.copy_from_slice(&data[offset..offset + 4]);
    i32::from_le_bytes(num)
}

fn read_u32(data: &[u8], offset: usize) -> u32 {
    let mut num = [0u8; 4];
    num.copy_from_slice(&data[offset..offset + 4]);
    u32::from_le_bytes(num)
}

fn slurp_u16(data: &[u8], offset: &mut usize) -> u16 {
    let mut num = [0u8; 2];
    num.copy_from_slice(&data[*offset..*offset + 2]);
    *offset += 2;
    u16::from_le_bytes(num)
}

fn slurp_u32(data: &[u8], offset: &mut usize) -> u32 {
    let mut num = [0u8; 4];
    num.copy_from_slice(&data[*offset..*offset + 4]);
    *offset += 4;
    u32::from_le_bytes(num)
}

fn run() -> io::Result<()> {
    println!("Hello, world!");

    let mut result = FujifilmSettings::new();

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
                exif::Value::Undefined(ref v, _index) => {
                    let mut offset: usize = 0;

                    let fujifilm = slurp_string(v, &mut offset, 8);
                    println!("header {}", fujifilm);

                    while offset < v.len() {
                        let tag = slurp_u16(v, &mut offset);

                        if tag == 0xc {
                            offset += 16;
                            continue;
                        }

                        // TODO: when data_type is 9, data_value must be signed
                        let data_type = slurp_u16(v, &mut offset);
                        let comp_count = slurp_u32(v, &mut offset);
                        let data_value = slurp_u32(v, &mut offset);

                        println!(" tag --- {} {:#01x}", tag, tag);
                        println!(" data --- {}", data_type);
                        println!(" comp --- {}", comp_count);
                        println!(" data --- {}", data_value);

                        if data_type == 2 {
                            let serial = read_string(v, data_value as usize, comp_count as usize);
                            println!(" value --- {}", serial);
                        }

                        match tag {
                            0xc => {
                                panic!("RAF data shouldn't be here");
                            }
                            0x0010 => {
                                println! {"  match serial"}
                            }
                            0x1000 => {
                                println! {"  match - quality"}
                            }
                            0x1001 => {
                                println! {"  match - shapness"}
                                let s = Sharpness::from_u16(data_value as u16);
                                println!("{:?}", s);

                                result.sharpness = s;
                            }
                            0x1002 => {
                                println! {"  match - white balance"}
                                let s = WhiteBalance::from_u16(data_value as u16);
                                println!("{:?}", s);
                                result.white_balance = s;
                            }
                            0x100a => {
                                println! {"  match - white balance fine tune"}
                                let red = read_i32(v, data_value as usize);
                                let blue = read_i32(v, (data_value + 4) as usize);

                                let wbft = WhiteBalanceFineTune::from_i32(red, blue);
                                println!("  ft {:?}", wbft);
                                result.white_balance_fine_tune = wbft;
                            }
                            0x100e => {
                                println! {"  match - noise reduction"}
                                let s = NoiseReduction::from_u16(data_value as u16);
                                println!("{:?}", s);
                                result.noise_reduction = s;
                            }
                            0x100f => {
                                println! {"  match - clarity"}
                                // let clarity = read_u32(v, data_value as usize);
                                // let s = NoiseReduction::from_u16(data_value as u16);
                                println!("{:?}", clarity);
                                // result.noise_reduction = s;
                            }
                            _ => {
                                // println!("no {} {:#01x}", tag, tag);
                            }
                        }

                        // break;
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

    println!("{}", result);
    println!("{:?}", result);

    Ok(())
}

fn main() {
    run();
}
