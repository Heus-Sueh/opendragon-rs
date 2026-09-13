use gpui_kit::component::separator::Separator as KitSeparator;

use crate::theme::Theme;

pub struct Separator;

impl Separator {
    pub fn horizontal(_theme: &Theme) -> KitSeparator {
        KitSeparator::horizontal()
    }

    pub fn vertical(_theme: &Theme) -> KitSeparator {
        KitSeparator::vertical()
    }
}
