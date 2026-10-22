use freya::prelude::*;

#[derive(Default, Clone, PartialEq)]
struct Form {
    name: String,
    email: String,
    phone: String,
    card: String,
    country: String,
    zip: String,
}

#[derive(PartialEq)]
pub struct Checkout {
    pub countries: Vec<(String, String)>, // ~250 entries
}

impl Component for Checkout {
    fn render(&self) -> impl IntoElement {
        let mut form = use_state(Form::default);
        let mut error = use_state(|| String::new());
        let theme = get_theme();

        let submit = move |_| {
            let f = form.read().clone();
            if f.phone.len() != 10 || !f.phone.chars().all(|c| c.is_ascii_digit()) {
                error.set("Invalid input".into());
                form.set(Form::default());
                return;
            }
            if f.card.len() != 16 {
                error.set("Invalid input".into());
                return;
            }
            spawn(async move {
                let _ = pay(f).await;
            });
        };

        rect()
            .padding(14.)
            .spacing(14.)
            .width(Size::fill())
            .child(
                rect()
                    .horizontal()
                    .main_align(Alignment::SpaceBetween)
                    .child(label().font_size(18.).text("Checkout"))
                    .child(
                        rect()
                            .width(Size::px(16.))
                            .height(Size::px(16.))
                            .on_press(move |_| RouterContext::get().go_back())
                            .child("✕"),
                    ),
            )
            .child(Input::new(form.read().name.clone(), move |v| form.write().name = v).placeholder("Full name"))
            .child(Input::new(form.read().email.clone(), move |v| form.write().email = v).placeholder("Email"))
            .child(Input::new(form.read().phone.clone(), move |v| form.write().phone = v).placeholder("Phone"))
            .child(Input::new(form.read().card.clone(), move |v| form.write().card = v).placeholder("Card number"))
            .child(
                Select::new(form.read().country.clone(), move |v| form.write().country = v)
                    .options(self.countries.clone()),
            )
            .child(Input::new(form.read().zip.clone(), move |v| form.write().zip = v).placeholder("ZIP"))
            .maybe(!error.read().is_empty(), |r| {
                r.child(label().font_size(11.).color(theme.danger).text(error.read().clone()))
            })
            .child(
                rect()
                    .horizontal()
                    .spacing(6.)
                    .child(Button::new().on_press(move |_| form.set(Form::default())).child("Reset"))
                    .child(Button::new().on_press(submit).child("Pay")),
            )
    }
}

async fn pay(_f: Form) -> Result<(), String> {
    Ok(())
}
