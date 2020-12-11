mod md_to_model;
mod model;
mod model_to_lex;

use crate::md_to_model::Md2Model;
use crate::model::Model;
use itertools::Itertools;
use pulldown_cmark::{Options, Parser};
use scan_dir::ScanDir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Vragen
    ///
    #[structopt(default_value = "vragen", parse(from_os_str))]
    vragen: PathBuf,

    /// tmp
    ///
    #[structopt(default_value = "target/swq", parse(from_os_str))]
    tmp: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut models: Vec<_> = ScanDir::files()
        .walk(&opt.vragen, |iter| {
            iter.filter(|&(_, ref name)| name.ends_with(".md"))
                .map(|(ref entry, _)| {
                    let markdown_input = std::fs::read_to_string(entry.path()).unwrap();
                    let parser = Parser::new_ext(&markdown_input, Options::empty());

                    let mut model: Model = Default::default();
                    model.path = String::from(
                        entry
                            .path()
                            .strip_prefix(opt.vragen.clone())
                            .expect("Start given path")
                            .with_extension("")
                            .to_string_lossy(),
                    );
                    let mut md2model = Md2Model::new(parser, &mut model);
                    md2model.run().unwrap();
                    println!("{:#?}", model);
                    model
                })
                .collect()
        })
        .map_err(|e| format!("{:?}", e))?;

    match &models.len() % 4 {
        0 => (),
        m => models.resize_with(models.len() + 4 - m % 4, Default::default),
    };

    let mut latex = model_to_lex::ModelToLatex::new();
    for (a, b, c, d) in models.into_iter().tuples() {
        latex.voorkant(&a);
        latex.voorkant(&b);
        latex.voorkant(&c);
        latex.voorkant(&d);
        latex.achterkant(&b);
        latex.achterkant(&a);
        latex.achterkant(&d);
        latex.achterkant(&c);
    }

    let mut path = opt.tmp.clone();
    path.push("report.tex");
    std::fs::create_dir_all(opt.tmp.clone()).expect("dir");
    latex.write_to_file(path).map_err(|e| format!("{:?}", e))?;

    // Then call latexmk on it
    let _exit_status = Command::new("latexmk")
        .current_dir(opt.tmp.clone())
        .arg("report.tex")
        .arg("-pdf")
        .arg("-interaction=nonstopmode")
        .status()
        .map_err(|e| format!("{:?}", e))?;

    let _exit_status = Command::new("latexmk")
        .current_dir(opt.tmp.clone())
        .arg("report.tex")
        .arg("-pdf")
        .arg("-interaction=nonstopmode")
        .status()
        .map_err(|e| format!("{:?}", e))?;

    let my_str = include_str!("a6_to_a4.tex");
    let mut a6_to_a4_tex = opt.tmp.clone();
    a6_to_a4_tex.push("a6_to_a4.tex");
    let mut file = File::create(a6_to_a4_tex).map_err(|e| format!("{:?}", e))?;
    file.write_all(my_str.as_ref())
        .map_err(|e| format!("{:?}", e))?;
    file.sync_all().map_err(|e| format!("{:?}", e))?;

    let _exit_status = Command::new("latexmk")
        .current_dir(opt.tmp)
        .arg("a6_to_a4.tex")
        .arg("-pdf")
        .arg("-interaction=nonstopmode")
        .status()
        .map_err(|e| format!("{:?}", e))?;

    Ok(())
}
