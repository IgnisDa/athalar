mod atoms;
mod constants;

use crate::{atoms::AthalarPythonKind, constants::DEFAULT_CLASS_NAME};
use athalar_core::{AthalarAdapter, AthalarAtom, AthalarBinding};
use serde::{Deserialize, Serialize};
use tera::{Context as TeraContext, Tera};

const PYTHON_TEMPLATE: &str = include_str!("typescript.tera");

#[derive(Debug, Serialize, Deserialize)]
struct PropertyContext {
    name: String,
    kind: String,
    comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {
    class_name: String,
    properties: Vec<PropertyContext>,
}

pub fn get_typescript_contents(
    binding: &AthalarBinding,
    atoms: &Vec<AthalarAtom>,
) -> anyhow::Result<String> {
    let mut context = match &binding.profile {
        AthalarAdapter::ClassValidator(_) => unimplemented!(),
        AthalarAdapter::Pydantic(x) => Context {
            class_name: x
                .class_name
                .clone()
                .unwrap_or_else(|| DEFAULT_CLASS_NAME.to_string()),
            properties: vec![],
        },
    };
    for atom in atoms {
        context.properties.push(PropertyContext {
            name: atom.name.clone(),
            kind: AthalarPythonKind::from(atom.kind).to_string(),
            comment: atom.description.clone(),
            // TODO: Handle validators
        })
    }
    let context = TeraContext::from_serialize(context)?;
    let rendered = Tera::one_off(PYTHON_TEMPLATE, &context, false)?;
    Ok(rendered)
}
