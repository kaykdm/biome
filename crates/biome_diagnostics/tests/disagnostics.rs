use biome_console::fmt::{Formatter, HTML};
use biome_console::markup;
use biome_diagnostics::{DiagnosticTags, MessageAndDescription, PrintDiagnostic};
use biome_diagnostics_macros::Diagnostic;

#[derive(Diagnostic, Debug)]
#[diagnostic(
    severity = Error,
    category = "check"
)]
struct TestDiagnostic {
    #[message]
    #[description]
    message_and_description: MessageAndDescription,

    #[tags]
    tags: DiagnosticTags,

    #[location(resource)]
    path: String,
}

impl Default for TestDiagnostic {
    fn default() -> Self {
        Self {
            message_and_description: MessageAndDescription::from(markup!(
                "This is the message of the diagnostic. It will appear in different colours based on the severity of the diagnostic."
            ).to_owned()),
            tags: DiagnosticTags::FIXABLE,
            path: "code_block".to_string()
        }
    }
}

#[test]
fn console_print_diagnostic() {}

#[test]
fn html_print_diagnostic() {
    let mut content = vec![];
    let mut writer = HTML::new(&mut content);

    Formatter::new(&mut writer)
        .write_markup(markup! {
            {PrintDiagnostic::simple(&TestDiagnostic::default())}
        })
        .unwrap();

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(String::from_utf8(content).unwrap());
    });
}

#[test]
fn html_mdx_print_diagnostic() {
    let mut content = vec![];
    let mut writer = HTML::new(&mut content).with_mdx();

    Formatter::new(&mut writer)
        .write_markup(markup! {
            {PrintDiagnostic::simple(&TestDiagnostic::default())}
        })
        .unwrap();

    let content = String::from_utf8(content).unwrap();
    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(content);
    });
}