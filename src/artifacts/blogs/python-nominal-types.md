---
title: "Safer Python Boundaries, Tiny Types, and Their Hidden Cost"
time: "2026-03-21"
tags:
  - python
  - performance
  - typing
  - dataclasses
---
When Python programs grow, a surprising number of bugs come from values that are technically the same shape but semantically different.

A `user_id` and an `email` might both be strings. An `account_code` and a `region_name` might both be strings. A function can accept the wrong one, and nothing will complain until much later, usually at a boundary that is far away from the mistake.

Typing helps because it lets us make those boundaries explicit. Instead of passing around anonymous strings, we can give values names and meaning. That makes programs easier to reason about, easier to refactor, and harder to misuse.

One common Python move is to wrap these values in a small dataclass:

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class UserId:
    value: str
```

This is pleasant for a lot of reasons. It is readable. It is explicit. It gives us a real runtime type, so `isinstance(user_id, UserId)` works. That matters if the goal is not just static annotation, but strict nominal boundaries that can also be checked at runtime.

The problem is that this convenience is not free.

This investigation started from a practical question: if we want tiny nominal types for safer Python programs, how much performance do we give up, and what is the least painful way to get runtime-checkable types?

## The Goal

The target here was narrow on purpose: create strict typing boundaries for Python programs using nominal types that behave like real runtime classes.

That means the lightweight static-only tools are not always enough. `typing.NewType` is excellent when a type checker is the only audience, but it does not create a real runtime type, so `isinstance(value, UserId)` is not available.

For some systems, that is fine. For others, especially systems that validate inputs at boundaries or want explicit domain objects moving through the codebase, runtime nominal typing is part of the design.

So the question becomes: what is the cheapest shape that still gives us that runtime distinction?

## What Was Tested

The benchmark compared a few tiny wrappers around a raw `str`:

- a frozen one-field dataclass
- a frozen dataclass with `slots=True`
- a manual `__slots__` class
- a `NamedTuple`
- a `str` subclass
- and a raw `str` baseline

The environment was Python `3.14.3`, with timing measured as the median of seven `timeit` runs over one million iterations. Memory was measured both with `sys.getsizeof` and with `tracemalloc` over large batch allocations.

That setup matters because the interesting question was not theoretical purity. It was whether these tiny wrappers are cheap enough to use as a default boundary pattern in real Python programs.

## The First Surprise: `isinstance` Is Not the Problem

It is easy to assume that runtime type checking is what hurts. It is not.

Positive `isinstance` checks landed at roughly `33 ns` across the board, whether the object was a raw `str`, a dataclass wrapper, a slots wrapper, or a `NamedTuple`.

That is the key framing for the whole discussion: the runtime type check itself is cheap once the object already exists.

The real cost is everything required to create and carry the wrapper.

## The Second Surprise: The Nice Dataclass Is Expensive

The frozen one-field dataclass was materially heavier than carrying a raw string.

- raw `str` baseline: about `10.3 ns` to reuse
- frozen dataclass: about `149.6 ns`
- frozen dataclass with `slots=True`: about `143.0 ns`

That means the ergonomic dataclass version came in at roughly `14.5x` the raw-string baseline. Even the slots variant was still around `13.9x`.

Memory told the same story.

- raw `str` baseline: about `8.1` bytes per instance in batch allocation
- frozen dataclass: about `88.1` bytes
- frozen dataclass with `slots=True`: about `48.1` bytes

This is where the practical tension shows up. Dataclasses are a very attractive way to express intent, but if a hot path creates many of these wrappers, the abstraction cost becomes real.

## What `slots=True` Actually Buys You

If you already like dataclasses, `slots=True` is the easiest optimization.

```python
from dataclasses import dataclass

@dataclass(frozen=True, slots=True)
class UserId:
    value: str
```

It does help, especially on memory. In this benchmark, batch allocation dropped from about `88.1` bytes per instance to about `48.1` bytes.

But it is important to be precise about what improved and what did not.

`slots=True` helped memory much more than speed. Construction only moved from about `149.6 ns` to `143.0 ns`, which is better, but not transformative.

So if the problem is primarily instance count and memory pressure, slots are a good default. If the problem is wrapper construction on a hot path, slots alone do not solve it.

## The Fastest Wrapper: Manual `__slots__`

If the goal is simply "give me a real runtime type with less overhead," the manual slots wrapper was the best tradeoff from the tested shapes.

```python
class UserId:
    __slots__ = ("value",)

    def __init__(self, value: str) -> None:
        self.value = value
```

This came in at about `70.3 ns` to construct, roughly half the cost of the frozen dataclass, while matching the slots-dataclass memory result at about `48.1` bytes per instance.

That does not make it universally better. You give up some dataclass conveniences and have to write more by hand. But if the program cares about runtime nominal typing and construction overhead, this shape is hard to ignore.

## Other Alternatives

There are a few other ways to slice the problem, each with a different compromise.

### `typing.NewType`

```python
from typing import NewType

UserId = NewType("UserId", str)
```

This is the "almost free" option. It has essentially zero runtime overhead and gives strong static meaning to type checkers.

But it is not a real runtime class. If you need `isinstance`, this does not satisfy the requirement.

### `str` Subclass

```python
class UserId(str):
    pass
```

This preserves string ergonomics nicely and still supports `isinstance(user_id, UserId)`.

But it was the heaviest option in the memory benchmark, at about `140.1` bytes per instance, and still slower to construct than the manual slots wrapper.

So it is useful when preserving string behavior matters more than memory footprint, but it is not the cheap path.

### `NamedTuple`

`NamedTuple` gives you a real runtime type, but in this benchmark it was slower than both dataclass variants and not especially compelling on memory for this use case.

### Raw Strings in the Core, Wrappers at the Edge

This is less a language feature and more an architectural alternative.

If runtime validation is most important at system boundaries, you can construct nominal wrappers at the edge, validate there, and then carry raw strings through the hottest internal paths.

That weakens end-to-end nominal guarantees, but it often gives a much better performance profile.

## So What Should You Use?

The answer depends on which property you are optimizing for.

- If static distinction is enough, `NewType` is the obvious winner.
- If you want runtime `isinstance` and the nicest default developer experience, `@dataclass(frozen=True, slots=True)` is a reasonable compromise.
- If you want runtime `isinstance` and lower wrapper overhead, a manual `__slots__` class is the strongest option from this investigation.
- If you want runtime `isinstance` and direct string behavior, a `str` subclass can work, but you are paying for it in memory.

The broader lesson is that Python typing choices are not just about correctness and readability. Once you turn types into real runtime objects, they become part of your performance model.

That does not mean the wrappers are a bad idea. It means they should be placed deliberately.

If the type only exists to keep humans and static analyzers honest, use the cheapest tool available.

If the type must survive at runtime and support nominal checks, accept that you are making a trade, and choose the shape that matches your workload.

For this investigation, the default answer was clear: if I want strict nominal runtime typing without paying the full cost of a regular frozen dataclass, I should start with either `@dataclass(frozen=True, slots=True)` or a manual `__slots__` wrapper, depending on whether I care more about ergonomics or raw speed.

## Comparison Table

| Approach | Runtime nominal | `isinstance` | Construction | Memory |
| --- | --- | --- | --- | --- |
| raw `str` | no | `str` only | `10.3 ns (1.0x)` | `8.1 B (1.0x)` |
| `typing.NewType` | no | no | `~1.0x` | `~1.0x` |
| `@dataclass(frozen=True)` | yes | yes | `149.6 ns (14.5x)` | `88.1 B (10.9x)` |
| `@dataclass(frozen=True, slots=True)` | yes | yes | `143.0 ns (13.9x)` | `48.1 B (5.9x)` |
| manual `__slots__` | yes | yes | `70.3 ns (6.8x)` | `48.1 B (5.9x)` |
| `NamedTuple` | yes | yes | `177.4 ns (17.2x)` | `72.1 B (8.9x)` |
| `str` subclass | yes | yes | `85.7 ns (8.3x)` | `140.1 B (17.3x)` |
