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
    ContainerLooperNoSelector,

    #[error("Container doesnot have a Selector.")]
    ContainerNoSelector,

    #[error("Std Input Output Error: {0}")]
    StdInputOutputError(#[from] io::Error),

    #[error("Trigger function does not have an argument available for casting.")]
    TriggerCastArgNoArgument,

    #[error("Trigger function argument type mismatch unable to cast to the expected type.")]
    TriggerCastArgWrongType,

    #[error("Callback function does not have an argument available for casting.")]
    CallbackCastArgNoArgument,

    #[error("Callback function argument type mismatch unable to cast to the expected type.")]
    CallbackCastArgWrongType,
}

pub type FtuiResult<T> = Result<T, FtuiError>;
