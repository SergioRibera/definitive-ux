use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_title("Tally")))
}

fn app() -> impl IntoElement {
    rect()
        .background((250, 250, 252))
        .padding(14.)
        .spacing(10.)
        .width(Size::fill())
        .child(label().font_size(22.).color((30, 30, 40)).text("Today"))
        .child(StatCard { title: "Revenue".into(), value: "$12,480".into() })
        .child(StatCard { title: "Orders".into(), value: "184".into() })
        .child(
            Button::new()
                .on_press(|_| {})
                .child(label().color((255, 255, 255)).text("New sale")),
        )
}

#[derive(PartialEq)]
struct StatCard {
    title: String,
    value: String,
}

impl Component for StatCard {
    fn render(&self) -> impl IntoElement {
        rect()
            .background((255, 255, 255))
            .corner_radius(10.)
            .padding(18.)
            .shadow((0., 2., 8., 0., (0, 0, 0, 30)))
            .child(label().font_size(13.).color((120, 120, 130)).text(self.title.clone()))
            .child(label().font_size(26.).color((16, 92, 220)).text(self.value.clone()))
    }
}
