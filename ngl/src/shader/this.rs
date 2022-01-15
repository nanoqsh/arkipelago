use crate::{declare::Declare, layout::Field, shader::VERSION};
use std::{collections::HashMap, fmt::Write};

#[derive(Copy, Clone)]
pub(crate) enum ShaderType {
    Vertex,
    Fragment,
}

pub(crate) struct Declaration<'a> {
    pub typ: fn(&mut String),
    pub name: &'a str,
}

impl Declaration<'_> {
    fn write_glsl(&self, kind: &str, out: &mut String) {
        write!(out, "{kind} ").unwrap();
        (self.typ)(out);
        write!(out, " {}", self.name).unwrap();
    }
}

pub(crate) struct Shader<'a> {
    pub typ: ShaderType,
    pub layout: fn(&mut Vec<Field>),
    pub consts: HashMap<&'a str, &'a dyn Declare>,
    pub uniforms: &'a [Declaration<'a>],
    pub in_vars: &'a [Declaration<'a>],
    pub out_vars: &'a [Declaration<'a>],
    pub src: &'a str,
}

impl Shader<'_> {
    pub fn write_glsl(&self, out: &mut String) {
        fn not_alpha_numeric(c: char) -> bool {
            !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
        }

        writeln!(out, "{VERSION}").unwrap();

        let mut fields = Vec::new();
        (self.layout)(&mut fields);
        for (idx, Field { name, declare, .. }) in fields.into_iter().enumerate() {
            write!(out, "layout (location = {idx}) in ").unwrap();
            declare(out);
            writeln!(out, " {name};").unwrap();
        }

        for uni in self.uniforms {
            uni.write_glsl("uniform", out);
            writeln!(out, ";").unwrap();
        }

        for var in self.in_vars {
            var.write_glsl("in", out);
            writeln!(out, ";").unwrap();
        }

        for var in self.out_vars {
            var.write_glsl("out", out);
            writeln!(out, ";").unwrap();
        }

        let mut chunks = self.src.split('$');
        if let Some(first) = chunks.next() {
            write!(out, "{first}").unwrap();
        }

        for chunk in chunks {
            let (key, rest) = match chunk.split_once(not_alpha_numeric) {
                None => (chunk, ""),
                Some((key, rest)) => (key.trim(), rest),
            };

            match self.consts.get(key) {
                None => panic!("key {key} not found"),
                Some(glsl) => glsl.literal(out),
            }
            write!(out, "{rest}").unwrap()
        }
    }

    pub fn gl(&self) -> u32 {
        match self.typ {
            ShaderType::Vertex => glow::VERTEX_SHADER,
            ShaderType::Fragment => glow::FRAGMENT_SHADER,
        }
    }
}
