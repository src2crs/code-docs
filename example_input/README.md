# Examples

This directory contains examples for source files
that define different aspects of a programming course.

These examples may later become integration tests.

## Requirements

This section collects requirements that are derived from the examples in this directory.

*Note*: This is work in progress and may not be the final requirements for the project.

### Interpret Doc Comments as Task

* A doc comment in a source file that contains a task should be interpreted directly
as a task description, without further tags.
* This should apply to plain comments (like in Go) as well as to structured doc comments
  like when using Doxygen/JavaDoc or like they are used by Rust.
* This is useful, because this kind of comment will also be interpreted by IDEs used by students.

### Label Sections with Comments

Examples:

* `SOLUTION` ... `SOLUTION_END`
* `// EXAMPLE` ... `// EXAMPLE_END`
* `// EXAMPLE_BEGIN` ... `// EXAMPLE_END`
* `// EXAMPLE[tag]` ... `// EXAMPLE_END`
* `// EXAMPLE[tag]` ... `// EXAMPLE_END[tag]`
* `// HINTS`
* `// HINTS[tag]`
* `// HINTS[tag]` ... `// HINTS_END[tag]`

These examples show that there are several viable ways for defining sections.
The general idea is to have begin and end tags in the form of line comments.
These can have different forms, include labels, etc.
The system shpould be as flexible as possible:

* An end tag with a label closes a corresponding begin tag.
* An end tag without a label closes the nearest begin tag.
* A begin tag without an end tag is closed by the next begin tag or by EOF.
* ... (more rules needed?)

Ideally, these can be mixed freely.
It should be configurable what tags exist and whether some of them should be ignored.
This way, the same source file might be used for a task as well as as an example.

### Allow Block Comments

Note how the `submission` example hides the submitted solution in a block comment.
When grading the submission, it should be compiled regularly, of course.
However, as the code doesn't compile, this may become a problem when compiling
multiple submissions at once.

Therefore, it should be possible to define complete code blocks inside such
block comments and the tool should be able to generate a separate file from it
that can be tested.
The same may be useful e.g. for a task that asks the student to fix errors in existing code.
