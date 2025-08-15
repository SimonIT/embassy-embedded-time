# embassy-embedded-time

Provides an
[`embedded-time::Clock`](https://docs.rs/embedded-time/latest/embedded_time/clock/trait.Clock.html)
using [`embassy_time`] so that `embedded-time` can easily be used in on-host testing.

### Usage

It's extremely straight-forward to start using a clock:

```rust
use embassy_embedded_time::EmbassyClock;
use embedded_time::Clock;

fn main() {
    let clock = EmbassyClock::default();

    let now = clock.try_now().unwrap();
    println!("Current time: {:?}", now);
}
```
