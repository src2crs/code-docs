# Code Document Model

This module contains a document model for source documents.

## State of Development

Currently, this is a concept being actively developed.
The current data structures allow for describing a source file from Rust source code.
This serves to explore how such a document model might look like.

Note that it is unclear whether this model is also useful for parsing code.
The same block of code might be interpreted just as a bunch or lines or as something
more structured. Things are ambiguous right now.

It might be useful to develop a low level grammar for the source files that describes
the tokens that are to be expected.
This grammar would identify things like comments, source lines,
or more structured comments with a label (e.g. `// EXAMPLE`).
However, it would not yet allow for giving these tokens a semantic.
I.e. it would not allow to parse a complete example block
that is guarded by begin and end comments into the fenced block element that is
currently defined in the sources of this module.
