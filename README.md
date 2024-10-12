# CodeDocs

This is a project that aims to develop a document model that allows to treat
source code as documents that can be manipulated and transformed into either
different source files or actual document types.

## Goals / Use Cases

The main use cases for this project are related to programming courses.
It should be possible to define various aspects of a programming course
directly in source files and provide a way to export these definitions
to different document formats.

### Assignments

Assume, for example, an assignment, where a function stub is given
along with a task description that specifies what the function does.
The assignment will also contain a solution and hints, and there
should be a way to generate the task with or wihtout hints and&or solutions.

Such a task could be defined by creating a source file that contains
the complete task including hints and solutions.
The task description would be provided using a doc comment for the function,
the solution would be marked inside the function's implementation using
line comments, and the hints would be provided after the function, also using comments.

The tool to be developed here should then be able to parse this source file
and produce versions where the solutions and/or hints are stripped away.

### Examples for Slides

Examples for use in slides or other companion documents could also be defined
in source files. The advantage of using a real source file that can actually
be compiled and run/tested is that this makes it easier to ensure that the
examples on the slides actually work and show the desired behaviour.

Therefore, to define examples in code, it should be possible to define it in
a source file. The parts that should actually be shown in documents could be
marked using comments. This is in fact similar to the requirements for
hints/solutions of assignments. We also need appropriate comments for this,
however, the result is not a different compilable source file, but rather
a snippet that can be included in other documents, or even another document
entirely (e.g. a Markdown or LaTeX file).

### Exams

For exams there are two use cases: defining and grading.
Defining an exam is similar (if not identical) to defining tasks in general.
There may be additional aspects like the maximum reachable score, which, however,
can be treated in a similar way.

The document model defined here should also be helpful for grading the submissions
handed in by students.
The idea is that comments could also be used for adding remarks to an existing solution.

Such a grading comment would have a special format to distinguish it from comments
that the students themselves may have put in the code.
The comment can also be used to set a score,
or a modifier for e.g. subtracting points for mistakes.

The document model must then be able to collect the grading comments
and add up / apply the score modifiers, or at least create a summary document from them.
There should also be a way to extract the code that is marked by grading comments
in order to create a report for the exam.

## Characteristics and Related Tools

The requirements that can be derived from the above examples might be
satisfied using different existing tools.

* Extracting source code can e.g. be done using parsers for the respective languages.
* Doc comments are parsed by documentation tools like e.g. Doxygen or JavaDoc,
  or are also included in parsers like for Go.
* Converting various types of documents into each other can be done using PanDoc.

Some of the problems here, however, require solutions that go beyond the scope of those tools:

* The tool should work for various programming languages,
  ruling out parsers/doc tools for specific languages.
* The tool should be able to produce source code ,
  which may be difficult using pandoc.
* The tool must be able to parse non compiling source code (e.g. for exam submissions).
  This also rules out parsers for existing programming languages.

In addition to these aspects, my goal is to produce a tool that can be deployed without
further dependencies. Thus, e.g. no Pandoc with extensions should be needed.
And, last but not least, I want to explore how to create a document model like this
and therefor simply want to do it myself.
Therefore, this will be developed independently of other similar tools.
However, I may consider integrating it with other tools later.
