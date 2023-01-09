# ArcticFox

ArcticFox is a Monad, but in more rusty terms:
- Freezeable: The monad refuses updates when an operation panics and the monad will wrap the value in the `Usuccessful(T, Err)` enum option.
```rust
pub struct Foo {}

// also implement Model for Foo

let response: ArcticFox(Foo) = Successful(Foo::default());

response.run(|t| {
    if let Err(e) = panicing_function() {
        freeze!(response, e)
    } else // will never get here because the fictional function will always panic
})

response.run(|t| {
    // nothing will happen here beveause now the response is frozen.
})
```
- Functional: The monad is functional and `run` and `async_run` can be chained together. 
```rust
response.run().async_run()...
```

- *ox: it is a quasy box poiner where you can always eject or orphan the cub stored in the value when done operating. By calling the `response.orphan()` method you will get the value stored and the arctic fox is frozen as a result.

## Why

This was originally written to make my life easier in writing a common library for apis for the app I am writing; however, i soon realized I had the beginnings of this really wierd but ultimately helpful datastructure. 

## When should I use?

When you dont really care if there is an error during operation. You care about at the end of operation **WAS** there an error, and if there was what was the reason?

## How should i think about `ArcticFox`

Think of it as an ArcticFox goes on a journey with a cub. If the cub makes it throught the journey without any hicups: the cub is `Live(T)`. If the data you are useing errors in some way then the cub is frozen for the cub's saftey by the ArcticFox parent and then when the cub is given where needed, andthe ArcticFox relinqueshes ownership when `ArcticFox::freeze()` is called. This autimatically halts any operation on the cub and effectively makes it static.

## In the future
 
- [ ] Have features for Actix
- [ ] Have derive macro for the Cub trait.
- [ ] Optimize ;)
