use leptos::prelude::*;
use leptos::html;
use shell::lexer::Lexer;
use shell::parser::parse;

use crate::components::outputpane::{OutputKind, OutputLine, Outputpane};
use crate::components::pathline::Pathline;
use crate::components::promptline::Promptline;

stylance::import_style!(style, "shell.css");

#[derive(Clone)]
struct TerminalEntry {
    id: usize,
    path: String,
    command: String,
    outputs: Vec<OutputLine>,
}

#[component]
pub fn Shell() -> impl IntoView {
    let (history, set_history) = signal(Vec::<String>::new());
    let (entries, set_entries) = signal(Vec::<TerminalEntry>::new());
    let next_entry_id = RwSignal::new(0usize);
    let shell_content_ref: NodeRef<html::Div> = NodeRef::new();

    let on_submit = Callback::new(move |command: String| {
        let mut lexer = Lexer::init(command.as_str());
        lexer.next();

        let mut next_output_lines = Vec::<OutputLine>::new();

        match parse(&mut lexer) {
            Some(ast) => {
                let ast_pretty = format!("{:#?}", ast);

                next_output_lines.push(OutputLine {
                    text: "AST:".to_string(),
                    kind: OutputKind::Executable,
                });

                next_output_lines.extend(ast_pretty.lines().map(|line| OutputLine {
                    text: line.to_string(),
                    kind: OutputKind::Plain,
                }));
            }
            None => {
                next_output_lines.push(OutputLine {
                    text: "Parse error: parser returned None".to_string(),
                    kind: OutputKind::Error,
                });
            }
        }

        let entry_id = next_entry_id.get_untracked();
        next_entry_id.set(entry_id + 1);

        set_entries.update(|values| {
            values.push(TerminalEntry {
                id: entry_id,
                path: "~/".to_string(),
                command: command.clone(),
                outputs: next_output_lines,
            });

            if values.len() > 120 {
                let drop_count = values.len() - 120;
                values.drain(0..drop_count);
            }
        });

        set_history.update(|values| {
            values.push(command);
            if values.len() > 10 {
                let drop_count = values.len() - 10;
                values.drain(0..drop_count);
            }
        });
    });

    Effect::new(move |_| {
        let _ = entries.get().len();
        if let Some(container) = shell_content_ref.get() {
            container.set_scroll_top(container.scroll_height());
        }
    });

    view! {
        <main class=style::shell_page>
            <section class=style::shell_frame>
                <div class=style::shell_content node_ref=shell_content_ref>
                    <For
                        each=move || entries.get()
                        key=|entry| entry.id
                        children=move |entry| {
                            view! {
                                <article class=style::terminal_entry>
                                    <Pathline user="user".to_string() cwd=entry.path.clone() />
                                    <div class=style::promptline_static>
                                        <span class=style::prompt_symbol>"❯"</span>
                                        <span class=style::prompt_command>{entry.command}</span>
                                    </div>
                                    <Outputpane lines=entry.outputs.clone() />
                                </article>
                            }
                        }
                    />

                    <article class=style::terminal_entry>
                        <Pathline user="user".to_string() cwd="~/".to_string() />
                        <Promptline history=history on_submit=on_submit />
                    </article>
                </div>
            </section>
        </main>
    }
}

pub use Shell as ShellPage;
