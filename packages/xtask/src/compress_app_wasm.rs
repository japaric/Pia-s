use std::{borrow::Cow, fs};

use wasm_encoder::{EntityType, ImportSection, Module, TypeSection};
use wasmparser::{Import, Parser, Payload, TypeRef, ValType};

use crate::MyResult;

pub fn run() -> MyResult<()> {
    let root = crate::repo_root();

    let path = root.join("dist").join("app.wasm");
    let input = fs::read(&path)?;

    let original_sections_len = reencode_import_and_type_sections(&input, false)?.len();
    let compressed_sections = reencode_import_and_type_sections(&input, true)?;

    let mut output = compressed_sections;
    output.extend_from_slice(&input[original_sections_len..]);

    fs::write(path, output)?;

    Ok(())
}

fn reencode_import_and_type_sections(input: &[u8], compress: bool) -> MyResult<Vec<u8>> {
    let mut module = Module::new();

    for res in Parser::new(0).parse_all(input) {
        match res? {
            Payload::ImportSection(input) => {
                fn cvt(tyref: TypeRef) -> EntityType {
                    match tyref {
                        TypeRef::Func(f) => EntityType::Function(f),
                        _ => todo!(),
                    }
                }

                let mut output = ImportSection::new();
                for (index, res) in input.into_iter().enumerate() {
                    let Import { module, name, ty } = res?;
                    let name = if compress {
                        Cow::Owned(format!("${index}"))
                    } else {
                        Cow::Borrowed(name)
                    };

                    output.import(module, &name, cvt(ty));
                }

                module.section(&output);
            }

            Payload::TypeSection(input) => {
                fn cvt(val: &ValType) -> wasm_encoder::ValType {
                    use wasm_encoder::ValType::*;

                    match val {
                        ValType::I32 => I32,
                        ValType::I64 => I64,
                        ValType::F32 => F32,
                        ValType::F64 => F64,
                        ValType::V128 => V128,
                        ValType::Ref(_ref_type) => todo!(),
                    }
                }

                let mut output = TypeSection::new();
                for res in input.into_iter_err_on_gc_types() {
                    let ty = res?;
                    output
                        .ty()
                        .function(ty.params().iter().map(cvt), ty.results().iter().map(cvt));
                }
                module.section(&output);
            }

            _ => {}
        }
    }

    let output = module.finish();
    Ok(output)
}
