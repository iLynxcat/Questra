use std::fmt::Display;

pub enum CameraDirection {
    PlusXPlusZ = 0,
    MinusXPlusZ = 1,
    MinusXMinusZ = 2,
    PlusXMinusZ = 3,
}

impl CameraDirection {
    pub fn get_next(&self) -> Self {
        match self {
            Self::PlusXPlusZ => Self::MinusXPlusZ,
            Self::MinusXPlusZ => Self::MinusXMinusZ,
            Self::MinusXMinusZ => Self::PlusXMinusZ,
            Self::PlusXMinusZ => Self::PlusXPlusZ,
        }
    }
}

impl Display for CameraDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::PlusXPlusZ => "+x +z",
            Self::PlusXMinusZ => "+x -z",
            Self::MinusXMinusZ => "-x -z",
            Self::MinusXPlusZ => "-x +z",
        })
    }
}
