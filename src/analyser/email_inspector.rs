use crate::analyser::*;

pub struct EmailInspectorResult{
    emails: Vec<String>,
    hosts: Vec<String>,
}

impl EmailInspectorResult{
    pub fn void() -> EmailInspectorResult{
        EmailInspectorResult{ emails: vec![], hosts: vec![] }
    }
}



pub struct EmailInspector{

}

impl HtmlInspector for EmailInspector{
    fn new() -> Self where Self: Sized{
        EmailInspector { }
    }

    fn analyse_html(&mut self, html: &String) -> Result<InspectorResult, Errcode>{
        println!("Analysing HTML code for e-mail addresses ...\n");
        Ok(InspectorResult::Email(EmailInspectorResult::void()))
    }
}













/*              TESTS               */
#[test]
fn test_email_get_all(){
    let tests_emails = vec![
        "a@A.com",
        "32.23@A.com",
        "a9234235b@A.com",
        "a_b@A.com",
        "a.b@A.com",
        "a@A.B.com",
        "a@contact.A.com",
        "a@B.com",
        "a@A.fr",
    ];
    let tests_false_emails = vec![
        "a @ A.com",
        "a@ A.com",
        "a @A.com",
        "a;b@A.com",
        "a@A",
    ];
    let mut test_vector = String::from("<html><body>");
    for email in tests_emails.iter(){
        test_vector += "<div>";
        test_vector += email;
        test_vector += "</div>\n";
    }
    for email in tests_false_emails.iter(){
        test_vector += "<div>";
        test_vector += email;
        test_vector += "</div>\n";
    }
    test_vector += "</body></html>";
    let mut inspector = EmailInspector::new();
    let InspectorResult::Email(result) = inspector.analyse_html(&test_vector).unwrap();
    
    println!("");
    for email in tests_emails.iter(){
        println!("Checking that inspector caught email \"{}\" ?", email);
        assert!(result.emails.contains(&String::from(*email)));
    }
    
    for email in tests_false_emails.iter(){
        println!("Checking that inspector didn't caught email \"{}\" ?", email);
        assert!(!result.emails.contains(&String::from(*email)));
    }
}
