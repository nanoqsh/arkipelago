pub(crate) use crate::{
    declare::Declare,
    layout::Layout,
    shader::{
        this::{Declaration, ShaderType},
        Shader,
    },
};

macro_rules! const_args {
    ($key:ident : $val:expr) => {
        (stringify!($key), &($val) as &dyn Declare)
    };
    ($key:ident) => {
        (stringify!($key), &($key) as &dyn Declare)
    };
}

pub(crate) use const_args;

macro_rules! shader_type {
    (vertex) => {
        ShaderType::Vertex
    };
    (fragment) => {
        ShaderType::Fragment
    };
}

pub(crate) use shader_type;

macro_rules! shader_layout_type {
    () => {
        ()
    };
    ($layout:ty) => {
        $layout
    };
}

pub(crate) use shader_layout_type;

macro_rules! shader {
    {
        type: $shader_type:ident,
        $( struct: $layout:ty, )?
        $( const: {
            $( $const_name:ident $( : $const_val:expr )? ),* $( , )?
        }, )?
        $( where: {
            $( $uni_name:ident : $uni_type:ty ),* $( , )?
        }, )?
        $( fn: ( $( $in_name:ident : $in_type:ty ),* ) -> ( $( $out_name:ident : $out_type:ty ),* ), )?
        impl: { $( $t:tt )* } $( , )?
    } => {
        Shader {
            typ: shader_type!($shader_type),
            layout: <shader_layout_type!($( $layout )?) as Layout>::layout,
            consts: std::collections::HashMap::from([
                $( $( const_args!($const_name $( : $const_val )?) ),* )?
            ]),
            uniforms: &[
                $( $( Declaration {
                    typ: <$uni_type as Declare>::declare_type,
                    name: stringify!($uni_name),
                } ),* )?
            ],
            in_vars: &[
                $( $( Declaration {
                    typ: <$in_type as Declare>::declare_type,
                    name: stringify!($in_name),
                } ),* )?
            ],
            out_vars: &[
                $( $( Declaration {
                    typ: <$out_type as Declare>::declare_type,
                    name: stringify!($out_name),
                } ),* )?
            ],
            src: stringify!( $( $t )* ),
        }
    };
}

pub(crate) use shader;

macro_rules! def_shader {
    { $( $t:tt )* } => {
        pub(crate) fn src() -> Shader<'static> {
            shader! { $( $t )* }
        }
    };
}

pub(crate) use def_shader;
