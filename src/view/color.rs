use crate::construct::*;
use crate::prelude::*;
use bevy::{
    ecs::{
        system::{
            SystemParam,
            SystemState,
        },
        world::Command,
        query::QueryEntityError,
    },
    prelude::*,
};
use std::fmt::Write;

#[derive(Component, Reflect)]
pub struct View;

// #[derive(Debug, Component, Reflect)]
#[repr(u8)]
enum ViewPart {
    Focus = 0,
    Header = 1,
    PreQuestion = 2,
    Question = 3,
    Answer = 4,
    Options = 5,
    Feedback = 6,
}

impl Construct for View {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        _props: Self::Props,
    ) -> Result<Self, ConstructError> {
        // let mut system_state: SystemState<Query<&Parent>> = SystemState::new(&mut context.world);
        // let parents = system_state.get(&context.world);

        let mut commands = context.world.commands();
        commands
            .entity(context.id)
            .insert(NodeBundle::default());
            // .with_children(|parent| {
            //     // Q: Why have these broken into different bundles?
            //     // A: So I can control the background color independently.
            //     parent.spawn(TextBundle::default()); // Focus
            //     parent.spawn(TextBundle::default()); // Header
            //     parent.spawn(TextBundle::default()); // PreQuestion
            //     parent.spawn(TextBundle::default()); // Question
            //     parent.spawn(TextBundle::default()); // Answer
            //     parent
            //         .spawn(NodeBundle::default()) // Answer
            //         // .spawn_empty() // Options
            //         .with_children(|parent| {
            //             parent.spawn(TextBundle::default());
            //             parent.spawn(TextBundle::default());
            //             parent.spawn(TextBundle::default());
            //             parent.spawn(TextBundle::default());
            //         });
            //     parent.spawn(TextBundle::default()); // Feedback
            // });

        context.world.flush();

        Ok(View)
    }
}

#[derive(SystemParam)]
struct Inserter<'w, 's, C: Component> {
    roots: Query<
        'w,
        's,

            &'static mut C
            // Option<&'static mut BackgroundColor>,
       ,
    >,
    children: Query<'w, 's, &'static Children>,
    commands: Commands<'w, 's>,
}

impl<'w, 's, C: Component> Inserter<'w, 's, C> {

    fn insert_or_get_child(&mut self,
                           root: Entity,
                           index: usize,
    ) -> Result<Entity, Entity> {
        match self.children.get(root) {
            Ok(children) => {
                if index < children.len() {
                    Ok(children[index])
                } else {
                    let mut id = None;
                    self.commands.entity(root).with_children(|parent| {
                        for _ in children.len()..index {
                            parent.spawn(TextBundle::default());
                        }
                        id = Some(parent.spawn(TextBundle::default()).id());
                    });
                    Err(id.unwrap())
                }
            }
            _ => {
                let mut id = None;
                self.commands.entity(root).with_children(|parent| {
                    for _ in 0..index {
                        parent.spawn(TextBundle::default());
                    }
                    id = Some(parent.spawn(TextBundle::default()).id());
                });
                Err(id.unwrap())
            }
        }
    }

    fn insert_or_get_mut<F>(&mut self,
                            root: Entity,
                            index: usize,
                            apply: F,
    ) -> Result<(), QueryEntityError>
    where F: Fn(&mut C), C: Default {
        match self.children.get(root) {
            Ok(children) => {
                if index < children.len() {
                    self.roots.get_mut(children[index]).map(|mut t: Mut<C>| apply(&mut t))
                } else {
                    // dbg!(index, children.len());
                    self.commands.entity(root).with_children(|parent| {
                        for _ in children.len()..index {
                            parent.spawn(TextBundle::default());
                        }
                        let mut text = C::default();
                        apply(&mut text);
                        parent
                            .spawn(TextBundle::default())
                            .insert(text);
                    });
                    Ok(())
                }
            }
            _ => {
                self.commands.entity(root).with_children(|parent| {
                    for _ in 0..index {
                        parent.spawn(TextBundle::default());
                    }
                    let mut text = C::default();
                    apply(&mut text);
                    parent
                        .spawn(TextBundle::default())
                        .insert(text);
                });
                Ok(())
            }
        }
    }
}

#[derive(Debug, Resource, Component)]
struct Palette {
    text_color: Srgba,
    background: Option<Srgba>,
    highlight: Srgba,
    complete: Srgba,
    answer: Srgba,
    lowlight: Srgba,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            text_color: Srgba::WHITE,
            background: None,
            highlight: Srgba::hex("80ADFA").unwrap(),
            complete: Srgba::hex("94DD8D").unwrap(),
            answer: Srgba::hex("FFB9E8").unwrap(),
            lowlight: Srgba::hex("5A607A").unwrap(),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (
            super::add_view_to_checkbox::<View>,
            super::add_view_to_radio::<View>,
        ),
    );
    app.add_systems(
        Update,
        (
            (focus_view,
            header_view,
            radio_view,
            checkbox_view,
            prompt_view,
            text_view,
            password_view,
            confirm_view,
            toggle_view,
            feedback_view).chain(),
            clear_feedback::<StringCursor>,
            clear_feedback::<Toggle>,
        ),
    )
    .insert_resource(Palette::default());
}

pub(crate) fn replace_or_insert(text: &mut Text, index: usize, replacement: &str) {
    let len = text.sections.len();
    if len <= index {
        for i in len.saturating_sub(1)..index {
            text.sections.push(TextSection::default());
        }
        text.sections.push(TextSection::from(replacement));
    } else {
        text.sections[index].value.replace_range(.., replacement);
    }
}

pub(crate) fn prompt_view(
    mut query: Query<(Entity, &Prompt), (With<View>, Changed<Prompt>)>,
    mut writer: Inserter<Text>,
) {
    for (id, prompt) in query.iter_mut() {
        writer
            .insert_or_get_mut(id,
                               ViewPart::Question as usize,
                               |mut text| {
                                    replace_or_insert(
                                        &mut text,
                                        0,
                                        prompt);
                               })
            .expect("prompt");
    }
}

pub(crate) fn feedback_view(
    mut query: Query<(Entity, &Feedback), (With<View>, Changed<Feedback>)>,
    mut writer: Inserter<Text>,
) {
    for (id, feedback) in query.iter_mut() {
        writer
            .insert_or_get_mut(id,
                               ViewPart::Feedback as usize,
                               |mut text| {
                                   replace_or_insert(&mut text, 0, &format!(" {}", feedback.message));
                               })
            .expect("feedback");
    }
}

pub(crate) fn clear_feedback<T: Component>(
    mut query: Query<&mut Feedback, (With<View>, Changed<T>)>,
) {
    for mut feedback in query.iter_mut() {
        feedback.clear();
    }
}

pub(crate) fn focus_view(
    mut query: Query<(Entity, &Focusable), (With<View>, Changed<Focusable>)>,
    mut writer: Inserter<Text>,
    palette: Res<Palette>,
) {
    for (id, focusable) in query.iter_mut() {
        writer
            .insert_or_get_mut(id,
                               ViewPart::Focus as usize,
                               |mut text| {
                                    replace_or_insert(
                                        &mut text,
                                        0,
                                        match focusable.state() {
                                            FocusState::Focused => "> ",
                                            _ => "  ",
                                        },
                                    );
                                    text.sections[0].style.color = palette.highlight.into();
                               })
            .expect("focus");
    }
}

pub(crate) fn header_view(
    mut query: Query<(Entity, &AskyState), (With<View>, Changed<AskyState>)>,
    mut writer: Inserter<Text>,
) {
    for (id, asky_state) in query.iter_mut() {
        writer
            .insert_or_get_mut(id,
                               ViewPart::Header as usize,
                               |mut text| {
                                   replace_or_insert(
                                       &mut text,
                                       0,
                                       match asky_state {
                                           AskyState::Reading => "[ ] ",
                                           AskyState::Complete => "[x] ",
                                           AskyState::Error => "[!] ",
                                       },
                                   );
                               })
            .expect("header");
    }
}

pub(crate) fn text_view(
    mut query: Query<
        (Entity, &StringCursor, &Children, Option<&Placeholder>, &Focusable),
        (
            With<View>,
            Without<Password>,
            Or<(Changed<StringCursor>, Changed<Focusable>)>,
        ),
    >,
    mut texts: Query<&mut Text>, //, &mut BackgroundColor)>,
    mut sections: Query<&Children>,
    palette: Res<Palette>,
    mut commands: Commands,
) {
    for (root, text_state, children, placeholder, focusable) in query.iter_mut() {
        let index = ViewPart::Answer as usize;
        let id = if index < children.len() {
            children[index]
        } else {
            let mut new_node = None;
            commands.entity(root).with_children(|parent| {
                for _ in children.len()..index {
                    parent.spawn(TextBundle::default());
                }
                new_node = Some(parent.spawn(TextBundle::default()).id());
            });
            new_node.unwrap()
        };
        if let Ok(cursor_parts) = sections.get(id) {
            let mut parts = texts.iter_many_mut(cursor_parts);
            if focusable.state() == FocusState::Focused {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert(&mut pre_cursor, 0, &text_state.value[0..text_state.index]);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert(
                    &mut cursor,
                    0,
                    if text_state.index >= text_state.value.len() {
                        " "
                    } else {
                        &text_state.value[text_state.index..text_state.next_index()]
                    },
                );
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert(
                    &mut post_cursor,
                    0,
                    &text_state.value[text_state.next_index()..],
                );
            } else {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert(&mut pre_cursor, 0, &text_state.value);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert(&mut cursor, 0, "");
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert(&mut post_cursor, 0, "");
            }
        } else {
            // Make the parts.
            commands.entity(id).with_children(|parent| {
                // pre cursor
                parent.spawn(TextBundle::from_section(
                    &text_state.value[0..text_state.index],
                    TextStyle::default(),
                ));
                // cursor
                parent.spawn(
                    TextBundle::from_section(
                        if text_state.index >= text_state.value.len() {
                            " "
                        } else {
                            &text_state.value[text_state.index..text_state.next_index()]
                        },
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_background_color(Color::WHITE),
                );
                // post cursor
                parent.spawn(TextBundle::from_section(
                    &text_state.value[text_state.next_index()..],
                    TextStyle::default(),
                ));
            });
        }
    }
}

pub(crate) fn password_view(
    mut query: Query<
        (Entity, &StringCursor, &Children, Option<&Placeholder>, &Focusable),
        (
            With<View>,
            With<Password>,
            Or<(Changed<StringCursor>, Changed<Focusable>)>,
        ),
    >,
    mut texts: Query<&mut Text>, //, &mut BackgroundColor)>,
    mut sections: Query<&Children>,
    palette: Res<Palette>,
    mut commands: Commands,
) {
    for (root, text_state, children, placeholder, focusable) in query.iter_mut() {
        let glyph = "*";
        let index = ViewPart::Answer as usize;
        let id = if index < children.len() {
            children[index]
        } else {
            let mut new_node = None;
            commands.entity(root).with_children(|parent| {
                for _ in children.len()..index {
                    parent.spawn(TextBundle::default());
                }
                new_node = Some(parent.spawn(TextBundle::default()).id());
            });
            new_node.unwrap()
        };
        if let Ok(cursor_parts) = sections.get(id) {
            let mut parts = texts.iter_many_mut(cursor_parts);
            if focusable.state() == FocusState::Focused {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert_rep(&mut pre_cursor, 0, glyph, text_state.index);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert_rep(
                    &mut cursor,
                    0,
                    if text_state.index >= text_state.value.len() {
                        " "
                    } else {
                        glyph
                    },
                    1,
                );
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert_rep(
                    &mut post_cursor,
                    0,
                    glyph,
                    text_state.value.len().saturating_sub(text_state.index + 1),
                );
            } else {
                let mut pre_cursor = parts.fetch_next().expect("pre cursor");
                replace_or_insert(&mut pre_cursor, 0, &text_state.value);
                let mut cursor = parts.fetch_next().expect("cursor");
                replace_or_insert(&mut cursor, 0, "");
                let mut post_cursor = parts.fetch_next().expect("post cursor");
                replace_or_insert(&mut post_cursor, 0, "");
            }
        } else {
            // Make the parts.
            commands.entity(id).with_children(|parent| {
                // pre cursor
                parent.spawn(TextBundle::from_section(
                    glyph.repeat(text_state.index),
                    TextStyle::default(),
                ));
                // cursor
                parent.spawn(
                    TextBundle::from_section(
                        if text_state.index >= text_state.value.len() {
                            " "
                        } else {
                            glyph
                        },
                        TextStyle {
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                    .with_background_color(Color::WHITE),
                );
                // post cursor
                parent.spawn(TextBundle::from_section(
                    glyph.repeat(text_state.value.len().saturating_sub(text_state.index)),
                    TextStyle::default(),
                ));
            });
        }
    }
}

pub(crate) fn toggle_view(
    mut query: Query<
        (Entity, &AskyState, &Toggle),
        (With<View>, Or<(Changed<AskyState>, Changed<Toggle>)>),
    >,
    palette: Res<Palette>,
    mut commands: Commands,
    mut writer: Inserter<BackgroundColor>,
) {
    for (root, asky_state, toggle) in query.iter_mut() {
        let id = match writer.insert_or_get_child(root, ViewPart::Options as usize) {
            Ok(options) => {

                writer.insert_or_get_mut(options,
                                1,
                                |mut color| {
                                    *color = if toggle.index == 0 {
                                        palette.highlight.into()
                                    } else {
                                        palette.lowlight.into()
                                    };
                                })
                .expect("option 0");

                writer.insert_or_get_mut(options,
                                3,
                                |mut color| {
                                    *color = if toggle.index == 1 {
                                        palette.highlight.into()
                                    } else {
                                        palette.lowlight.into()
                                    };
                                })
                .expect("option 1");
            }
            Err(new) => {
                commands.entity(new).with_children(|parent| {
                    let style = TextStyle::default();
                    parent.spawn(TextBundle::from_section(" ", style.clone())); // 0
                    parent.spawn(TextBundle::from_section(format!(" {} ", toggle.options[0]), style.clone())
                                .with_background_color(if toggle.index == 0 {
                                    palette.highlight.into()
                                } else {
                                    palette.lowlight.into()
                                })); // 1
                    parent.spawn(TextBundle::from_section(" ", style.clone())); // 2
                    parent.spawn(TextBundle::from_section(format!(" {} ", toggle.options[1]), style) // 3
                                .with_background_color(if toggle.index == 1 {
                                    palette.highlight.into()
                                } else {
                                    palette.lowlight.into()
                                }));
                });
            }
        };
    }
}

pub(crate) fn confirm_view(
    mut query: Query<
        (Entity, &AskyState, &Confirm),
        (With<View>, Or<(Changed<AskyState>, Changed<Confirm>)>),
    >,
    palette: Res<Palette>,
    mut commands: Commands,
    mut writer: Inserter<BackgroundColor>,
) {
    for (root, asky_state, confirm) in query.iter_mut() {
        let id = match writer.insert_or_get_child(root, ViewPart::Options as usize) {
            Ok(options) => {

                writer.insert_or_get_mut(options,
                                1,
                                |mut color| {
                                    *color = if ! confirm.yes {
                                        palette.highlight.into()
                                    } else {
                                        palette.lowlight.into()
                                    };
                                })
                .expect("option 0");

                writer.insert_or_get_mut(options,
                                3,
                                |mut color| {
                                    *color = if confirm.yes {
                                        palette.highlight.into()
                                    } else {
                                        palette.lowlight.into()
                                    };
                                })
                .expect("option 1");
            }
            Err(new) => {
                commands.entity(new).with_children(|parent| {
                    let style = TextStyle::default();
                    parent.spawn(TextBundle::from_section(" ", style.clone())); // 0
                    parent.spawn(TextBundle::from_section(" No ", style.clone())
                                .with_background_color(if !confirm.yes {
                                    palette.highlight.into()
                                } else {
                                    palette.lowlight.into()
                                })); // 1
                    parent.spawn(TextBundle::from_section(" ", style.clone())); // 2
                    parent.spawn(TextBundle::from_section(" Yes ", style) // 3
                                .with_background_color(if confirm.yes {
                                    palette.highlight.into()
                                } else {
                                    palette.lowlight.into()
                                }));
                });
            }
        };
    }
}

pub(crate) fn checkbox_view(
    mut query: Query<
        (Entity, &Checkbox, &Focusable),
        (With<View>, Or<(Changed<Checkbox>, Changed<Focusable>)>),
    >,
    palette: Res<Palette>,
    mut writer: Inserter<Text>,
) {
    for (id, checkbox, focusable) in query.iter_mut() {
        writer
            .insert_or_get_mut(id,
                               ViewPart::PreQuestion as usize,
                               |mut text| {
                                   replace_or_insert(&mut text, 0, if checkbox.checked { "[x] " } else { "[ ] " });
                                   text.sections[0].style.color = if focusable.state() == FocusState::Focused {
                                       palette.highlight.into()
                                   } else {
                                       palette.text_color.into()
                                   };
                               })
            .expect("prequestion");
    }
}

pub(crate) fn radio_view(
    mut query: Query<
        (Entity, &Radio, &Focusable),
        (With<View>, Or<(Changed<Radio>, Changed<Focusable>)>),
    >,
    palette: Res<Palette>,
    mut writer: Inserter<Text>,
) {
    for (id, radio, focusable) in query.iter_mut() {
        writer
            .insert_or_get_mut(id,
                               ViewPart::PreQuestion as usize,
                               |mut text| {
                                   replace_or_insert(&mut text, 0, if radio.checked { "(x) " } else { "( ) " });
                                   text.sections[0].style.color = if focusable.state() == FocusState::Focused {
                                       palette.highlight.into()
                                   } else {
                                       palette.text_color.into()
                                   };
                               })
            .expect("prequestion");
    }
}
