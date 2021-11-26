use crate::transmute;
use std::mem;

pub type CommandID = u8;

#[derive(Debug)]
#[non_exhaustive]
// Identifier for commands written to file.
pub enum CommandKind {
    UpdateTabNavigation,
}

impl CommandKind {
    pub fn from(x: CommandID) -> Option<CommandKind> {
        match x {
            6 => Some(CommandKind::UpdateTabNavigation),
            _ => None,
        }
    }
}

pub struct Command {
    id: CommandID,
    payload: Vec<u8>,
}

impl Command {
    pub fn id(&self) -> CommandID {
        self.id
    }

    pub fn payload(&self) -> &[u8] {
        self.payload.as_slice()
    }

    pub fn kind(&self) -> Option<CommandKind> {
        CommandKind::from(self.id)
    }
}

impl From<Vec<u8>> for Command {
    fn from(mut data: Vec<u8>) -> Self {
        let id = {
            let id_slice = &data[0..mem::size_of::<CommandID>()];
            let id_ref = unsafe { transmute::as_ref::<CommandID>(id_slice).unwrap() };
            *id_ref
        };

        let payload = data.split_off(mem::size_of::<CommandID>());

        return Command {
            id,
            payload,
        };
    }
}

// Various payload structures.
