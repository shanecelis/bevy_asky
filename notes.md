# Notes on construct

I'm really taken with the possibilities of this proposal. It would be extremely
helpful for code I'm writing currently. So much so I thought I'd try using its
design lightly in practice. A few stumbling points came up that I thought
might be helpful to point out.

* `Handle<T>` is `Default + Clone` which means you have to choose between it and
  the blanket impl. Or perhaps consider a marker type on `Construct`.

* My old code looked like this:

```rust
    commands
        .spawn((
            NodeBundle { ..default() },
            AskyState::default(),
            Confirm {
                message: "Do you like ascii?".into(),
                init: None,
            },
        ))
        .observe(|trigger: Trigger<AskyEvent<bool>>| {
            eprintln!("trigger {:?}", trigger.event());
        });
```

and became this:

```rust
    commands
        .construct::<Confirm>("Do you like ascii?")
        .observe(|trigger: Trigger<AskyEvent<bool>>| {
            eprintln!("trigger {:?}", trigger.event());
        });
```

which I consider a win.

* This pattern didn't play as nicely with `ChildBuilder` however; a
  `.spawn_empty()` was required.

```rust
    commands.entity(column).with_children(|parent| {
        parent
            .spawn_empty() // spawn_empty() required in order to attach an observer.
            .construct::<ascii::View<Confirm>>("Do you like ascii?".into())
            .observe(
                move |trigger: Trigger<AskyEvent<bool>>, mut commands: Commands| {
                    eprintln!("trigger {:?}", trigger.event());
                },
            );
    });
```
  Perhaps my inferred `ConstructExt` is to blame though.
  
```rust
pub trait ConstructExt {
    fn construct<T: Construct + Component>(&mut self, props: impl Into<T::Props>) -> &mut Self
    where
        <T as Construct>::Props: Send;
}
```

## Questions

* Because `ConstructContext` contains the `id: Entity` and `&mut World`, as I
  was writing my first impls to use it, I wondered about interrogating the
  entity for further information. Questions that came to mind are this: Given
  this code
  
```rust
commands
    .construct::<A>(())
    .construct::<B>(());
```
  
  when B's `Construct` impl runs, can we expect A to be present on the entity?
  Or in code:
  
```rust

impl Construct for B {
    type Props = ();

    fn construct(
        context: &mut ConstructContext,
        props: Self::Props,
    ) -> Result<Self, ConstructError> {
        assert!(context.world.entity(context.id).contains::<A>(), "No A component present.");
        Ok(B)
    }
```
   
  I'm not suggesting it would be good practice to encourage this kind of
  coupling. If B requires A, then it can `#[require(A)]`. But if A is optional,
  B's `Construct` may wish to interrogate it.

* 

## Expected Deviations of My Implementation

* I did not implement `#[require]`. I merely immediately added required components in each
  `Construct` impl using `id` and `world`.
  
* 
