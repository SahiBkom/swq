use crate::model::Model;
use latex::{Document, Element, List, ListKind, Paragraph, Section, SubSection};
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
        doc.push(Element::UserDefined(
            "\\renewcommand{\\thesubsection}{\\hspace*{-1.0em}}".to_string(),
        ));
        Self { doc }
    }

    pub fn voorkant(&mut self, model: &Model) -> &mut Self {
        self.push(Element::ClearPage);

        let mut s = Section::new(&model.path);

        let mut introductie = Paragraph::new();
        introductie.push(&*model.introductie);
        s.push(introductie);

        let mut vraag = SubSection::new(&*model.vraag);
        let mut list = List::new(ListKind::Itemize);
        list.argument(&"noitemsep");
        for a in model.antwoord.iter() {
            list.push(a);
        }
        vraag.push(list);
        s.push(vraag);

        let mut verklaring = SubSection::new("Verklaring");
        verklaring.push(&*model.verklaring);
        s.push(verklaring);

        self.push(s);

        self
    }

    pub fn achterkant(&mut self, model: &Model) -> &mut Self {
        self.push(Element::ClearPage);
        self.push(Section::new(&model.path));
        self.push(Element::UserDefined("\\begin{multicols}{2}".to_string()));
        let mut voorbereiding = SubSection::new("Voorbereiding");
        voorbereiding.push(&*model.voorbereiding);
        self.push(voorbereiding);
        self.push(Element::UserDefined("\\vfill\\null".to_string()));
        self.push(Element::UserDefined("\\columnbreak".to_string()));
        let mut materiaal = SubSection::new("Materiaal");
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
        write!(file, "{}", self.rendered().map_err(|_| ())?).map_err(|_| ())?;
        Ok(())
    }
}
