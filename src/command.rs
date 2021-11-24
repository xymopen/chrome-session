use crate::transmute;
use std::mem;

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
#[repr(C)]
#[derive(Debug)]
struct ClosedPayload {
    id: u8,
    close_time: i64,
}

#[repr(C)]
#[derive(Debug)]
struct WindowBoundsPayload2 {
    window_id: u8,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    is_maximized: bool,
}

#[repr(C)]
#[derive(Debug)]
struct WindowBoundsPayload3 {
    window_id: u8,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    show_state: i32,
}

type ActiveWindowPayload = u8;

#[repr(C)]
#[derive(Debug)]
struct IDAndIndexPayload {
    id: u8,
    index: i32,
}

type TabIndexInWindowPayload = IDAndIndexPayload;

type TabNavigationPathPrunedFromBackPayload = IDAndIndexPayload;

type SelectedNavigationIndexPayload = IDAndIndexPayload;

type SelectedTabInIndexPayload = IDAndIndexPayload;

type WindowTypePayload = IDAndIndexPayload;

type TabNavigationPathPrunedFromFrontPayload = IDAndIndexPayload;

#[repr(C)]
#[derive(Debug)]
struct TabNavigationPathPrunedPayload {
    id: u8,
    // Index starting which |count| entries were removed.
    index: i32,
    // Number of entries removed.
    count: i32,
}

#[repr(C)]
#[derive(Debug)]
struct SerializedToken {
    // These fields correspond to the high and low fields of |base::Token|.
    id_high: u64,
    id_low: u64,
}

#[repr(C)]
#[derive(Debug)]
struct TabGroupPayload {
    tab_id: u8,
    maybe_group: SerializedToken,
    has_group: bool,
}

#[repr(C)]
#[derive(Debug)]
struct PinnedStatePayload {
    tab_id: u8,
    pinned_state: bool,
}

#[repr(C)]
#[derive(Debug)]
struct LastActiveTimePayload {
    tab_id: u8,
    last_active_time: i64,
}

#[repr(C)]
#[derive(Debug)]
struct VisibleOnAllWorkspacesPayload {
    window_id: u8,
    visible_on_all_workspaces: bool,
}
