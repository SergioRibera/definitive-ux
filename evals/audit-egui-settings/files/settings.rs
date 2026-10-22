use egui::{Color32, RichText, Sense, Ui};

#[derive(Clone, Copy, PartialEq)]
pub enum Frequency { Instant, Daily, Weekly }

pub struct NotificationSettings {
    pub email: bool,
    pub push: bool,
    pub frequency: Frequency,
    pub share_with_partners: bool,
    pub show_cancel_modal: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self { email: true, push: true, frequency: Frequency::Daily, share_with_partners: true, show_cancel_modal: false }
    }
}

impl NotificationSettings {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing.y = 24.0;

        ui.label("Channels");
        ui.checkbox(&mut self.email, "Email");
        ui.checkbox(&mut self.push, "Push");

        ui.label("Frequency");
        ui.horizontal(|ui| {
            for (f, name) in [(Frequency::Instant, "Instant"), (Frequency::Daily, "Daily"), (Frequency::Weekly, "Weekly")] {
                let color = if self.frequency == f { Color32::from_rgb(59, 130, 246) } else { Color32::GRAY };
                let r = ui.add(egui::Label::new(RichText::new(name).color(color)).sense(Sense::click()));
                if r.clicked() { self.frequency = f; }
            }
        });

        ui.label("Privacy");
        ui.checkbox(&mut self.share_with_partners, "Share my data with trusted partners");

        ui.horizontal(|ui| {
            if ui.button("Save").clicked() { save(self); }
            if ui.button("Cancel subscription").clicked() { self.show_cancel_modal = true; }
            if ui.button("Export data").clicked() { export(); }
        });

        if self.show_cancel_modal {
            egui::Window::new("Wait!").collapsible(false).show(ui.ctx(), |ui| {
                ui.label("Are you sure? You'll lose all your premium benefits and your friends will miss you.");
                if ui.button("Keep my benefits").clicked() { self.show_cancel_modal = false; }
                let r = ui.add(egui::Label::new(RichText::new("no thanks, I don't want to save money").size(10.0).color(Color32::from_gray(140))).sense(Sense::click()));
                if r.clicked() { cancel(); self.show_cancel_modal = false; }
            });
        }
    }
}

fn save(_s: &NotificationSettings) {}
fn export() {}
fn cancel() {}
