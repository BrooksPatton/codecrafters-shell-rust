enum ProcessInputState {
    Escaping,
    InsideSingleQuotes,
    InsideDoubleQuotes,
    InsideDoubleQuotesEscaping,
    Normal,
}

impl ProcessInputState {
    pub fn inside_quotes(&self) -> bool {
        matches!(self, Self::InsideSingleQuotes) || matches!(self, Self::InsideDoubleQuotes)
    }

    pub fn to_normal(&mut self) {
        *self = Self::Normal;
    }

    pub fn to_double_quoting(&mut self) {
        *self = Self::InsideDoubleQuotes;
    }

    pub fn to_double_quote_escaping(&mut self) {
        *self = Self::InsideDoubleQuotesEscaping;
    }

    pub fn to_single_quoting(&mut self) {
        *self = Self::InsideSingleQuotes;
    }

    pub fn to_escaping(&mut self) {
        *self = Self::Escaping;
    }
}

pub fn parse_input(input: String) -> Vec<String> {
    let mut result = vec![];
    let mut current_argument = String::new();
    let mut state = ProcessInputState::Normal;

    for argument_char in input.trim().chars() {
        if matches!(state, ProcessInputState::Escaping) {
            current_argument.push(argument_char);
            state.to_normal();
            continue;
        }

        match argument_char {
            '\'' => match state {
                ProcessInputState::InsideSingleQuotes => state.to_normal(),
                ProcessInputState::InsideDoubleQuotes => current_argument.push(argument_char),
                ProcessInputState::InsideDoubleQuotesEscaping => {
                    current_argument.push('\\');
                    current_argument.push(argument_char);
                    state.to_double_quoting();
                }
                ProcessInputState::Normal => state.to_single_quoting(),
                _ => (),
            },
            '"' => match state {
                ProcessInputState::InsideSingleQuotes => current_argument.push(argument_char),
                ProcessInputState::InsideDoubleQuotes => state.to_normal(),
                ProcessInputState::InsideDoubleQuotesEscaping => {
                    current_argument.push(argument_char);
                    state.to_double_quoting();
                }
                ProcessInputState::Normal => state.to_double_quoting(),
                _ => (),
            },
            '~' => {
                if matches!(state, ProcessInputState::InsideSingleQuotes) {
                    current_argument.push(argument_char);
                } else {
                    let home_directory = std::env::home_dir().unwrap_or_default();
                    current_argument.push_str(home_directory.to_str().unwrap_or_default());
                }
            }
            ' ' => {
                if state.inside_quotes() {
                    current_argument.push(argument_char);
                } else if !current_argument.is_empty() {
                    result.push(current_argument.clone());
                    current_argument.clear();
                }
            }
            '\\' => match state {
                ProcessInputState::InsideSingleQuotes => current_argument.push(argument_char),
                ProcessInputState::InsideDoubleQuotes => state.to_double_quote_escaping(),
                ProcessInputState::InsideDoubleQuotesEscaping => {
                    current_argument.push(argument_char);
                    state.to_double_quoting();
                }
                ProcessInputState::Normal => state.to_escaping(),
                _ => (),
            },
            _ => {
                if matches!(state, ProcessInputState::InsideDoubleQuotesEscaping) {
                    state.to_double_quoting();
                    current_argument.push('\\');
                }
                current_argument.push(argument_char);
            }
        }
    }

    if !current_argument.is_empty() {
        result.push(current_argument);
    }

    result
}
