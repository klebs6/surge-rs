ix!();

#[derive(Debug,PartialEq,Eq)]
pub enum Padding {
    NoPadding,
    Units(usize),
}

pub const NO_PADDING: Padding = Padding::NoPadding;
