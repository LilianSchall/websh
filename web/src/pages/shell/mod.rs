use leptos::prelude::*;
use shell::lexer::Lexer;
use shell::parser::parse;

use crate::components::outputpane::{OutputKind, OutputSpan, Outputpane};
use crate::components::pathline::Pathline;
use crate::components::promptline::Promptline;

stylance::import_style!(style, "shell.css");

#[component]
pub fn Shell() -> impl IntoView {
    let sample_spans = vec![
        OutputSpan {
            text: "$".to_string(),
            kind: OutputKind::Plain,
        },
        OutputSpan {
            text: "src".to_string(),
            kind: OutputKind::Directory,
        },
        OutputSpan {
            text: "target".to_string(),
            kind: OutputKind::Directory,
        },
        OutputSpan {
            text: "README.md".to_string(),
            kind: OutputKind::MarkdownHeading,
        },
        OutputSpan {
            text: "(errors styled when parser/executor returns diagnostics)".to_string(),
            kind: OutputKind::Error,
        },
    ];

    view! {
        <main class=style::shell_page>
            <section class=style::shell_frame>
                <header class=style::shell_header>
                    <Pathline />
                </header>
                <div class=style::shell_content>
                    <Promptline command_preview="echo 'hello from websh'".to_string() />
                    <Outputpane spans=sample_spans />
                </div>
            </section>
        </main>
    }
}

pub use Shell as ShellPage;
