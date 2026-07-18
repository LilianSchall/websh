use leptos::prelude::*;

stylance::import_style!(style, "outputpane.css");

#[derive(Clone)]
pub enum OutputKind {
    Plain,
    Executable,
    Error,
}

#[derive(Clone)]
pub struct OutputLine {
    pub text: String,
    pub kind: OutputKind,
}

#[component]
pub fn Outputpane(#[prop(optional)] lines: Vec<OutputLine>) -> impl IntoView {
    let rendered = lines.into_iter().map(|line| {
        let class_name = match line.kind {
            OutputKind::Plain => style::plain,
            OutputKind::Executable => style::executable,
            OutputKind::Error => style::error,
        };

        view! {
            <li class=style::line>
                <span class=class_name>{line.text}</span>
            </li>
        }
    });

    view! {
        <ul class=style::outputpane>
            {rendered.collect_view()}
        </ul>
    }
}
