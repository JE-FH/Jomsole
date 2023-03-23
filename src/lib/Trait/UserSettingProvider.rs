#[derive(Clone)]
pub enum WindowsCwdHandling {
    FullUNC,
    LaunchWithoutUNC,
    NeverUNC
}

pub trait UserSettingProvider {
    fn windows_cwd_handling(&self) -> WindowsCwdHandling;
}
