use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    EditingSearch,
    EditingTitle,
    EditingLanguage,
    EditingCode,
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl InputMode {
    const modes: [InputMode; 5] = [Self::Normal, Self::EditingSearch, Self::EditingTitle, Self::EditingLanguage, Self::EditingCode];

    pub fn next_mode(&self) -> InputMode {
        match self {
            Self::Normal => Self::EditingSearch,
            Self::EditingSearch => Self::EditingTitle,
            Self::EditingTitle => Self::EditingLanguage,
            Self::EditingLanguage => Self::EditingCode,
            Self::EditingCode => Self::Normal
        }
    }
    pub fn previous_mode(&self) -> InputMode {
        match self {
            Self::Normal => Self::EditingCode,
            Self::EditingSearch => Self::Normal,
            Self::EditingTitle => Self::EditingSearch,
            Self::EditingLanguage => Self::EditingTitle,
            Self::EditingCode => Self::EditingLanguage
        }
    }
}

#[cfg(test)]
mod test {
    use super::InputMode;

    #[test]
    fn normal_to_edit_search(){
        let input = InputMode::Normal;
        assert_eq!(input.next_mode(), InputMode::EditingSearch);
    }

    #[test]
    fn edit_search_to_edit_title(){
        let input = InputMode::EditingSearch;
        assert_eq!(input.next_mode(), InputMode::EditingTitle);
    }

    #[test]
    fn edit_code_to_normal(){
        let input = InputMode::EditingCode;
        assert_eq!(input.next_mode(), InputMode::Normal);
    }

    #[test]
    fn edit_search_to_normal(){
        let input = InputMode::EditingSearch;
        assert_eq!(input.previous_mode(), InputMode::Normal);
    }

    #[test]
    fn edit_title_to_edit_search(){
        let input = InputMode::EditingTitle;
        assert_eq!(input.previous_mode(), InputMode::EditingSearch);
    }

    #[test]
    fn normal_to_edit_code(){
        let input = InputMode::Normal;
        assert_eq!(input.previous_mode(), InputMode::EditingCode);
    }


}
