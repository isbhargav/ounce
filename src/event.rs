pub enum ActionEvent {
    BackSpace,
    AddCharacter,
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    MoveEndOfWordForward,
    MoveEndOfWordBack,
    MoveStartOfWordForward,
    MoveStartOfWordBack,
    MoveEndOfLine,
    MoveStartOfLine,
    DeletePreviousWord,
}
impl ActionEvent {
    /* Based on provided action do the following task */
    pub fn handle_action(action: Self) {
        match action {
            Self::BackSpace => { /*  Deleat last character */ }
            Self::AddCharacter => {}
            Self::MoveRight => {}
            Self::MoveLeft => {}
            Self::MoveUp => {}
            Self::MoveDown => {}
            Self::MoveEndOfWordForward => {}
            Self::MoveEndOfWordBack => {}
            Self::MoveStartOfWordForward => {}
            Self::MoveStartOfWordBack => {}
            Self::MoveEndOfLine => {}
            Self::MoveStartOfLine => {}
            Self::DeletePreviousWord => {}
            _ => {}
        }
    }
}
