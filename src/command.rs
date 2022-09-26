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

// Various payload structures.
