use thiserror::Error;
use std::io;

#[repr(u8)]
#[derive(Error, Debug)]
pub enum FtuiError {
    #[error("TextFlags::NONE cannot be combined with other TextFlags.")]
    TextFlagNoneWithOther,

    #[error("TextFlags cannot contain multiple color.")]
    TextFlagMultipleColor,

    #[error("A Header label cannot be empty.")]
    HeaderLabelEmpty,

    #[error("An Option label cannot be empty.")]
    OptionLabelEmpty,

    #[error("Renderer requires the container to have a header.")]
    RendererContainerNoHeader,

    #[error("The container's looper method requires a Selector.")]
    ContainerNoSelector,

    #[error("Std Input Output Error: {0}")]
    StdInputOutputError(#[from] io::Error),
}
