# bevy_asky

This library is intended to make asking questions of the user easy. It is not
intended to provide a comprehensive UI beyond question and answer and may indeed
be better thought of as scaffolding for whatever your eventual UI may become.

# Usage

```rust
commands
    .construct::<Confirm>("Do you like cats?")
    .construct::<ascii::View>(())
    .observe(
        move |trigger: Trigger<AskyEvent<bool>>, mut commands: Commands| {
            if let AskyEvent(Ok(yes)) = trigger.event() {
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
