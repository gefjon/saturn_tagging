# NaNBoxing

`Saturn` is the name of my planned byte-interpreted virtual machine. So far, the
only code I have written for it is this repository, NaNboxing primitives.

## `NaNboxing`

In dynamically typed, interpreted languages, it is often benificial to develop a
system such that all types fit within a single machine word, either as a pointer
to a value on the heap or as an immediate value. In modern machines, a word is
64 bits, and being able to fit all of a language's representable values into 64
bits - half of a cache line - gives immense performance improvements over using
64 bits + a tag, for a total of one cache line per value.

This provides a problem when using 64-bit floats as a value, since intuitively a
64-bit floating-point number plus a tag cannot fit within 64 bits. The solution
to this problem is called *NaNboxing*, and it allows Javascript engines, and in
theory any dynamically typed language, to pass 64-bit floats by value while
still keeping their unified `Value` type within 64 bits.

An ANSI 64-bit float has the following components, from MSB to LSB:

* a 1-bit sign

* an 11-bit exponent

* a 52-bit mantissa

There are two special values than an exponent might hold which cause the
mantissa to be interpreted differently:

* `0b111_1111_1111` or `0x7ff` holds the `NaN` block - the value is either `NaN`
  or `Infinity`
  
* `0b000_0000_0000` or `0x0` holds the signed zeros and subnormals

Of these two, only the `NaN` block is interesting to us. A mantissa of `0x0`
denotes `Infinity`, which may be positive or negative depending on the sign. All
other combinations denote `NaN` and are treated the same way. Arithmetic
operations which result in `NaN` will only ever produce a mantissa of `0x1`, so
all other `2^53 - 4` possibilities, counting the sign bit, are unused.

Because x86-64 pointers are restricted to 48 bits, we can safely use bits 49-52
as a 4-bit tag, which leaves 16 possible types that can be encoded. The sign
tends to be best left untouched, so that signed integers can be encoded without
having to worry about sign-extends and such. As long as the type denoted by the
tag `0x0` is an untagged 8- or 16-bit-aligned pointer (to avoid conflicts with
the actual arithmetic `NaN` and `Infinity` values), we can store 16 different
types with payloads of 48 bits + sign and our normal 64-bit-float values, all
within half a cache line.

## This library

This library exposes utilities for tagging and untagging NaNboxed values. It has
only been tested in my `x86-64` macOS High Sierra machine, but it should work on
Windows and Linux 64-bit (and probably on 64-bit ARM architectures as well).

In the future, I intend to add traditional pointer-tagging (in the low 3/4 bits
of aligned pointers), but I do not currently have the free time to pursue this
project.
