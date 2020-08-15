## TODO:
1. Propose options for class layout (diagram)

## Dev Issues:
1. "Wait for acknowledgements" will be a limited API if there isnt a convenient method to interrogate QoS. (Look into whether dynamic ack timeout is configureable)

## Observations
### Lifetimes
The most obvious way to cache symbols loaded from a library is to maintain a reference to loaded symbols and thier library, perhaps in one data structure. Rust lifetimes prevent you from doing this naively. If an object and its references are refernced at the same level (with the same inferred lifetime) we have a compiler error. It is necessary to distinguish between the lifetime of an object and its references such that dropping references occurs before dropping the referenced object. The library a symbol is loaded from is not particularly interesting after we have references to the symbols we need. It is only important that the library is still loaded *somewhere*. So, because Rust's lifetime guarantees mean that if a referenced symbol is valid, its' library is still valid, we can ignore the continued existence of the library and be sure that if we can use a cached symbol it is "safe" to do so. We need not concern ourselves with where the library is unloaded (or "dropped"). 