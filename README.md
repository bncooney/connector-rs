## TODO:
1. Move ReturnCode out of Reader, and implement on Writer
1. Add and implement "get_last_error"
1. Add appropriate ReturnCode handling
1. Improve consistency; "pointer" and "handle"
1. Create crate to enable async patterns (Futures, Stream etc.)
1. Update class layout (diagram)
1. Add type generation, write .idl parser using "nom"

## Dev Issues:
1. Decide on generic type handling in abstraction layer
1. "Wait for acknowledgements" will be a limited API if there isnt a convenient method to interrogate QoS. (Decide whether to call into DDS "core" library symbols)

## Design Decisions (and observations on Rust)
### Lifetimes
The most obvious way to cache symbols loaded from a library is to maintain a reference to loaded symbols and thier library, perhaps in one data structure. Rust lifetimes prevent you from doing this naively. If an object and its references are refernced at the same level (with the same inferred lifetime) we have a compiler error. It is necessary to distinguish between the lifetime of an object and its references such that dropping references occurs before dropping the referenced object. The library a symbol is loaded from is not particularly interesting after we have references to the symbols we need. It is only important that the library is still loaded *somewhere*. So, because Rust's lifetime guarantees mean that if a referenced symbol is valid, its' library is still valid, we can ignore the continued existence of the library and be sure that if we can use a cached symbol it is "safe" to do so. We need not concern ourselves with where the library is unloaded (or "dropped"). 

Until introducing wait, read, and take, it was not necessary for a Reader to have reference to its "Connector" at all. I had expected to call these with the Reader's handle, forgetting that DDS places these operations on the *Participant*. So, the reader requires a reference to its Connector / Participant. This has an, interesting, effect on lifetimes. Connector and Reader must have a lifetime shorter than thier library, and Reader must have a lifetime shorter than its Connector such that; library > connector > reader. As such, it is only *neccessary*, given the current design, that Reader has a lifetime shorter than its Connector. Although not strictly neccessary I have added both lifetimes, library and connector, to the reader. This does not effect the current calling convention whatsoever, and is indeed "redundant", but to provide a stable API and scope for flexibility I feel that this is an appropriate choice.

In time it would be interesting to use other DDS libraries (e.g. OpenDDS). This wouldn't be too difficult; extract DDS traits, break up the crate into interface and implementation. So to enable that work in the future, I have chosen to more closely model the DDS standard than the .js / .Net RTI Connext Connector implementations, and most operations now reside on the Participant (Connector).

## Class Diagram (WIP)
```
                  ┌──────────────────┐
                  │                  │
 ─ ─ ─ ─ ─ ─ ─ ─ ▷│  ConnextLibrary  │◁─ ─ ─ ─ ─ ─ ─ ─
│                 │                  │                │
                  └──────────────────┘
│                           △                         │
                            │
│                             &                       │
                            │
│                 ┌──────────────────┐                │
                  │                  │
│ &               │    Connector     │                │ &
                  │                  │
│                 └──────────────────┘                │
                            △
│                           │ &                       │
             ┌ ─ ─ ─ ─ ─ ─ ─└ ─ ─ ─ ─ ─ ─ ┐
│                                                     │
             │                            │
│  ┌──────────────────┐         ┌──────────────────┐  │
   │                  │         │                  │
└ ─│      Reader      │         │      Writer      │─ ┘
   │                  │         │                  │
   └──────────────────┘         └──────────────────┘
```
