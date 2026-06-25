use crate::editor::*;

#[test]
fn test_move_cursor_right_underflow_edge_case() {
    let mut editor = Editor::default();
    editor.move_cursor_left();
    editor.currently_editing = CurrentlyEditing::Title;
    assert_eq!(0, editor.char_index);
}

#[test]
fn test_move_cursor_right_overflow_edge_case() {
    let mut editor = Editor::default();
    editor.title_input = String::from("aaaaa");
    editor.currently_editing = CurrentlyEditing::Title;
    editor.char_index = 5;
    editor.move_cursor_right();
    assert_eq!(5, editor.char_index);
}

#[test]
fn test_move_cursor_left_standard_case() {
    let mut editor = Editor::default();
    editor.title_input = String::from("aaaaa");
    editor.currently_editing = CurrentlyEditing::Title;
    editor.char_index = 3;
    editor.move_cursor_left();
    assert_eq!(2, editor.char_index);
}

#[test]
fn test_move_cursor_right_standard_case() {
    let mut editor = Editor::default();
    editor.title_input = String::from("aaaaa");
    editor.currently_editing = CurrentlyEditing::Title;
    editor.char_index = 3;
    editor.move_cursor_right();
    assert_eq!(4, editor.char_index);
}

#[test]
fn test_cursor_to_end() {
    let mut editor = Editor::default();
    editor.title_input = String::from("aaaaa");
    editor.currently_editing = CurrentlyEditing::Title;
    editor.cursor_to_end();
    assert_eq!(5, editor.char_index);
}

#[test]
fn test_switch_editing_title_to_info() {
    let mut editor = Editor::default();
    editor.title_input = String::from("aaaaa");
    editor.info_input = String::from("bbbbbbbb"); //8 chars
    editor.currently_editing = CurrentlyEditing::Title;
    editor.switch_editing();
    assert_eq!(8, editor.char_index);
    assert_eq!(CurrentlyEditing::Info, editor.currently_editing);
}

#[test]
fn test_switch_editing_info_to_title() {
    let mut editor = Editor::default();
    editor.currently_editing = CurrentlyEditing::Info;
    editor.title_input = String::from("aaaaa");
    editor.info_input = String::from("bbbbbbbb"); //8 chars
    editor.switch_editing();
    assert_eq!(5, editor.char_index);
    assert_eq!(CurrentlyEditing::Title, editor.currently_editing);
}

#[test]
fn test_reset_cursor() {
    let mut editor = Editor::default();
    editor.title_input = String::from("aaaaa");
    editor.char_index = 5;
    editor.reset_cursor();
    assert_eq!(0, editor.char_index);
}
