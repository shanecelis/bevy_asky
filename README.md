# bevy_asky

This library is intended to make asking questions of the user easier within an
application built with [Bevy](https://bevyengine.org). It is not
intended to provide a comprehensive UI beyond question-and-answer, and may
indeed be better thought of as scaffolding for whatever one's eventual UI may
become.

> [!WARNING]
> `bevy_asky` is currently in the early stages of development and is subject to
> breaking changes. The principle consumer of this crate is
> [bevy_minibuffer](https://github.com/shanecelis/bevy_minibuffer), a gamedev
> console. As such it is under-developed for usage independent of
> `bevy_minibuffer` currently.

# Architecture

This crate uses a Model-View-Controller (MVC) architecture. Normally I am not
too enthusiastic about MVC because there is a lot of ambiguity about what goes
where especially when it comes to the controller aspect. However, I found that
within Bevy's Entity-Component-Systems (ECS) architecture, MVC enjoys much
clearer boundaries.

## Model

The models are all found in the `bevy_asky::prompt` module. They represent the
data that is being manipulated.

- checkbox
- confirm 
- number
- password
- radio button
- text field
- toggle

## Controller

The controllers are all implemented as systems and are not exposed to the user.
If you prompt for a text field, and then hit 'a', the text field will append an
'a' character. It will not be shown though unless it has a view component.

## View

The view handles presentation. One chooses which view by using a marker
component. There are two view modules in this crate: 'ascii' and 'color'. Their
marker components are `ascii::View` and `color::View` respectively.

One can use a view of their own. The configurability of these particular views
are limited. It is suggested to copy-and-paste [ascii.rs](/src/view/ascii.rs) or
[color.rs](/src/view/colors.rs) for fine-grained control of the presentation.

# Usage

![Run of code below](https://github.com/user-attachments/assets/3570dc94-2042-494c-b926-dfa06ea30cb6)

```rust
commands
    .construct::<Confirm>("Do you like cats?")
    .construct::<ascii::View>(())
    .observe(
        move |trigger: Trigger<Submit<bool>>, mut commands: Commands| {
            if let Submit(Ok(yes)) = trigger.event() {
                commands.entity(trigger.entity())
                        .construct::<Feedback>(Feedback::info(if *yes {
                            "\nMe too!"
                        } else {
                            "\nOk."
                        }));
            }
        },
    );
```

## TODO

- [ ] Design a setting for what to do when input is submitted, possible options:
  - do nothing,
  - block focus (take no more input),
  - or despawn.
- [ ] Make keys re-bindable.
- [ ] Add a `button::View` that uses mouse-clickable elements.

  There is old button code that used to do this, but it has rotted and no longer
  compiles.

# Compatibility

| bevy_asky | bevy |
|-----------|------|
| 0.1.0     | 0.14 |

# Acknowledgments
Thanks to [Axel Vasquez](https://github.com/axelvc) for his excellent and
inspiring [asky](https://github.com/axelvc/asky) crate. 

> [!NOTE] 
> I originally tried to extend Vasquez's work from its terminal origin to work
> directly with Bevy. You can find that work in [my
> fork](https://github.com/shanecelis/asky), but it required a lot of
> compromises and pull requests needed on dependencies were not being accepted.
> So I decided to do a native-port of asky to bevy; this crate is the
> result.

# License

This crate is licensed under the MIT License or the Apache License 2.0.
