use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    _organisation_id: i32,
    count: u32,
}

pub fn count(count: u32, organisation_id: i32) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            {
                LazyNodes::new(|f| f.text(format_args!("<turbo-frame id='negotiation-count'>")))
            }
            if cx.props.count == 0 {
                None
            } else {
                cx.render(rsx!(
                    Counter {
                        count: cx.props.count
                    }
                ))
            }
            {
                LazyNodes::new(|f| f.text(format_args!("</turbo-frame>")))
            }
        })
    }

    let mut app = VirtualDom::new_with_props(
        app,
        Props {
            count,
            _organisation_id: organisation_id,
        },
    );
    let _ = app.rebuild();
    dioxus::ssr::render_vdom(&app)
}
