use leptos::prelude::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main style="background:#fafafa;padding:20px;font-family:Inter,sans-serif">
            <h1 style="font-size:24px;color:#222;margin:0 0 12px">"Today"</h1>
            <div style="background:#fff;border-radius:12px;padding:16px;box-shadow:0 1px 4px rgba(0,0,0,.12)">
                <div style="font-size:12px;color:#8a8a8a">"Revenue"</div>
                <div style="font-size:28px;color:#1a73e8">"$12,480"</div>
            </div>
            <button style="margin-top:12px;background:#1a73e8;color:#fff;border:0;border-radius:8px;padding:10px 16px;font-size:14px">
                "New sale"
            </button>
        </main>
    }
}
