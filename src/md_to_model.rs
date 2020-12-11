use crate::model::Model;
use pulldown_cmark::Event::*;
use pulldown_cmark::{Event, Tag};
use std::io;

#[derive(Debug)]
enum Action {
    None,
    Header(String),
    Intro,
    Voorbereiding,
    Materiaal,
    Antwoord,
    Verklaring,
}

pub struct Md2Model<'a, I> {
    /// Iterator supplying events.
    iter: I,
    model: &'a mut Model,
    action: Action,
}

impl<'a, I> Md2Model<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(iter: I, model: &'a mut Model) -> Self {
        Self {
            iter,
            model,
            action: Action::None,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => match tag {
                    Tag::Paragraph => {}
                    Tag::Heading(_h) => {
                        self.action = Action::Header(String::new());
                    }
                    Tag::BlockQuote => {}
                    Tag::CodeBlock(_) => {}
                    Tag::List(_l) => {}
                    Tag::Item => match self.action {
                        Action::Materiaal => {
                            self.model.materiaal.push(String::new());
                        }
                        Action::Antwoord => {
                            self.model.antwoord.push(String::new());
                        }
                        _ => {}
                    },
                    Tag::FootnoteDefinition(_) => {}
                    Tag::Table(_) => {}
                    Tag::TableHead => {}
                    Tag::TableRow => {}
                    Tag::TableCell => {}
                    Tag::Emphasis => {}
                    Tag::Strong => {}
                    Tag::Strikethrough => {}
                    Tag::Link(_, _, _) => {}
                    Tag::Image(_, _, _) => {}
                },
                End(tag) => match tag {
                    Tag::Paragraph => {}
                    Tag::Heading(_h) => match &self.action {
                        Action::Header(s) => match s.as_str() {
                            "Intro" => self.action = Action::Intro,
                            "Voorbereiding" => {
                                self.action = Action::Voorbereiding;
                            }
                            "Materiaal" => self.action = Action::Materiaal,
                            "Verklaring" => self.action = Action::Verklaring,
                            other => {
                                if other.ends_with("?") {
                                    self.model.vraag.push_str(&other);
                                    self.action = Action::Antwoord;
                                }
                            }
                        },
                        _ => {}
                    },
                    Tag::BlockQuote => {}
                    Tag::CodeBlock(_) => {}
                    Tag::List(_) => {}
                    Tag::Item => {}
                    Tag::FootnoteDefinition(_) => {}
                    Tag::Table(_) => {}
                    Tag::TableHead => {}
                    Tag::TableRow => {}
                    Tag::TableCell => {}
                    Tag::Emphasis => {}
                    Tag::Strong => {}
                    Tag::Strikethrough => {}
                    Tag::Link(_, _, _) => {}
                    Tag::Image(_, _, _) => {}
                },
                Text(text) => match &self.action {
                    Action::Header(_s) => {
                        self.action = Action::Header(text.into_string().trim().to_string())
                    }
                    Action::Intro => self.model.introductie.push_str(&text),
                    Action::Voorbereiding => {
                        self.model.voorbereiding.push_str(&text);
                    }
                    Action::Materiaal => {
                        self.model.materiaal.last_mut().map(|s| s.push_str(&text));
                    }
                    Action::Antwoord => {
                        self.model.antwoord.last_mut().map(|s| s.push_str(&text));
                    }
                    Action::Verklaring => {
                        self.model.verklaring.push_str(&text);
                    }
                    _ => {}
                },
                Code(_text) => {}
                Html(_html) => {}
                SoftBreak => {
                    // Voeg een spatie toe om te voorkomen dat dingen aan elkaar belanden.
                    if let Some(s) = self.get_text_mut() {
                        s.push_str(" ");
                    }
                }
                HardBreak => {
                    // Komt niet voor?
                }
                Rule => {}
                FootnoteReference(_name) => {}
                TaskListMarker(true) => {}
                TaskListMarker(false) => {}
            }
        }

        Ok(())
    }

    fn get_text_mut(&mut self) -> Option<&mut String> {
        match &self.action {
            Action::None => None,
            Action::Header(_) => None,
            Action::Intro => Some(&mut self.model.introductie),
            Action::Voorbereiding => Some(&mut self.model.voorbereiding),
            Action::Materiaal => self.model.materiaal.last_mut(),
            Action::Antwoord => self.model.antwoord.last_mut(),
            Action::Verklaring => Some(&mut self.model.voorbereiding),
        }
    }
}
