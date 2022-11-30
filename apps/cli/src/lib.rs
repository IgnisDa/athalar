pub mod app;

use anyhow::anyhow;
use athalar_core::{from_path, AthalarAdapter, FinalFile};
use athalar_python::add_python_final_files;
use std::{fs::File, io::Write, path::PathBuf};

pub fn run(path: PathBuf) -> anyhow::Result<()> {
    // TODO: Correct error handling
    let athalar = from_path(path.to_str().unwrap().to_owned()).expect("msg");
    let information = athalar.get_information().unwrap();
    let mut final_files = vec![];
    for (generator, atoms) in information.generators.iter() {
        for binding in generator.data.bindings.iter() {
            let path = binding.output(&information.config.project_source());
            let contents = match &binding.profile {
                AthalarAdapter::Pydantic(_a) => add_python_final_files(binding, atoms),
                // TODO: Handle it
                AthalarAdapter::ClassValidator(_) => continue,
            }?;
            final_files.push(FinalFile { contents, path });
        }
    }
    for final_file in final_files {
        File::create(final_file.path)
            .map_err(|_| anyhow!("Unable to create file"))?
            .write_all(final_file.contents.as_bytes())
            .map_err(|_| anyhow!("Unable write file"))?;
    }
    Ok(())
}
