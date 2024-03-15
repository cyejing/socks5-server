/// SOCKS5 command
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Command {
    Connect,
    Bind,
    Associate,
    Padding,
}

impl Command {
    const CONNECT: u8 = 0x01;
    const BIND: u8 = 0x02;
    const ASSOCIATE: u8 = 0x03;
    const PADDING: u8 = 0x04;
}

impl TryFrom<u8> for Command {
    type Error = u8;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            Self::CONNECT => Ok(Self::Connect),
            Self::BIND => Ok(Self::Bind),
            Self::ASSOCIATE => Ok(Self::Associate),
            Self::PADDING => Ok(Self::Padding),
            code => Err(code),
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Connect => Command::CONNECT,
            Command::Bind => Command::BIND,
            Command::Associate => Command::ASSOCIATE,
            Command::Padding => Command::PADDING,
        }
    }
}
