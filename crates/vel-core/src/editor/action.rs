use crate::editor::cursor::Cursor;

#[derive(Debug, Clone, PartialEq)]
pub struct ActionData {
    // destination of the cursor after the action
    pub cursor: Cursor,
}


#[derive(Debug, Clone, PartialEq)]
pub struct InputAction {
    pub data: ActionData,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PasteAction {
    pub data: ActionData,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteAction {
    pub data: ActionData,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionAction {
    pub data: ActionData,
    pub start: Cursor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveAction {
    pub data: ActionData, // the cursor corresponds to the destination
    pub begin: Cursor,
    pub end: Cursor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveCursorAction {
    pub data: ActionData,
    pub start: Cursor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiCursorAction {
    pub data: ActionData,
    // starting positions for the action
    pub cursors: Vec<Cursor>,
    // it can only be one action for all cursors
    pub action: Box<Action>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Input(InputAction),
    Delete(DeleteAction),
    Paste(PasteAction),
    Selection(SelectionAction),
    Move(MoveAction),
    MoveCursor(MoveCursorAction),
    MultiCursor(MultiCursorAction),
}

pub trait ActionHandler {
    fn handle(&self, action: &Action) -> Result<(), bool>;
}