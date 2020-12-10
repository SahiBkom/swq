use crate::model::Model;
use latex::{Align, Document, DocumentClass, Element, List, ListKind, Section};
use std::fs::File;
use std::io::{self, Error, Write};
use std::path::Path;
use std::process::Command;

pub fn model2latex(doc: &mut Document, model: Model) -> Result<(), ()> {
    let mut introductie = Section::new("Introductie");
    introductie.push(&*model.introductie);
    doc.push(introductie);

    let mut vraag = Section::new(&*model.vraag);

    let mut list = List::new(ListKind::Itemize);
    list.argument(&"noitemsep");
    for a in model.antwoord.iter() {
        list.push(a);
    }
    vraag.push(list);
    doc.push(vraag);

    let mut verklaring = Section::new("Verklaring");
    verklaring.push(&*model.verklaring);
    doc.push(verklaring);

    Ok(())
}

pub struct ModelToLatex {
    doc: Document,
}

impl ModelToLatex {
    pub fn new() -> Self {
        let mut doc = latex::Document::new(latex::DocumentClass::Article);
        let g = latex::PreambleElement::UsePackage {
            package: "geometry".to_string(),
            argument: Some("a6paper,margin=5mm,landscape".to_string()),
        };
        doc.preamble.push(g);
        doc.preamble.use_package(&"enumitem");
        doc.preamble.use_package(&"multicol");
        // remove section nummering
        doc.push(Element::UserDefined(
            "\\renewcommand{\\thesection}{\\hspace*{-1.0em}}".to_string(),
        ));
        Self { doc }
    }

    pub fn voorkant(&mut self, model: &Model) -> &mut Self {
        self.push(Element::ClearPage);
        let mut introductie = Section::new("Introductie");
        introductie.push(&*model.introductie);
        self.push(introductie);

        let mut vraag = Section::new(&*model.vraag);
        let mut list = List::new(ListKind::Itemize);
        list.argument(&"noitemsep");
        for a in model.antwoord.iter() {
            list.push(a);
        }
        vraag.push(list);
        self.push(vraag);

        let mut verklaring = Section::new("Verklaring");
        verklaring.push(&*model.verklaring);
        self.push(verklaring);

        self
    }

    pub fn achterkant(&mut self, model: &Model) -> &mut Self {
        self.push(Element::ClearPage);
        self.push(Element::UserDefined("\\begin{multicols}{2}".to_string()));
        let mut voorbereiding = Section::new("Voorbereiding");
        voorbereiding.push(&*model.introductie);
        self.push(voorbereiding);
        self.push(Element::UserDefined("\\vfill\\null".to_string()));
        self.push(Element::UserDefined("\\columnbreak".to_string()));
        let mut materiaal = Section::new("Materiaal");
        let mut list = List::new(ListKind::Itemize);
        list.argument(&"noitemsep");
        for a in model.materiaal.iter() {
            list.push(a);
        }
        materiaal.push(list);
        self.push(materiaal);
        self.push(Element::UserDefined("\\end{multicols}".to_string()));

        self
    }

    pub fn push<E>(&mut self, element: E) -> &mut Self
    where
        E: Into<Element>,
    {
        self.doc.push(element);
        self
    }

    pub fn rendered(&self) -> Result<String, ()> {
        latex::print(&self.doc).map_err(|_| ())
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), ()> {
        let mut file = File::create(filename).map_err(|_| ())?;
        write!(file, "{}", self.rendered().map_err(|_| ())?);
        Ok(())
    }
}
