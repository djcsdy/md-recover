bitflags! {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
    pub struct Permissions: u16 {
        const SET_USER_ID = 0x0800;
        const SET_GROUP_ID = 0x0400;
        const STICKY = 0x0200;
        const USER_READ = 0x0100;
        const USER_WRITE = 0x0080;
        const USER_EXECUTE = 0x0040;
        const GROUP_READ = 0x0020;
        const GROUP_WRITE = 0x0010;
        const GROUP_EXECUTE = 0x0008;
        const OTHER_READ = 0x0004;
        const OTHER_WRITE = 0x0002;
        const OTHER_EXECUTE = 0x0001;
    }
}
