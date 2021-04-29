use crate::errors::Errcode;

mod email_inspector;
use email_inspector::{EmailInspectorResult, EmailInspector};

pub enum InspectorResult{
    Email(EmailInspectorResult),
}

pub struct AnalyserResult{
    emails: EmailInspectorResult,
}

impl AnalyserResult{
    pub fn void() -> AnalyserResult{
        AnalyserResult{
            emails: EmailInspectorResult::void(),
        }
    }
}




pub struct Analyser{
    html_inspectors: Vec<Box<dyn HtmlInspector>>,
    result: AnalyserResult,
}

impl Analyser{
    pub fn new() -> Analyser{
        Analyser{
            html_inspectors: vec![Box::new(EmailInspector::new())],
            result: AnalyserResult::void(),
        }
    }

    pub fn inspect_html(&mut self, html: &String) -> Result<(), Errcode>{
        for inspector in self.html_inspectors.iter_mut(){
            inspector.analyse_html(html)?;
        }
        Ok(())
    }
}

pub trait HtmlInspector{
    fn new() -> Self where Self: Sized;
    fn analyse_html(&mut self, html: &String) -> Result<InspectorResult, Errcode>;
}
