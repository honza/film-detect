use serde::{Deserialize, Serialize, Serializer};
use std::io;

const MAKER_NOTES_TAG: u16 = 37500;

#[derive(Serialize, Deserialize, Debug)]
pub enum Saturation {
    Normal,
    MediumHigh,
    VeryHigh,
    Highest,
    High,
    MediumLow,
    Low,
    NoneBW,
    BWRed,
    BWYellow,
    BWGreen,
    BWSepia,
    VeryLow,
    Lowest,
    Acros,
    AcrosRed,
    AcrosYellow,
    AcrosGreen,
}

impl Saturation {
    fn from_u16(n: u16) -> Result<Self, FilmError> {
        match n {
            0x0 => Ok(Saturation::Normal),
            0x80 => Ok(Saturation::MediumHigh),
            0xc0 => Ok(Saturation::VeryHigh),
            0xe0 => Ok(Saturation::Highest),
            0x100 => Ok(Saturation::High),
            0x180 => Ok(Saturation::MediumLow),
            0x200 => Ok(Saturation::Low),
            0x300 => Ok(Saturation::NoneBW),
            0x301 => Ok(Saturation::BWRed),
            0x302 => Ok(Saturation::BWYellow),
            0x303 => Ok(Saturation::BWGreen),
            0x310 => Ok(Saturation::BWSepia),
            0x4c0 => Ok(Saturation::VeryLow),
            0x4e0 => Ok(Saturation::Lowest),
            0x500 => Ok(Saturation::Acros),
            0x501 => Ok(Saturation::AcrosRed),
            0x502 => Ok(Saturation::AcrosYellow),
            0x503 => Ok(Saturation::AcrosGreen),
            _ => Err(FilmError::UnexpectedValue(format!(
                "Failed to parse {} as saturation value.",
                n
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Sharpness {
    Softest = -4,  // -4  0x0
    VerySoft = -3, // -3  0x1
    Soft,          // -2  0x2
    MediumSoft,    // -1  0x82
    Normal = 0,    //  0  0x3
    MediumHard,    // +1  0x84
    Hard,          // +2  0x4
    VeryHard,      // +3  0x5
    Hardest,       // +4  0x6
}

impl Sharpness {
    fn from_u16(n: u16) -> Result<Self, FilmError> {
        match n {
            0x0 => Ok(Self::Softest),
            0x1 => Ok(Self::VerySoft),
            0x2 => Ok(Self::Soft),
            0x3 => Ok(Self::Normal),
            0x4 => Ok(Self::Hard),
            0x5 => Ok(Self::VeryHard),
            0x6 => Ok(Self::Hardest),
            0x82 => Ok(Self::MediumSoft),
            0x84 => Ok(Self::MediumHard),
            _ => Err(FilmError::UnexpectedValue(format!(
                "Failed to parse {} as sharpness value.",
                n
            ))),
        }
    }
}

impl Serialize for Sharpness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Softest => serializer.serialize_i32(-4),
            Self::VerySoft => serializer.serialize_i32(-3),
            Self::Soft => serializer.serialize_i32(-2),
            Self::MediumSoft => serializer.serialize_i32(-1),
            Self::Normal => serializer.serialize_i32(0),
            Self::MediumHard => serializer.serialize_i32(1),
            Self::Hard => serializer.serialize_i32(2),
            Self::VeryHard => serializer.serialize_i32(3),
            Self::Hardest => serializer.serialize_i32(4),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DynamicRange {
    Auto,
    DR100,
    DR200,
    DR400,
}

#[derive(Serialize, Deserialize, Debug)]
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
    fn from_u16(n: u16) -> Result<Self, FilmError> {
        match n {
            0x0 => Ok(Self::Auto),
            0x1 => Ok(Self::AutoWhitePriority),
            0x2 => Ok(Self::AutoAmbiancePriority),
            0x100 => Ok(Self::Daylight),
            0x200 => Ok(Self::Cloudy),
            0x300 => Ok(Self::DaylightFluorescent),
            0x301 => Ok(Self::DayWhiteFluorescent),
            0x302 => Ok(Self::WhiteFluorescent),
            0x303 => Ok(Self::WarmWhiteFluorescent),
            0x304 => Ok(Self::LivingRoomWarmWhiteFluorescent),
            0x400 => Ok(Self::Incandescent),
            0x500 => Ok(Self::Flash),
            0x600 => Ok(Self::Underwater),
            0xf00 => Ok(Self::Custom),
            0xf01 => Ok(Self::Custom2),
            0xf02 => Ok(Self::Custom3),
            0xf03 => Ok(Self::Custom4),
            0xf04 => Ok(Self::Custom5),
            0xff0 => Ok(Self::Kelvin),
            _ => Err(FilmError::UnexpectedValue(format!(
                "Failed to parse {} as white balance value.",
                n
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
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
    fn from_u16(n: u16) -> Result<Self, FilmError> {
        match n {
            0x0 => Ok(Self::Normal),
            0x100 => Ok(Self::Strong),
            0x180 => Ok(Self::MediumStrong),
            0x1c0 => Ok(Self::VeryStrong),
            0x1e0 => Ok(Self::Strongest),
            0x200 => Ok(Self::Weak),
            0x280 => Ok(Self::MediumWeak),
            0x2c0 => Ok(Self::VeryWeak),
            0x2e0 => Ok(Self::Weakest),
            _ => Err(FilmError::UnexpectedValue(format!(
                "Failed to parse {} as noise reduction value.",
                n
            ))),
        }
    }
}

impl Serialize for NoiseReduction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Strongest => serializer.serialize_i32(4),
            Self::VeryStrong => serializer.serialize_i32(3),
            Self::Strong => serializer.serialize_i32(2),
            Self::MediumStrong => serializer.serialize_i32(1),
            Self::Normal => serializer.serialize_i32(0),
            Self::MediumWeak => serializer.serialize_i32(-1),
            Self::Weak => serializer.serialize_i32(-2),
            Self::VeryWeak => serializer.serialize_i32(-3),
            Self::Weakest => serializer.serialize_i32(-4),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum GrainRoughness {
    Off,
    Weak,
    Strong,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GrainSize {
    Off,
    Small,
    Large,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorChrome {
    Off,
    Weak,
    Strong,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorChromeFxBlue {
    Off,
    Weak,
    Strong,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub enum Shadow {
    Plus4,
    Plus3,
    Plus2,
    Plus1,
    Zero,
    Minus1,
    Minus2,
}

impl Shadow {
    fn from_i32(n: i32) -> Result<Self, FilmError> {
        match n {
            -64 => Ok(Self::Plus4),
            -48 => Ok(Self::Plus3),
            -32 => Ok(Self::Plus2),
            -16 => Ok(Self::Plus1),
            0 => Ok(Self::Zero),
            16 => Ok(Self::Minus1),
            32 => Ok(Self::Minus2),
            _ => Err(FilmError::UnexpectedValue(format!(
                "Failed to parse {} as shadow value.",
                n
            ))),
        }
    }
}

impl Serialize for Shadow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Plus4 => serializer.serialize_i8(4),
            Self::Plus3 => serializer.serialize_i8(3),
            Self::Plus2 => serializer.serialize_i8(2),
            Self::Plus1 => serializer.serialize_i8(1),
            Self::Zero => serializer.serialize_i8(0),
            Self::Minus1 => serializer.serialize_i8(-1),
            Self::Minus2 => serializer.serialize_i8(-2),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Highlight {
    Plus4,
    Plus3,
    Plus2,
    Plus1,
    Zero,
    Minus1,
    Minus2,
}

impl Highlight {
    fn from_i32(n: i32) -> Result<Self, FilmError> {
        match n {
            -64 => Ok(Self::Plus4),
            -48 => Ok(Self::Plus3),
            -32 => Ok(Self::Plus2),
            -16 => Ok(Self::Plus1),
            0 => Ok(Self::Zero),
            16 => Ok(Self::Minus1),
            32 => Ok(Self::Minus2),
            _ => Err(FilmError::UnexpectedValue(format!(
                "Failed to parse {} as highlight value.",
                n
            ))),
        }
    }
}

impl Serialize for Highlight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Plus4 => serializer.serialize_i8(4),
            Self::Plus3 => serializer.serialize_i8(3),
            Self::Plus2 => serializer.serialize_i8(2),
            Self::Plus1 => serializer.serialize_i8(1),
            Self::Zero => serializer.serialize_i8(0),
            Self::Minus1 => serializer.serialize_i8(-1),
            Self::Minus2 => serializer.serialize_i8(-2),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FujifilmSettings {
    white_balance: WhiteBalance,
    white_balance_fine_tune: WhiteBalanceFineTune,
    sharpness: Sharpness,
    noise_reduction: NoiseReduction,
    clarity: i32,
    shadow: Shadow,
    highlight: Highlight,
    grain_roughness: GrainRoughness,
    grain_size: GrainSize,
    color_chrome: ColorChrome,
    color_chrome_fx_blue: ColorChromeFxBlue,
    film_mode: FilmMode,
    dynamic_range: DynamicRange,
    saturation: Saturation,
}

impl FujifilmSettings {
    fn new() -> FujifilmSettings {
        FujifilmSettings {
            white_balance: WhiteBalance::Auto,
            sharpness: Sharpness::Normal,
            white_balance_fine_tune: WhiteBalanceFineTune { red: 0, blue: 0 },
            noise_reduction: NoiseReduction::Normal,
            clarity: 0,
            shadow: Shadow::Zero,
            highlight: Highlight::Zero,
            grain_roughness: GrainRoughness::Off,
            grain_size: GrainSize::Off,
            color_chrome: ColorChrome::Off,
            color_chrome_fx_blue: ColorChromeFxBlue::Off,
            film_mode: FilmMode::Provia,
            dynamic_range: DynamicRange::Auto,
            saturation: Saturation::Normal,
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

impl std::fmt::Display for FilmMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Provia => write!(f, "Provia"),
            Self::Velvia => write!(f, "Velvia"),
            Self::Astia => write!(f, "Astia"),
            Self::ProNegStd => write!(f, "Pro Neg Std"),
            Self::ProNegHi => write!(f, "Pro Neg Hi"),
            Self::ClassicChrome => write!(f, "Classic Chrome"),
            Self::Eterna => write!(f, "Eterna"),
            Self::ClassicNegative => write!(f, "Classic Negative"),
            Self::NostalgicNeg => write!(f, "Nostalgic Neg"),
            Self::RealaACE => write!(f, "Reala ACE"),
        }
    }
}

impl std::fmt::Display for NoiseReduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Strongest => write!(f, "4"),
            Self::VeryStrong => write!(f, "3"),
            Self::Strong => write!(f, "2"),
            Self::MediumStrong => write!(f, "1"),
            Self::Normal => write!(f, "0"),
            Self::MediumWeak => write!(f, "-1"),
            Self::Weak => write!(f, "-2"),
            Self::VeryWeak => write!(f, "-3"),
            Self::Weakest => write!(f, "-4"),
        }
    }
}

impl std::fmt::Display for Shadow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus4 => write!(f, "4"),
            Self::Plus3 => write!(f, "3"),
            Self::Plus2 => write!(f, "2"),
            Self::Plus1 => write!(f, "1"),
            Self::Zero => write!(f, "0"),
            Self::Minus1 => write!(f, "-1"),
            Self::Minus2 => write!(f, "-2"),
        }
    }
}

impl std::fmt::Display for Highlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus4 => write!(f, "4"),
            Self::Plus3 => write!(f, "3"),
            Self::Plus2 => write!(f, "2"),
            Self::Plus1 => write!(f, "1"),
            Self::Zero => write!(f, "0"),
            Self::Minus1 => write!(f, "-1"),
            Self::Minus2 => write!(f, "-2"),
        }
    }
}

// white_balance_fine_tune: WhiteBalanceFineTune,
// sharpness: Sharpness,
// grain_roughness: GrainRoughness,
// grain_size: GrainSize,
// color_chrome: ColorChrome,
// color_chrome_fx_blue: ColorChromeFxBlue,
// dynamic_range: DynamicRange,
// saturation: Saturation,

impl std::fmt::Display for FujifilmSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "White Balance: {}
Film simulation: {}
Clarity: {}
Shadow: {}
Highlight: {}
Noise Reduction: {}
",
            self.white_balance,
            self.film_mode,
            self.clarity,
            self.shadow,
            self.highlight,
            self.noise_reduction
        )
    }
}

// TODO
const PORTRA: FujifilmSettings = FujifilmSettings {
    white_balance: WhiteBalance::Auto,
    sharpness: Sharpness::Normal,
    white_balance_fine_tune: WhiteBalanceFineTune { red: 0, blue: 0 },
    noise_reduction: NoiseReduction::Normal,
    clarity: 0,
    shadow: Shadow::Minus2,
    highlight: Highlight::Zero,
    grain_roughness: GrainRoughness::Off,
    grain_size: GrainSize::Off,
    color_chrome: ColorChrome::Off,
    color_chrome_fx_blue: ColorChromeFxBlue::Off,
    film_mode: FilmMode::Provia,
    dynamic_range: DynamicRange::Auto,
    saturation: Saturation::Normal,
};

fn slurp_string<'a>(data: &'a [u8], offset: &'a mut usize, length: usize) -> &'a str {
    let s = std::str::from_utf8(&data[*offset..(*offset + length)]).unwrap();
    *offset += length;
    s
}

// fn read_string(data: &[u8], offset: usize, length: usize) -> &str {
//     std::str::from_utf8(&data[offset..(offset + length)]).unwrap()
// }

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

pub enum FilmError {
    // We failed to read a file or something similar.
    IO(io::Error),
    // Exif parsing failed.
    Exif(exif::Error),
    // The provided file isn't a Fujifilm photo.
    NotAFujifilmFile,
    // Unexpected Fujifilm value
    UnexpectedValue(String),
}

impl From<io::Error> for FilmError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<exif::Error> for FilmError {
    fn from(error: exif::Error) -> Self {
        Self::Exif(error)
    }
}

pub fn get_fujifilm_settings(path: &std::path::Path) -> Result<FujifilmSettings, FilmError> {
    let mut result = FujifilmSettings::new();

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    for field in exif.fields() {
        if field.tag.number() == MAKER_NOTES_TAG {
            match field.value {
                exif::Value::Undefined(ref v, _index) => {
                    let mut offset: usize = 0;

                    let fujifilm = slurp_string(v, &mut offset, 8);

                    if fujifilm != "FUJIFILM" {
                        return Err(FilmError::NotAFujifilmFile);
                    }

                    while offset < v.len() {
                        let tag = slurp_u16(v, &mut offset);

                        // Fuji RAF, skip it
                        if tag == 0xc {
                            offset += 16;
                            continue;
                        }

                        let _data_type = slurp_u16(v, &mut offset);
                        let _comp_count = slurp_u32(v, &mut offset);

                        match tag {
                            0xc => {
                                return Err(FilmError::UnexpectedValue(format!(
                                    "Unexpected RAF data."
                                )));
                            }
                            0x1001 => {
                                let data_value = slurp_u32(v, &mut offset);
                                result.sharpness = Sharpness::from_u16(data_value as u16)?;
                            }
                            0x1002 => {
                                let data_value = slurp_u32(v, &mut offset);
                                result.white_balance = WhiteBalance::from_u16(data_value as u16)?;
                            }
                            0x1003 => {
                                let data_value = slurp_u32(v, &mut offset);
                                result.saturation = Saturation::from_u16(data_value as u16)?;
                            }
                            0x100a => {
                                let data_value = slurp_u32(v, &mut offset);
                                let red = read_i32(v, data_value as usize);
                                let blue = read_i32(v, (data_value + 4) as usize);

                                let wbft = WhiteBalanceFineTune::from_i32(red, blue);
                                result.white_balance_fine_tune = wbft;
                            }
                            0x100e => {
                                let data_value = slurp_u32(v, &mut offset);
                                result.noise_reduction =
                                    NoiseReduction::from_u16(data_value as u16)?;
                            }
                            0x100f => {
                                let clarity = slurp_i32(v, &mut offset) / 1000;
                                result.clarity = clarity;
                            }
                            0x1040 => {
                                let shadow = slurp_i32(v, &mut offset);
                                result.shadow = Shadow::from_i32(shadow)?;
                            }
                            0x1041 => {
                                let highlight = slurp_i32(v, &mut offset);
                                result.highlight = Highlight::from_i32(highlight)?;
                            }
                            0x1047 => {
                                let roughness = slurp_i32(v, &mut offset);
                                result.grain_roughness = match roughness {
                                    0 => GrainRoughness::Off,
                                    32 => GrainRoughness::Weak,
                                    64 => GrainRoughness::Strong,
                                    _ => {
                                        return Err(FilmError::UnexpectedValue(format!(
                                            "Failed to parse {} as grain roughness value.",
                                            roughness
                                        )))
                                    }
                                };
                            }
                            0x1048 => {
                                let color_chrome = slurp_i32(v, &mut offset);
                                result.color_chrome = match color_chrome {
                                    0 => ColorChrome::Off,
                                    32 => ColorChrome::Weak,
                                    64 => ColorChrome::Strong,
                                    _ => {
                                        return Err(FilmError::UnexpectedValue(format!(
                                            "Failed to parse {} as color chrome value.",
                                            color_chrome
                                        )))
                                    }
                                };
                            }
                            0x104c => {
                                let size = slurp_u16(v, &mut offset);
                                offset += 2;
                                result.grain_size = match size {
                                    0 => GrainSize::Off,
                                    16 => GrainSize::Small,
                                    32 => GrainSize::Large,
                                    _ => {
                                        return Err(FilmError::UnexpectedValue(format!(
                                            "Failed to parse {} as grain size value.",
                                            size
                                        )))
                                    }
                                };
                            }
                            0x104e => {
                                let color_chrome = slurp_i32(v, &mut offset);
                                result.color_chrome_fx_blue = match color_chrome {
                                    0 => ColorChromeFxBlue::Off,
                                    32 => ColorChromeFxBlue::Weak,
                                    64 => ColorChromeFxBlue::Strong,
                                    _ => {
                                        return Err(FilmError::UnexpectedValue(format!(
                                            "Failed to parse {} as color chrome fx blue value.",
                                            color_chrome
                                        )))
                                    }
                                };
                            }
                            0x1401 => {
                                let film = slurp_u16(v, &mut offset);
                                offset += 2;
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
                                    _ => return Err(FilmError::UnexpectedValue(format!("hi"))),
                                };
                            }
                            0x1403 => {
                                let dynamic = slurp_u16(v, &mut offset);
                                offset += 2;
                                result.dynamic_range = match dynamic {
                                    0 => DynamicRange::Auto,
                                    100 => DynamicRange::DR100,
                                    200 => DynamicRange::DR200,
                                    400 => DynamicRange::DR400,
                                    _ => {
                                        return Err(FilmError::UnexpectedValue(format!(
                                            "Failed to parse {} as dynamic range value.",
                                            dynamic
                                        )))
                                    }
                                };
                            }
                            _ => {
                                let _data_value = slurp_u32(v, &mut offset);
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    Ok(result)
}
