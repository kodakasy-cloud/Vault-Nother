#[derive(Debug, Clone)]
pub enum Message {

    // HOME
    CreateNewNote,
    SearchChanged(String),


    SidebarNavPressed(&'static str),
    ToggleSidebar,
}