# TavernCommon

## Things to do

- [x] Rename the EnumTypes: 
  - `Live(T)`
  - `Frozen(T, &dyn Freezer)`

- [x] Make the Freezer trait
```rust
pub trait Freezer {
    // other things but i am not sure right now.
    pub agent(&self) -> String;
}
```
- [ ] Make a Freezer generator macro
- [ ] Make a freeze macro takes an `ArcticFox` & `dyn Fault` might need to be a proc macro.
- [x] Make the 
```rust
    /// Simple method to provide the value encased in the monad.
    pub fn freeze(&self) -> (T, Option<ArcticFoxError>){
        match self {
            Live(t) => { (t.clone(), None) }
            Freeze(t, e) => { return (t.clone(), Some(e.clone())) }
        }
    }
```
make the EnumType Frozen after using it ` *self = Frozen(t.clone, Freezer::default_freezer())`
- [ ] Get the Freezer agent: `let reason: String = fox.agent()` depending on what triggered the freezing.

- [ ] Make a function to iterate over an iterater and make it run async by having a queue
```rust
struct ArcticFoxIteraterQueue {
    data: // The vector in question
    item: //  refrence to the item data
    size: // uint of how much is done
    allc: // refrence to what should be done next
}
```
make this into actual code and not handwave-y and have the iteration be called as: `fox.pack(|i| { // loop over any i and deal with any falure while operating on i })`


