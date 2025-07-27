use std::{collections::HashMap, fs, path::PathBuf, str};

use anyhow::{anyhow, bail, Result};
use handlebars::Handlebars;

use crate::Templates;

pub fn create_project(template: &str, name: &str) -> Result<()> {
    let template_prefix = format!("{template}/");
    let template_exists = Templates::iter().any(|f| f.starts_with(&template_prefix));
    if !template_exists {
        bail!(
            "Template '{}' not found. Available templates are: cli, web.",
            template
        );
    }

    // set up handlebars for templating
    let handlebars = Handlebars::new();
    let mut data = HashMap::new();
    data.insert("name", name.to_string());

    let dest_dir = PathBuf::from(name);
    if dest_dir.exists() {
        bail!("Directory '{}' already exists!", name);
    }
    fs::create_dir_all(&dest_dir)?;

    let template_prefix = format!("{template}/");

    // iterate through all template files
    for filename in Templates::iter() {
        let filepath = filename.as_ref();

        // only process files in the specified template directory
        if !filepath.starts_with(&template_prefix) {
            continue;
        }

        let rel_path = match filepath.strip_prefix(&template_prefix) {
            Some(path) => path,
            None => continue,
        };

        let output_path = dest_dir.join(rel_path);

        // if the template file is in a subdirectory, create the output directory
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let file = Templates::get(filepath)
            .ok_or_else(|| anyhow!("Template file '{}' not found in '{}'", filepath, template))?;
        let content = str::from_utf8(file.data.as_ref())?;

        if filepath.ends_with(".hbs") {
            let rendered = handlebars.render_template(content, &data)?;

            // remove .hbs extension from template files before writing
            let output_path = output_path.with_extension("");
            fs::write(output_path, rendered)?;
        } else {
            let bytes = file.data.as_ref();
            fs::write(output_path, bytes)?;
        }
    }

    println!("Project '{name}' created from '{template}' template.");

    Ok(())
}
