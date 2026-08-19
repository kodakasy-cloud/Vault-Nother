#[derive(Debug, Clone)]
pub enum Message {
    SidebarNavPressed(&'static str),
    ToggleSidebar,
}