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
pub enum GrainRoughness {
    Off,
    Weak,
    Strong,
}

#[derive(Debug)]
pub enum GrainSize {
    Off,
    Small,
    Large,
}

#[derive(Debug)]
pub enum ColorChrome {
    Off,
    Weak,
    Strong,
}

#[derive(Debug)]
pub enum ColorChromeFxBlue {
    Off,
    Weak,
    Strong,
}

#[derive(Debug)]
pub enum FilmMode {
    Provia,
    Velvia,
    Astia,
    ProNegStd,
    ProNegHi,
    ClassicChrome,
    Eterna,
    ClassicNegative,
    NostalgicNeg,
    RealaACE,
}

#[derive(Debug)]
pub struct FujifilmSettings {
    white_balance: WhiteBalance,
    white_balance_fine_tune: WhiteBalanceFineTune,
    sharpness: Sharpness,
    noise_reduction: NoiseReduction,
    clarity: i32,
    shadow: i32,
    highlight: i32,
    grain_roughness: GrainRoughness,
    grain_size: GrainSize,
    color_chrome: ColorChrome,
    color_chrome_fx_blue: ColorChromeFxBlue,
    film_mode: FilmMode,
}

impl FujifilmSettings {
    fn new() -> FujifilmSettings {
        FujifilmSettings {
            white_balance: WhiteBalance::Auto,
            sharpness: Sharpness::Normal,
            white_balance_fine_tune: WhiteBalanceFineTune { red: 0, blue: 0 },
            noise_reduction: NoiseReduction::Normal,
            clarity: 0,
            shadow: 0,
            highlight: 0,
            grain_roughness: GrainRoughness::Off,
            grain_size: GrainSize::Off,
            color_chrome: ColorChrome::Off,
            color_chrome_fx_blue: ColorChromeFxBlue::Off,
            film_mode: FilmMode::Provia,
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

fn slurp_i32(data: &[u8], offset: &mut usize) -> i32 {
    let mut num = [0u8; 4];
    num.copy_from_slice(&data[*offset..*offset + 4]);
    *offset += 4;
    i32::from_le_bytes(num)
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

                                // TODO: this could be more intelligent
                                offset -= 4;
                                let clarity = slurp_i32(v, &mut offset) / 1000;
                                // let clarity = read_u32(v, data_value as usize);
                                // let s = NoiseReduction::from_u16(data_value as u16);
                                println!("{:?}", clarity);
                                result.clarity = clarity;
                            }
                            0x1040 => {
                                println! {"  match - shadow"}
                                offset -= 4;
                                let shadow = slurp_i32(v, &mut offset);
                                // let clarity = read_u32(v, data_value as usize);
                                // let s = NoiseReduction::from_u16(data_value as u16);
                                println!("{:?}", shadow);

                                result.shadow = match shadow {
                                    -64 => 4,
                                    -48 => 3,
                                    -32 => 2,
                                    -16 => 1,
                                    0 => 0,
                                    16 => -1,
                                    32 => -2,
                                    _ => panic!(""),
                                };
                            }
                            0x1041 => {
                                println! {"  match - highlight"}
                                offset -= 4;
                                let highlight = slurp_i32(v, &mut offset);
                                // let clarity = read_u32(v, data_value as usize);
                                // let s = NoiseReduction::from_u16(data_value as u16);
                                println!("{:?}", highlight);
                                result.highlight = match highlight {
                                    -64 => 4,
                                    -48 => 3,
                                    -32 => 2,
                                    -16 => 1,
                                    0 => 0,
                                    16 => -1,
                                    32 => -2,
                                    _ => panic!(""),
                                };
                            }
                            0x1047 => {
                                println! {"  match - grain roughness"}
                                offset -= 4;
                                let roughness = slurp_i32(v, &mut offset);
                                println!("{:?}", roughness);
                                result.grain_roughness = match roughness {
                                    0 => GrainRoughness::Off,
                                    32 => GrainRoughness::Weak,
                                    64 => GrainRoughness::Strong,
                                    _ => panic!(""),
                                };
                            }
                            0x1048 => {
                                println! {"  match - color chrome"}
                                offset -= 4;
                                let color_chrome = slurp_i32(v, &mut offset);
                                println!("{:?}", color_chrome);
                                result.color_chrome = match color_chrome {
                                    0 => ColorChrome::Off,
                                    32 => ColorChrome::Weak,
                                    64 => ColorChrome::Strong,
                                    _ => panic!(""),
                                };
                            }
                            0x104c => {
                                println! {"  match - grain effect size"}
                                offset -= 4;
                                let size = slurp_u16(v, &mut offset);
                                offset += 2;
                                println!("{:?}", size);
                                result.grain_size = match size {
                                    0 => GrainSize::Off,
                                    16 => GrainSize::Small,
                                    32 => GrainSize::Large,
                                    _ => panic!("weird grain effect"),
                                };
                            }
                            0x104e => {
                                println! {"  match - color chrome fx blue"}
                                offset -= 4;
                                let color_chrome = slurp_i32(v, &mut offset);
                                println!("{:?}", color_chrome);
                                result.color_chrome_fx_blue = match color_chrome {
                                    0 => ColorChromeFxBlue::Off,
                                    32 => ColorChromeFxBlue::Weak,
                                    64 => ColorChromeFxBlue::Strong,
                                    _ => panic!(""),
                                };
                            }
                            0x1401 => {
                                println! {"  match - film"}
                                offset -= 4;
                                let film = slurp_u16(v, &mut offset);
                                offset += 2;
                                println!("{:?}", film);
                                result.film_mode = match film {
                                    0x0 => FilmMode::Provia,
                                    0x120 => FilmMode::Astia,
                                    0x400 => FilmMode::Velvia,
                                    0x500 => FilmMode::ProNegStd,
                                    0x501 => FilmMode::ProNegHi,
                                    0x600 => FilmMode::ClassicChrome,
                                    0x700 => FilmMode::Eterna,
                                    0x800 => FilmMode::ClassicNegative,
                                    0xa00 => FilmMode::NostalgicNeg,
                                    0xb00 => FilmMode::RealaACE,
                                    _ => panic!(""),
                                };
                            }
                            _ => {}
                        }
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
