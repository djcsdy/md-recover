use binary_layout::LayoutAs;
use std::convert::Infallible;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct MdDeviceRole(u32);

impl MdDeviceRole {
    const MAX_POSITION: u32 = 0xfeff;
    const SPARE: u32 = 0xffff;
    const FAULTY: u32 = 0xfffe;
    const JOURNAL: u32 = 0xfffd;

    pub(super) fn from_u16(value: u16) -> Self {
        Self(value.into())
    }

    pub fn is_valid(&self) -> bool {
        !(Self::MAX_POSITION..Self::JOURNAL).contains(&self.0)
    }

    pub fn position(&self) -> Option<u16> {
        if self.0 <= Self::MAX_POSITION {
            Some(self.0 as u16)
        } else {
            None
        }
    }

    pub fn is_spare(&self) -> bool {
        self.0 == Self::SPARE
    }

    pub fn is_faulty(&self) -> bool {
        self.0 == Self::FAULTY
    }

    pub fn is_journal(&self) -> bool {
        self.0 == Self::JOURNAL
    }

    pub fn is_invalid(&self) -> bool {
        self.0 > Self::MAX_POSITION && self.0 < Self::JOURNAL
    }
}

impl LayoutAs<u32> for MdDeviceRole {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn try_read(v: u32) -> Result<Self, Self::ReadError> {
        Ok(Self(v))
    }

    fn try_write(v: Self) -> Result<u32, Self::WriteError> {
        Ok(v.0)
    }
}
