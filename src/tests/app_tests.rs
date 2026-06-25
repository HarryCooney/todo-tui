use crate::app::*;

#[test]
fn test_remove_task_no_selection() {
    //Default loads 3 tasks in automatically
    let mut app = App::default();
    app.list.remove_task(None);
    assert_eq!(3, app.list.items.len());
}

#[test]
fn test_remove_first_task() {
    let task1 = TodoItem::new(
        "Task 1",
        "This will contain info about the first task, whatever that might be. Basically this text is just here to fill up space and ensure that everything is formatting correctly and that the text is being contained within the border. Also, when I switch to another task I want to make sure that the corresponding text for that task also displays properly so I will write something for the other task",
         Status::Todo);
    let task2 = TodoItem::new(
        "Task 2", "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.",
                 Status::Complete);
    let task3 = TodoItem::new(
        "Task 3", "Text here",
         Status::Todo);

    let mut app = App::default();
    app.list.remove_task(Some(0));
    assert_eq!(2, app.list.items.len());
    assert_eq!(task2, app.list.items[0]);
    assert_eq!(task3, app.list.items[1]);
}
#[test]
fn test_remove_middle_task() {
    let task1 = TodoItem::new(
        "Task 1",
        "This will contain info about the first task, whatever that might be. Basically this text is just here to fill up space and ensure that everything is formatting correctly and that the text is being contained within the border. Also, when I switch to another task I want to make sure that the corresponding text for that task also displays properly so I will write something for the other task",
         Status::Todo);
    let task2 = TodoItem::new(
        "Task 2", "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.",
                 Status::Complete);
    let task3 = TodoItem::new(
        "Task 3", "Text here",

         Status::Todo);

    let mut app = App::default();
    app.list.remove_task(Some(1));
    assert_eq!(2, app.list.items.len());
    assert_eq!(task1, app.list.items[0]);
    assert_eq!(task3, app.list.items[1]);
}

#[test]
fn test_remove_last_task() {
    let task1 = TodoItem::new(
        "Task 1",
        "This will contain info about the first task, whatever that might be. Basically this text is just here to fill up space and ensure that everything is formatting correctly and that the text is being contained within the border. Also, when I switch to another task I want to make sure that the corresponding text for that task also displays properly so I will write something for the other task",
         Status::Todo);
    let task2 = TodoItem::new(
        "Task 2", "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.",
                 Status::Complete);
    let task3 = TodoItem::new(
        "Task 3", "Text here",
         Status::Todo);

    let mut app = App::default();
    app.list.remove_task(Some(2));
    assert_eq!(2, app.list.items.len());
    assert_eq!(task1, app.list.items[0]);
    assert_eq!(task2, app.list.items[1]);
}

#[test]
fn test_change_status() {
    let mut item = TodoItem::new("", "", Status::Todo);
    assert_eq!(Status::Todo, item.status);
    item.change_status();
    assert_eq!(Status::Complete, item.status);
}
