pub enum Raid6Algorithm {
    /// Rotating Parity N with Data Restart
    LeftAsymmetric,
    /// Rotating Parity 0 with Data Restart
    RightAsymmetric,
    /// Rotating Parity N with Data Continuation
    LeftSymmetric,
    /// Rotating Parity 0 with Data Continuation
    RightSymmetric,
    /// P and Q are initial devices
    Parity0,
    /// P and Q are final devices
    ParityN,
    /// DDF v1.2 PRL=6 RLQ=1
    Rotating0Restart,
    /// DDF v1.2 PRL=6 RLQ=2
    RotatingNRestart,
    /// DDF v1.2 PRL=6 RLQ=3
    RotatingNContinue,
    /// Same as LeftAsymmetric but with Q always on the last device
    LeftAsymmetric6,
    /// Same as RightAsymmetric but with Q always on the last device
    RightAsymmetric6,
    /// Same as LeftSymmetric but with Q always on the last device
    LeftSymmetric6,
    /// Same as RightSymmetric but with Q always on the last device
    RightSymmetric6,
    /// Same as Parity0 but with Q always on the last device
    Parity06,
    /// Same as ParityN but with Q always on the last device
    ParityN6,
}

impl Raid6Algorithm {
    pub fn from_layout(layout: u32) -> Option<Self> {
        match layout {
            0 => Some(Self::LeftAsymmetric),
            1 => Some(Self::RightAsymmetric),
            2 => Some(Self::LeftSymmetric),
            3 => Some(Self::RightSymmetric),
            4 => Some(Self::Parity0),
            5 => Some(Self::ParityN),
            8 => Some(Self::Rotating0Restart),
            9 => Some(Self::RotatingNRestart),
            10 => Some(Self::RotatingNContinue),
            16 => Some(Self::LeftAsymmetric6),
            17 => Some(Self::RightAsymmetric6),
            18 => Some(Self::LeftSymmetric6),
            19 => Some(Self::RightSymmetric6),
            20 => Some(Self::Parity06),
            21 => Some(Self::ParityN6),
            _ => None,
        }
    }
}
