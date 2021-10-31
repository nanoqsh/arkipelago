mod def;
mod export;
mod this;

pub(crate) mod src {
    use crate::shader::export::export;

    export! {
        col_fs,
        col_vs,
        def_fs,
        def_vs,
        post_fs,
        post_vs,
        skin_vs,
    }
}

pub(crate) use self::this::Shader;

const VERSION: &str = "#version 410";
pub(crate) const BONES_MAX_LEN: usize = 32;
