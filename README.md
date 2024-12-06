# bevy_asky

This library is intended to make asking questions of the user easier using the
[Bevy game engine](https://bevyengine.org). It is not intended to provide a
comprehensive UI beyond question and answer, and may indeed be better thought of
as scaffolding for whatever one's eventual UI may become.

# Architecture

This crate uses a Model-View-Controller (MVC) architecture. Normally, I am not
too enthusiastic about MVC because there is a lot of ambiguity about what goes
where especially when it comes to the controller aspect. However, I found that
within Bevy's Entity-Component-Systems (ECS) architecture, MVC has much clearer boundaries. 

## Model

The models are all found in the `bevy_asky::prompt` module.

- checkbox
- confirm 
- number
- password
- radio button
- text field
- toggle

## Controller

The controllers are all implemented as systems and are not exposed to the user. If you prompt for a text field, and then hit 'a', the text field will append an 'a' character. It will not be shown though unless it has a view component.

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

- [ ] Rename `AskyState` to ...?
- [ ] Design a setting for what to do when input is submitted, possible options:
  - nothing
  - block focus (take no more input)
  - despawn
- [ ] Add a `button::View` that uses mouse-clickable elements.

# Acknowledgments
Thanks to [Axel Vasquez](https://github.com/axelvc) for his excellent and inspiring [asky](https://github.com/axelvc/asky) crate.
