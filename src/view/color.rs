//! Uses colored text
use crate::{construct::*, prelude::*, string_cursor::*};
use bevy::{
    ecs::{query::QueryEntityError, system::SystemParam},
    prelude::*,
};
use std::borrow::Cow;

const PADDING: Val = Val::Px(5.);

/// Marker for color views
#[derive(Component, Reflect, Default)]
pub struct View;

/// - Node
///   - Text, Focus
///     - TextSpan, PreQuestion
///     - TextSpan, Question
///     - TextSpan, Answer
///     - Text, PreCursor, Toggle0
///     - Text, Cursor
///     - Text, PostCursor, Toggle1
///     - Text, Feedback
#[derive(Debug, Reflect, Component)]
#[repr(u8)]
enum ViewPart {
    Focus = 0,
    PreQuestion = 1,
    Question = 2,
    Answer = 3,
    PreCursor = 4,
    Toggle0 = 5,
    Cursor = 6,
    Toggle1 = 7,
    PostCursor = 8,
    Feedback = 9,
}


#[derive(SystemParam)]
pub(crate) struct ViewWriter<'w, 's> {
    writer: TextUiWriter<'w, 's>,
    children: Query<'w, 's, &'static Children>,
    commands: Commands<'w, 's>,
}

impl<'w, 's> ViewWriter<'w, 's> {

    fn entity(&mut self, root: Entity, part: ViewPart) -> Entity {
        use ViewPart::*;
        let children = self.children.get(root).expect("view children");
        match part {
            Focus => children[0],
            PreQuestion | Question | Answer => {
                let span_children = self.children.get(children[0]).expect("text children");
                span_children[part as usize - 1]
            }
            PreCursor | Toggle0 => children[1],
            Cursor => children[2],
            PostCursor | Toggle1 => children[3],
            Feedback => children[4],
        }
    }

    fn text(&mut self, root: Entity, part: ViewPart) -> Mut<'_, String> {
        use ViewPart::*;
        let children = self.children.get(root).expect("view children");
        match part {
            Focus | PreQuestion | Question | Answer => self.writer.text(children[0], part as usize),
            PreCursor | Toggle0 => self.writer.text(children[1], 0),
            Cursor => self.writer.text(children[2], 0),
            PostCursor | Toggle1 => self.writer.text(children[3], 0),
            Feedback => self.writer.text(children[4], 0),
        }
    }
    fn color(&mut self, root: Entity, part: ViewPart) -> Mut<'_, TextColor> {
        use ViewPart::*;
        let children = self.children.get(root).expect("view children");
        match part {
            Focus | PreQuestion | Question | Answer => self.writer.color(children[0], part as usize),
            PreCursor | Toggle0 => self.writer.color(children[1], 0),
            Cursor => self.writer.color(children[2], 0),
            PostCursor | Toggle1 => self.writer.color(children[3], 0),
            Feedback => self.writer.color(children[4], 0),
        }
    }
}

#[derive(Debug, Component, Reflect)]
struct Cursor;

#[derive(Resource, Deref, DerefMut, Reflect)]
struct CursorBlink(Timer);

impl Construct for View {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        _props: Self::Props,
    ) -> Result<Self, ConstructError> {

        let highlight = context.world.resource::<Palette>().highlight;
        if let Ok(mut eref) = context.world.get_entity_mut(context.id) {
            if !eref.contains::<Node>() {
                eref.insert(Node::default());
            }
            eref.with_children(|node| {
                node.spawn((Text::default(), TextColor(highlight.into()))) // Focus
                    .with_children(|parent| {
                        parent.spawn(TextSpan::default()); // PreQuestion
                        parent.spawn(TextSpan::default()); // Question
                        parent.spawn(TextSpan::default()); // Answer
                    });
                node.spawn(Text::default()); // PreCursor, Toggle0
                node.spawn(Text::default()); // Cursor
                node.spawn(Text::default()); // PostCursor, Toggle1
                node.spawn(Text::default()); // Feedback
            });
        }
        Ok(View)
    }
}

#[derive(SystemParam)]
pub(crate) struct Inserter<'w, 's, C: Component> {
    roots: Query<'w, 's, &'static mut C>,
    children: Query<'w, 's, &'static Children>,
    commands: Commands<'w, 's>,
}

impl<'w, 's, C: Component> Inserter<'w, 's, C> {
    fn insert_or_get_child(
        &mut self,
        root: Entity,
        index: usize,
    ) -> Result<Entity, Option<Entity>> {
        match self.children.get(root) {
            Ok(children) => {
                if index < children.len() {
                    Ok(children[index])
                } else {
                    let mut id = None;
                    if let Some(mut ecommands) = self.commands.get_entity(root) {
                        ecommands.with_children(|parent| {
                            for _ in children.len()..index {
                                parent.spawn(Text::default());
                            }
                            id = Some(parent.spawn(Text::default()).id());
                        });
                    }
                    Err(id)
                }
            }
            _ => {
                let mut id = None;
                if let Some(mut ecommands) = self.commands.get_entity(root) {
                    ecommands.with_children(|parent| {
                        for _ in 0..index {
                            parent.spawn(Text::default());
                        }
                        id = Some(parent.spawn(Text::default()).id());
                    });
                }
                Err(id)
            }
        }
    }

    fn insert_or_get_mut<F>(
        &mut self,
        root: Entity,
        index: usize,
        apply: F,
    ) -> Result<(), QueryEntityError>
    where
        F: Fn(&mut C),
        C: Default,
    {
        match self.children.get(root) {
            Ok(children) => {
                if index < children.len() {
                    self.roots
                        .get_mut(children[index])
                        .map(|mut t: Mut<C>| apply(&mut t))
                } else {
                    // dbg!(index, children.len());
                    if let Some(mut ecommands) = self.commands.get_entity(root) {
                        ecommands.with_children(|parent| {
                            for _ in children.len()..index {
                                parent.spawn(Text::default());
                            }
                            let mut text = C::default();
                            apply(&mut text);
                            parent.spawn(Text::default()).insert(text);
                        });
                    }
                    Ok(())
                }
            }
            _ => {
                if let Some(mut ecommands) = self.commands.get_entity(root) {
                    ecommands.with_children(|parent| {
                        for _ in 0..index {
                            parent.spawn(Text::default());
                        }
                        let mut text = C::default();
                        apply(&mut text);
                        parent.spawn(Text::default()).insert(text);
                    });
                }
                Ok(())
            }
        }
    }
}

/// The color palette
#[derive(Debug, Resource, Component, Reflect)]
pub struct Palette {
    /// Text color
    pub text_color: Srgba,
    /// Background color
    pub background: Option<Srgba>,
    /// Highlight color
    pub highlight: Srgba,
    /// Complete color
    pub complete: Srgba,
    /// Answered color
    pub answer: Srgba,
    /// Lowlight color
    pub lowlight: Srgba,
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

/// Add color views handlers.
pub fn plugin(app: &mut App) {
    app.register_type::<View>()
        .register_type::<ViewPart>()
        .register_type::<Cursor>()
        .register_type::<CursorBlink>()
        .register_type::<Palette>()
        .add_systems(
            PreUpdate,
            (
                (
                    focus_view,
                    radio_view,
                    checkbox_view,
                    prompt_view,
                    text_view::<Without<Password>>,
                    opaque_view::<With<Password>>,
                    option_view::<Confirm>,
                    option_view::<Toggle>,
                    feedback_view,
                )
                    .chain(),
                clear_feedback::<StringCursor>,
                clear_feedback::<Toggle>,
                blink_cursor,
            )
                .in_set(AskySet::View),
        )
        .insert_resource(CursorBlink(Timer::from_seconds(
            1.0 / 3.0,
            TimerMode::Repeating,
        )))
        .insert_resource(Palette::default());
}

pub(crate) fn prompt_view(
    mut writer: ViewWriter,
    mut query: Query<(Entity, &Prompt), (With<View>, Changed<Prompt>)>,
) {
    for (id, prompt) in query.iter_mut() {
        writer.text(id, ViewPart::Question)
            .replace_range(.., prompt);
    }
}

pub(crate) fn feedback_view(
    query: Query<(Entity, &Feedback), (With<View>, Changed<Feedback>)>,
    mut node: Query<&mut Node>,
    mut writer: ViewWriter,
) {
    for (id, feedback) in &query {
        writer.text(id, ViewPart::Feedback).replace_range(.., &feedback.message);
        node.get_mut(writer.entity(id, ViewPart::Feedback)).unwrap().margin = UiRect {
            left: PADDING,
            ..default()
        };
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
    focus: Focus,
    mut query: Query<Entity, Or<(Changed<View>, Changed<Focusable>)>>,
    mut writer: ViewWriter,
    palette: Res<Palette>,
) {
    for id in query.iter_mut() {
        writer.text(id, ViewPart::Focus)
            .replace_range(.., if focus.is_focused(id) { "> " } else { "  " });
    }
}

/// Displays a [StringCursor] matching a query filter.
pub(crate) fn text_view<F: bevy::ecs::query::QueryFilter>(
    query: Query<
        (Entity, &StringCursor, Option<&Placeholder>),
        (
            With<View>,
            F,
            Or<(Changed<StringCursor>, Changed<Focusable>)>,
        ),
    >,
    palette: Res<Palette>,
    mut commands: Commands,
    focus: Focus,
    mut writer: ViewWriter,
) {
    for (id, text_state, placeholder) in query.iter() {
        writer.text(id, ViewPart::PreCursor)
            .replace_range(.., &text_state.value[0..text_state.index]);
        writer.text(id, ViewPart::Cursor)
            .replace_range(..,
                           if text_state.value.is_empty() && placeholder.is_some() {
                               let p = placeholder.unwrap();
                               &p[0..ceil_char_boundary(p, 1)]
                           } else if text_state.index >= text_state.value.len() {
                               " "
                           } else {
                               &text_state.value[text_state.index..text_state.next_index()]
                           });
        commands.entity(writer.entity(id, ViewPart::Cursor))
                .insert(Cursor);
        if text_state.value.is_empty() && placeholder.is_some() {
            let p = placeholder.unwrap();
            writer.text(id, ViewPart::PostCursor)
                  .replace_range(.., &p[ceil_char_boundary(p, 1)..]);
            writer.color(id, ViewPart::PostCursor).0 = palette.lowlight.into();
        } else {
            writer.text(id, ViewPart::PostCursor)
                .replace_range(.., &text_state.value[text_state.next_index()..]);
            writer.color(id, ViewPart::PostCursor).0 = palette.text_color.into();
        }
    }
}

/// Displays a [StringCursor] matching a query filter.
pub(crate) fn opaque_view<F: bevy::ecs::query::QueryFilter>(
    query: Query<
        (Entity, &StringCursor, Option<&Placeholder>),
        (
            With<View>,
            F,
            Or<(Changed<StringCursor>, Changed<Focusable>)>,
        ),
    >,
    palette: Res<Palette>,
    mut commands: Commands,
    focus: Focus,
    mut writer: ViewWriter,
) {
    for (id, text_state, placeholder) in query.iter() {
        let glyph = "*";
        let mut pre = writer.text(id, ViewPart::PreCursor);
        pre.clear();
        write_rep(&mut *pre, glyph, text_state.index);
        let mut cursor = writer.text(id, ViewPart::Cursor);
        cursor.clear();
        if text_state.value.is_empty() && placeholder.is_some() {
            let p = placeholder.unwrap();
            cursor.replace_range(.., &p[0..ceil_char_boundary(p, 1)]);
        } else if text_state.index >= text_state.value.len() {
            cursor.replace_range(.., " ");
        } else {
            write_rep(&mut *cursor, glyph, 1);
        }
        commands.entity(writer.entity(id, ViewPart::Cursor))
                .insert(Cursor);
        if text_state.value.is_empty() && placeholder.is_some() {
            let p = placeholder.unwrap();
            writer.text(id, ViewPart::PostCursor)
                  .replace_range(.., &p[ceil_char_boundary(p, 1)..]);
            writer.color(id, ViewPart::PostCursor).0 = palette.lowlight.into();
        } else {
            let mut post = writer.text(id, ViewPart::PostCursor);
            post.clear();
            write_rep(&mut *post, glyph, text_state.value.len().saturating_sub(text_state.index + 1));
            writer.color(id, ViewPart::PostCursor).0 = palette.text_color.into();
        }
    }
}

pub(crate) fn option_view<C: Component + OptionPrompt>(
    mut query: Query<(Entity, &C), (With<View>, Or<(Changed<Focusable>, Changed<C>)>)>,
    palette: Res<Palette>,
    mut commands: Commands,
    mut writer: ViewWriter,
    mut node: Query<&mut Node>,
    mut background: Query<&mut BackgroundColor>,
) {
    // TODO: Shouldn't this just show the answer when it is not in focus?
    for (id, confirm) in query.iter_mut() {

        let toggle0 = writer.entity(id, ViewPart::Toggle0);
        writer.text(id, ViewPart::Toggle0).replace_range(.., &confirm.name(0));
        // *writer.color(id, ViewPart::Toggle0) =
        *background.get_mut(toggle0)
                  .expect("background color") =
                if confirm.state() == 0 {
                    palette.highlight
                } else {
                    palette.lowlight
                }.into();

        let mut node0 = node.get_mut(toggle0).unwrap();
        node0.margin = UiRect {
            right: PADDING,
            left: PADDING,
            ..default()
        };
        node0.padding = UiRect {
            right: PADDING,
            left: PADDING,
            ..default()
        };
        commands.entity(toggle0)
            .insert(TextLayout { justify: JustifyText::Center, ..default() });

        let toggle1 = writer.entity(id, ViewPart::Toggle1);
        writer.text(id, ViewPart::Toggle1).replace_range(.., &confirm.name(1));
        *background.get_mut(toggle1)
                  .expect("background color") =
                if confirm.state() == 1 {
                    palette.highlight
                } else {
                    palette.lowlight
                }.into();
        let mut node1 = node.get_mut(toggle1).unwrap();
        node1.margin = UiRect {
            right: PADDING,
            left: PADDING,
            ..default()
        };
        node1.padding = UiRect {
            right: PADDING,
            left: PADDING,
            ..default()
        };
        commands.entity(toggle1)
            .insert(TextLayout { justify: JustifyText::Center, ..default() });
    }
}

pub(crate) fn checkbox_view(
    mut query: Query<
        (Entity, &Checkbox),
        (With<View>, Or<(Changed<Checkbox>, Changed<Focusable>)>),
    >,
    palette: Res<Palette>,
    mut writer: ViewWriter,
    focus: Focus,
) {
    for (id, checkbox) in query.iter_mut() {
        writer.text(id, ViewPart::PreQuestion).replace_range(.., if checkbox.checked { "[x] " } else { "[ ] " });
        *writer.color(id, ViewPart::PreQuestion) = if focus.is_focused(id) {
            palette.highlight.into()
        } else {
            palette.text_color.into()
        };
    }
}

pub(crate) fn radio_view(
    mut query: Query<(Entity, &Radio), (With<View>, Or<(Changed<Radio>, Changed<Focusable>)>)>,
    palette: Res<Palette>,
    mut writer: ViewWriter,
    focus: Focus,
) {
    for (id, radio) in query.iter_mut() {
        writer.text(id, ViewPart::PreQuestion).replace_range(.., if radio.checked { "(x) " } else { "( ) " });
        *writer.color(id, ViewPart::PreQuestion) = if focus.is_focused(id) {
            palette.highlight.into()
        } else {
            palette.text_color.into()
        };
    }
}

fn blink_cursor(
    mut query: Query<(Entity, &mut BackgroundColor), With<Cursor>>,
    mut timer: ResMut<CursorBlink>,
    time: Res<Time>,
    mut count: Local<u8>,
    focus: Focus,
    palette: Res<Palette>,
    mut writer: TextUiWriter,
    parent: Query<&Parent>,
) {
    if timer.tick(time.delta()).just_finished() {
        *count = count.checked_add(1).unwrap_or(0);
        for (id, mut color) in &mut query {
            if focus.is_focused(id) || parent.iter_ancestors(id).any(|id| focus.is_focused(id))
            {

                color.0 = if *count % 2 == 0 {
                    Color::WHITE
                } else {
                    Color::NONE
                }.into();

                *writer.color(id, 0) = if *count % 2 == 0 {
                    Color::BLACK.into()
                } else {
                    palette.text_color.into()
                };
            }
        }
    }
}
