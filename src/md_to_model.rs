use crate::model::Model;
use pulldown_cmark::escape::{escape_html, StrWrite, WriteWrapper};
use pulldown_cmark::Event::*;
use pulldown_cmark::{
    html, Alignment, CodeBlockKind, CowStr, Event, LinkType, Options, Parser, Tag,
};
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
                Start(tag) => {
                    match tag {
                        Tag::Paragraph => {}
                        Tag::Heading(h) => {
                            dbg!(("heading", h, &self.action));
                            self.action = Action::Header(String::new());
                        }
                        Tag::BlockQuote => {}
                        Tag::CodeBlock(_) => {}
                        Tag::List(l) => {
                            dbg!(&l);
                        }
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
                    }

                    // self.start_tag(tag)?;
                }
                End(tag) => {
                    match tag {
                        Tag::Paragraph => {}
                        Tag::Heading(h) => {
                            dbg!(("heading end", h, &self.action));
                            match &self.action {
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
                            }
                        }
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
                    }
                    // self.end_tag(tag)?;
                }
                Text(text) => match &self.action {
                    Action::Header(s) => {
                        self.action = Action::Header(text.into_string().trim().to_string())
                    }
                    Action::Intro => self.model.introductie.push_str(&text),
                    Action::Voorbereiding => {
                        self.model.voorbereiding.push_str(&text);
                    }
                    Action::Materiaal => {
                        self.model
                            .materiaal
                            .last_mut()
                            .map(|mut s| s.push_str(&text));
                    }
                    Action::Antwoord => {
                        self.model
                            .antwoord
                            .last_mut()
                            .map(|mut s| s.push_str(&text));
                    }
                    Action::Verklaring => {
                        self.model.verklaring.push_str(&text);
                    }
                    _ => {}
                },
                Code(text) => {
                    // self.write("<code>")?;
                    // escape_html(&mut self.writer, &text)?;
                    // self.write("</code>")?;
                }
                Html(html) => {
                    // self.write(&html)?;
                }
                SoftBreak => {
                    // self.write_newline()?;
                }
                HardBreak => {
                    // self.write("<br />\n")?;
                }
                Rule => {
                    // if self.end_newline {
                    //     self.write("<hr />\n")?;
                    // } else {
                    //     self.write("\n<hr />\n")?;
                    // }
                }
                FootnoteReference(name) => {}
                TaskListMarker(true) => {}
                TaskListMarker(false) => {}
            }
        }

        Ok(())
    }

    pub fn model(&self) -> Model {
        self.model.clone()
    }
}
