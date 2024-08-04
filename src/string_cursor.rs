use bevy::{
    prelude::*
};

#[derive(Debug)]
pub enum InputDirection {
    Left,
    Right,
}

/// State of the user input for read-line text prompts (like [`Input`]).
///
/// **Note**: This structure is not expected to be created, but it can be consumed when using a custom formatter.
#[derive(Debug, PartialEq, Eq, Default, Component)]
pub struct StringCursor {
    /// Current value of the input.
    pub value: String,
    /// Current index of the cursor (kept on ut8 char boundaries).
    pub index: usize,
}

impl StringCursor {
    #[allow(dead_code)]
    pub(crate) fn set_value(&mut self, value: &str) {
        self.value.replace_range(.., value);
        self.index = self.value.len();
    }

    pub(crate) fn insert(&mut self, ch: char) {
        self.value.insert(self.index, ch);
        self.index += ch.len_utf8();
    }

    pub(crate) fn backspace(&mut self) {
        if self.index >= self.value.len() {
            self.value.pop();
            self.index = self.value.len();
        } else {
            let start = floor_char_boundary(&self.value, self.index.saturating_sub(1));
            let _ = self.value.drain(start..self.index);
            self.index = start;
        }
    }

    pub(crate) fn next_index(&self) -> usize {
        ceil_char_boundary(&self.value, self.index + 1)
    }

    pub(crate) fn prev_index(&self) -> usize {
        floor_char_boundary(&self.value, self.index.saturating_sub(1))
    }

    pub(crate) fn delete(&mut self) {
        if !self.value.is_empty() && self.index < self.value.len() {
            self.value.remove(self.index);
        }
    }

    pub(crate) fn move_cursor(&mut self, position: InputDirection) {
        self.index = match position {
            // TODO: When round_char_boundary is stabilized, use std's impl.
            // InputDirection::Left => self.value.floor_char_boundary(self.index.saturating_sub(1)),
            InputDirection::Left => self.prev_index(),
            // InputDirection::Right => self.value.ceil_char_boundary(self.index + 1),
            InputDirection::Right => self.next_index(),
        }
    }
}

pub fn floor_char_boundary(s: &str, mut i: usize) -> usize {
    if i > s.len() {
        s.len()
    } else {
        while !s.is_char_boundary(i) {
            i = i.saturating_sub(1);
        }
        i
    }
}

pub fn ceil_char_boundary(s: &str, mut i: usize) -> usize {
    if i > s.len() {
        s.len()
    } else {
        while !s.is_char_boundary(i) {
            i = i.saturating_add(1);
        }
        i
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_floor_char() {
        let s = "❤️🧡💛💚💙💜";
        assert_eq!(s.len(), 26);
        assert!(!s.is_char_boundary(13));

        let closest = floor_char_boundary(s, 13);
        assert_eq!(closest, 10);
        assert_eq!(&s[..closest], "❤️🧡");
        assert_eq!(floor_char_boundary(s, 0), 0);
        assert_eq!(floor_char_boundary(s, 26), 26);
        assert_eq!(floor_char_boundary(s, 27), 26);
    }

    #[test]
    fn test_ceil_char() {
        let s = "❤️🧡💛💚💙💜";
        assert_eq!(s.len(), 26);
        assert!(!s.is_char_boundary(13));

        let closest = ceil_char_boundary(s, 13);
        assert_eq!(closest, 14);
        assert_eq!(&s[..closest], "❤️🧡💛");
        assert_eq!(ceil_char_boundary(s, 0), 0);
        assert_eq!(ceil_char_boundary(s, 26), 26);
        assert_eq!(ceil_char_boundary(s, 27), 26);
    }
}
