use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::visualizer;

pub fn show(
    mut contexts: EguiContexts,
    mut toaster: ResMut<visualizer::resource::toaster::Toaster>,
    time: Res<Time>,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    let messages= toaster.get_messages(time.delta_secs());

    if messages.is_empty(){ return; }

    egui::Area::new("toaster_list".into())
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_TOP, [0.0, 8.0])
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::window(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(5, 10, 18, 210))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;

                    let text_color = egui::Color32::from_rgb(180, 190, 205);

                    let text = |text: &str| {
                        egui::RichText::new(text)
                            .color(text_color)
                            .size(10.0)
                    };

                    for toaster in messages {
                        ui.horizontal(|ui| {
                            ui.label(text(&toaster.text));
                            ui.separator();
                        });   
                    }
                });
        });
}