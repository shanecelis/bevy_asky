//! Helper traits for creating common widgets

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    // ui::Val::*
};

/// Color palette for widgets
#[derive(Debug, Clone, Resource)]
pub struct Palette {
    /// Border color
    pub border: Color,
    /// Background color
    pub background: Color,
    /// Text color
    pub text: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            // interaction: InteractionPalette::default(),
            border: Color::BLACK,
            background: Color::WHITE.mix(&Color::BLACK, 0.8),
            text: Color::WHITE,
        }
    }
}

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>, palette: &Palette) -> EntityCommands;
    /// Spawn a column.
    fn column(&mut self) -> EntityCommands;
    /// Spawn a column whose elements can wrap around.
    fn column_wrap(&mut self) -> EntityCommands;

    // Spawn a simple header label. Bigger than [`Widgets::label`].
    // fn header(&mut self, text: impl Into<String>, palette: &Palette) -> EntityCommands;

    // Spawn a simple text label.
    // fn label(&mut self, text: impl Into<String>, palette: &Palette) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>, palette: &Palette) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            Button,
            Node {
                    margin: UiRect {
                        // right: Val::Px(5.0),
                        left: Val::Px(10.0),
                        ..default()
                    },
                    // width: Px(200.0),
                    // height: Px(65.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(palette.border.into()),
                BackgroundColor(palette.background.into()),
        ));
        entity.with_children(|children| {
            ChildBuild::spawn(
                children,
                (Name::new("Button Text"),
                 Text::new(text),
                 TextColor(palette.text),
                ));
        });
        entity
    }

    fn column(&mut self) -> EntityCommands {
        self.spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
    }

    fn column_wrap(&mut self) -> EntityCommands {
        self.spawn(Node {
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::Wrap,
            ..default()
        })
    }

    // fn header(&mut self, text: impl Into<String>, palette: &Palette) -> EntityCommands {
    //     let mut entity = self.spawn((
    //         Name::new("Header"),
    //         NodeBundle {
    //             style: Style {
    //                 width: Px(500.0),
    //                 height: Px(65.0),
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 ..default()
    //             },
    //             background_color: palette.background.into(),
    //             ..default()
    //         },
    //     ));
    //     entity.with_children(|children| {
    //         children.spawn((
    //             Name::new("Header Text"),
    //             TextBundle::from_section(
    //                 text,
    //                 TextStyle {
    //                     font_size: 40.0,
    //                     color: palette.header,
    //                     ..default()
    //                 },
    //             ),
    //         ));
    //     });
    //     entity
    // }

    // fn label(&mut self, text: impl Into<String>, palette: &Palette) -> EntityCommands {
    //     let mut entity = self.spawn((
    //         Name::new("Label"),
    //         NodeBundle {
    //             style: Style {
    //                 width: Px(500.0),
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //     ));
    //     entity.with_children(|children| {
    //         children.spawn((
    //             Name::new("Label Text"),
    //             TextBundle::from_section(
    //                 text,
    //                 TextStyle {
    //                     font_size: 24.0,
    //                     color: palette.label,
    //                     ..default()
    //                 },
    //             ),
    //         ));
    //     });
    //     entity
    // }
}

// /// An extension trait for spawning UI containers.
// pub trait Containers {
//     /// Spawns a root node that covers the full screen
//     /// and centers its content horizontally and vertically.
//     fn ui_root(&mut self) -> EntityCommands;
// }

// impl Containers for Commands<'_, '_> {
//     fn ui_root(&mut self) -> EntityCommands {
//         self.spawn((
//             Name::new("UI Root"),
//             NodeBundle {
//                 style: Style {
//                     width: Percent(100.0),
//                     height: Percent(100.0),
//                     justify_content: JustifyContent::Center,
//                     align_items: AlignItems::Center,
//                     flex_direction: FlexDirection::Column,
//                     row_gap: Px(10.0),
//                     position_type: PositionType::Absolute,
//                     ..default()
//                 },
//                 ..default()
//             },
//         ))
//     }
// }

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for EntityCommands<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.insert(bundle);
        self.reborrow()
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}
