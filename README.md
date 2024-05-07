# jacklang
experiments with the language of the people

Broadly, my aim for this language is to learn llvm development,
but also to realise something I believe is a "good" general 
purpose, reasonably high performance garbage collected language.

My belief in what a good language is over time has aligned closer
to C than it has to say, Haskell, for quite a few reasons.

I believe that developer experience is important, and that
includes nice error handling, a good editor experience, and nice
build tools. The latter two are stretch goals (this is a personal)
project, but certainly the former is handleable with a decent
lexer and good parser. Things like lsp's are not much more than
a customised lexer/parser with a specific format; I intend to
expose all parts of the compilation process as individualised
libraries to aid development.

Broadly, I want something that has a nice type system that's not
*too* nice a type system. Generic structs that are bound by traits
are good, I don't want to go down the path of purity to the point
of needing (explicit) effect handlers. I do want immutable by
default. In the unlikely event that this a) finishes and b)
develops a community that actually uses it, I don't want
immutable at any cost with default collections being immutable
data structures.

The rust iterator system is particularly nice, it's macro system
leaves a lot to be desired. Linear types are frankly brilliant,
and RAII provides opportunities for developers to implement
something like a drop trait for memory arenas, network sockets,
file handles etc. while still allowing for default GC.

Finally, I believe that a good language should compile very, very
quickly. Ideally a debug build should take maybe a second to
compile 500k loc.

A very large stretch goal is to provide first class FFI with Rust
for scenarios where doing something particularly high performance
is required... or maybe there's just a good library or something,
haven't thought this part through yet.

tl;dr, I want something that has a nice type system, compiles
well, has nice developer experience, exposes ways of maximising
performance for when required, has a good macro system, has
something like rust's error handling, and has a good generational
garbage collector.

progress:

- [x] lexer will process a million lines in something like 40 ms on
my macbook air m3. This is acceptably quick.
- [ ] grammar is finalised and documented in bnf
- [ ] semantics of the language have been documented
- [ ] experiments with a variety of parsers have been conducted,
a good one has been chosen
- [ ] a good (hopefully *great*) garbage collector implemented...
I'm very cognizant that Ocaml spent a LONG time making their
allocation/gc support multiple threads. I'm thinking I should
focus on this up front.
- [ ] simple debug builds can be made from one file
- [ ] trait system implemented
- [ ] generics implemented
- [ ] simple debug builds can be made from multiple files!
