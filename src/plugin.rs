use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{InspectableWidget, Options};

#[derive(Default)]
pub struct InspectorPlugin<T>(PhantomData<T>);

impl<T> InspectorPlugin<T> {
    pub fn new() -> Self {
        InspectorPlugin(PhantomData)
    }
}

impl<T> Plugin for InspectorPlugin<T>
where
    T: InspectableWidget + Default + Send + Sync + 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(EguiPlugin)
            .init_resource::<T>()
            .add_system(ui::<T>.system());
    }
}

fn ui<T>(mut egui_context: ResMut<EguiContext>, mut data: ResMut<T>)
where
    T: InspectableWidget + Send + Sync + 'static,
{
    let ctx = &mut egui_context.ctx;

    egui::Window::new("Inspector")
        .resizable(false)
        .show(ctx, |ui| {
            data.ui(ui, Options::default());
        });
}