//! A single reversible change to a buffer.

/// One edit to the text: at character index [`at`](Edit::at), the text
/// [`removed`](Edit::removed) was replaced by [`inserted`](Edit::inserted).
///
/// This is the atom that several subsystems are built on:
/// - **history** stores edits and replays their inverses to undo,
/// - **syntax** turns them into tree-sitter `InputEdit`s for incremental parsing,
/// - **lsp** turns them into `textDocument/didChange` ranges.
///
/// Keeping both the old and new text means an edit carries everything needed to
/// undo it, with no separate "before" snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    /// Character index at which the change begins.
    pub at: usize,
    /// The text that used to be there (empty for a pure insertion).
    pub removed: String,
    /// The text that replaced it (empty for a pure deletion).
    pub inserted: String,
}

impl Edit {
    /// An insertion of `text` at `at`.
    pub fn insertion(at: usize, text: impl Into<String>) -> Edit {
        Edit {
            at,
            removed: String::new(),
            inserted: text.into(),
        }
    }

    /// A deletion of `text` at `at`.
    pub fn deletion(at: usize, text: impl Into<String>) -> Edit {
        Edit {
            at,
            removed: text.into(),
            inserted: String::new(),
        }
    }

    /// Number of characters this edit removes.
    pub fn removed_chars(&self) -> usize {
        self.removed.chars().count()
    }

    /// Number of characters this edit inserts.
    pub fn inserted_chars(&self) -> usize {
        self.inserted.chars().count()
    }

    /// The edit that undoes this one: swap removed and inserted text.
    pub fn inverse(&self) -> Edit {
        Edit {
            at: self.at,
            removed: self.inserted.clone(),
            inserted: self.removed.clone(),
        }
    }

    /// Character index just past the edit once it has been applied (the natural
    /// place to leave the cursor).
    pub fn end(&self) -> usize {
        self.at + self.inserted_chars()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_classify_correctly() {
        let ins = Edit::insertion(3, "abc");
        assert_eq!(ins.removed, "");
        assert_eq!(ins.inserted, "abc");

        let del = Edit::deletion(3, "xy");
        assert_eq!(del.removed, "xy");
        assert_eq!(del.inserted, "");
    }

    #[test]
    fn char_counts_use_chars_not_bytes() {
        // "é😀" is 2 chars but 6 bytes.
        let edit = Edit::insertion(0, "é😀");
        assert_eq!(edit.inserted_chars(), 2);
        assert_eq!(edit.removed_chars(), 0);
    }

    #[test]
    fn inverse_swaps_and_is_self_cancelling() {
        let edit = Edit {
            at: 5,
            removed: "old".to_string(),
            inserted: "new!".to_string(),
        };
        let inv = edit.inverse();
        assert_eq!(inv.at, 5);
        assert_eq!(inv.removed, "new!");
        assert_eq!(inv.inserted, "old");
        // Inverting twice returns the original.
        assert_eq!(inv.inverse(), edit);
    }

    #[test]
    fn end_is_past_inserted_text() {
        assert_eq!(Edit::insertion(10, "hello").end(), 15);
        assert_eq!(Edit::deletion(10, "hello").end(), 10); // nothing inserted
    }
}
