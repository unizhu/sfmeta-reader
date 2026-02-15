use crate::cli::PathStyle;

#[derive(Clone, Debug)]
pub struct FlattenOptions {
    pub include_attributes: bool,
    pub include_text: bool,
    pub include_cdata: bool,
    pub strip_namespace_prefix: bool,
    pub max_text_len: usize,
    pub path_style: PathStyle,
}
