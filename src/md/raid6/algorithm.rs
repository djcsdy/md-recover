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
    ParityN6
}