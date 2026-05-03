use leptos::prelude::*;

stylance::import_style!(style, "outputpane.css");

#[derive(Clone)]
pub enum OutputKind {
    Plain,
    Directory,
    Executable,
    MarkdownHeading,
    Error,
}

#[derive(Clone)]
pub struct OutputSpan {
    pub text: String,
    pub kind: OutputKind,
}

#[component]
pub fn Outputpane(#[prop(optional)] spans: Vec<OutputSpan>) -> impl IntoView {
    let rendered = spans
        .into_iter()
        .map(|span| {
            let class_name = match span.kind {
                OutputKind::Plain => style::plain,
                OutputKind::Directory => style::directory,
                OutputKind::Executable => style::executable,
                OutputKind::MarkdownHeading => style::markdown_heading,
                OutputKind::Error => style::error,
            };

            view! {
                <span class=class_name>{span.text}</span>
                <span class=style::gap>" "</span>
            }
        })
        .collect_view();

    view! {
        <div class=style::outputpane>
            <div class=style::line>{rendered}</div>
        </div>
    }
}
