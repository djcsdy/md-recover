pub enum Raid5Algorithm {
    /// Rotating Parity N with Data Restart
    LeftAsymmetric,
    /// Rotating Parity 0 with Data Restart
    RightAsymmetric,
    /// Rotating Parity N with Data Continuation
    LeftSymmetric,
    /// Rotating Parity 0 with Data Continuation
    RightSymmetric,
    /// P is initial device
    Parity0,
    /// P is final device
    ParityN,
}

impl Raid5Algorithm {
    pub fn from_layout(layout: u32) -> Option<Self> {
        match layout {
            0 => Some(Self::LeftAsymmetric),
            1 => Some(Self::RightAsymmetric),
            2 => Some(Self::LeftSymmetric),
            3 => Some(Self::RightSymmetric),
            4 => Some(Self::Parity0),
            5 => Some(Self::ParityN),
            _ => None,
        }
    }
}
