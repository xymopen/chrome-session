pub type CommandID = u8;

#[derive(Debug)]
#[non_exhaustive]
// Identifier for commands written to file.
pub enum CommandKind {
    SetTabWindow,
    // OBSOLETE Superseded by SetWindowBounds3.
    // SetWindowBounds,
    SetTabIndexInWindow,

    // OBSOLETE: Preserved for backward compatibility. Using
    // TabNavigationPathPruned instead
    TabNavigationPathPrunedFromBack,

    UpdateTabNavigation,
    SetSelectedNavigationIndex,
    SetSelectedTabInIndex,
    SetWindowType,
    // OBSOLETE Superseded by SetWindowBounds3. Except for data migration.
    // SetWindowBounds2,

    // OBSOLETE: Preserved for backward compatibility. Using
    // TabNavigationPathPruned instead
    TabNavigationPathPrunedFromFront,

    SetPinnedState,
    SetExtensionAppID,
    SetWindowBounds3,
    SetWindowAppName,
    TabClosed,
    WindowClosed,
    // OBSOLETE: Superseded by SetTabUserAgentOverride2.
    SetTabUserAgentOverride,
    SessionStorageAssociated,
    SetActiveWindow,
    LastActiveTime,
    // OBSOLETE Superseded by SetWindowWorkspace2.
    // SetWindowWorkspace,
    SetWindowWorkspace2,
    TabNavigationPathPruned,
    SetTabGroup,
    // OBSOLETE Superseded by SetTabGroupMetadata2.
    // SetTabGroupMetadata,
    SetTabGroupMetadata2,
    SetTabGuid,
    SetTabUserAgentOverride2,
    SetTabData,
    SetWindowUserTitle,
    SetWindowVisibleOnAllWorkspaces,
}

impl CommandKind {
    pub fn from(x: CommandID) -> Option<CommandKind> {
        match x {
            0 => Some(CommandKind::SetTabWindow),
            // 1 => Some(CommandKind::SetWindowBounds),
            2 => Some(CommandKind::SetTabIndexInWindow),
            5 => Some(CommandKind::TabNavigationPathPrunedFromBack),
            6 => Some(CommandKind::UpdateTabNavigation),
            7 => Some(CommandKind::SetSelectedNavigationIndex),
            8 => Some(CommandKind::SetSelectedTabInIndex),
            9 => Some(CommandKind::SetWindowType),
            // 10 => Some(CommandKind::SetWindowBounds2),
            11 => Some(CommandKind::TabNavigationPathPrunedFromFront),
            12 => Some(CommandKind::SetPinnedState),
            13 => Some(CommandKind::SetExtensionAppID),
            14 => Some(CommandKind::SetWindowBounds3),
            15 => Some(CommandKind::SetWindowAppName),
            16 => Some(CommandKind::TabClosed),
            17 => Some(CommandKind::WindowClosed),
            18 => Some(CommandKind::SetTabUserAgentOverride),
            19 => Some(CommandKind::SessionStorageAssociated),
            20 => Some(CommandKind::SetActiveWindow),
            21 => Some(CommandKind::LastActiveTime),
            // 22 => Some(CommandKind::SetWindowWorkspace),
            23 => Some(CommandKind::SetWindowWorkspace2),
            24 => Some(CommandKind::TabNavigationPathPruned),
            25 => Some(CommandKind::SetTabGroup),
            // 26 => Some(CommandKind::SetTabGroupMetadata),
            27 => Some(CommandKind::SetTabGroupMetadata2),
            28 => Some(CommandKind::SetTabGuid),
            29 => Some(CommandKind::SetTabUserAgentOverride2),
            30 => Some(CommandKind::SetTabData),
            31 => Some(CommandKind::SetWindowUserTitle),
            32 => Some(CommandKind::SetWindowVisibleOnAllWorkspaces),
            _ => None,
        }
    }
}

pub struct Command<'a> {
    id: CommandID,
    payload: &'a [u8],
}

impl Command<'_> {
    pub fn from(data: &[u8]) -> Command<'_> {
        Command {
            id: data[0],
            payload: &data[1..],
        }
    }

    pub fn id(&self) -> CommandID {
        self.id
    }

    pub fn payload(&self) -> &[u8] {
        self.payload
    }

    pub fn kind(&self) -> Option<CommandKind> {
        CommandKind::from(self.id)
    }
}
