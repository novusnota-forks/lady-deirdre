<!------------------------------------------------------------------------------
  This file is a part of the "Lady Deirdre" work,
  a compiler front-end foundation technology.

  This work is proprietary software with source-available code.

  To copy, use, distribute, and contribute to this work, you must agree to
  the terms of the General License Agreement:

  https://github.com/Eliah-Lakhin/lady-deirdre/blob/master/EULA.md.

  The agreement grants you a Commercial-Limited License that gives you
  the right to use my work in non-commercial and limited commercial products
  with a total gross revenue cap. To remove this commercial limit for one of
  your products, you must acquire an Unrestricted Commercial License.

  If you contribute to the source code, documentation, or related materials
  of this work, you must assign these changes to me. Contributions are
  governed by the "Derivative Work" section of the General License
  Agreement.

  Copying the work in parts is strictly forbidden, except as permitted under
  the terms of the General License Agreement.

  If you do not or cannot agree to the terms of this Agreement,
  do not use this work.

  This work is provided "as is" without any warranties, express or implied,
  except to the extent that such disclaimers are held to be legally invalid.

  Copyright (c) 2024 Ilya Lakhin (Илья Александрович Лахин).
  All rights reserved.
------------------------------------------------------------------------------->

# Code Formatters

Many programming language compilers and language support extensions of code
editors typically include code formatting programs. These programs take the
source code text as input and bring it to a canonical format according to the
code formatting rules.

Code formatting presents a challenging problem that must consider both canonical
formatting rules and the original intentions of the code author, such as
preserving empty lines between code fragments and retaining code comments on
their respective lines.

Lady Deirdre offers two tools to aid in implementing the code formatter:

1. The
   [ParseTree](https://docs.rs/lady-deirdre/2.0.0/lady_deirdre/syntax/struct.ParseTree.html)
   builder constructs a concrete parsing tree. Unlike abstract syntax trees, it
   intentionally preserves all original source code whitespaces and comments.
2. The
   [PrettyPrinter](https://docs.rs/lady-deirdre/2.0.0/lady_deirdre/format/struct.PrettyPrinter.html)
   tool automatically decides on splitting the text into lines and determines
   line indentation to ensure that the final lines are aligned according to the
   predefined maximum line length.

The parse tree builder utilizes the same syntax parser defined within the Node
derive macro. However, unlike the Document or ImmutableSyntaxTree parsers, this
parser preserves all tokens and trivia nodes, such as comments, in the concrete
parse tree. While the structure of the output concrete parse tree resembles the
syntax tree in terms of node structural nesting, the parse tree nodes include
all possibly omitted children of the syntax tree grammar, regardless of
capturing rules.

Nodes of the parse tree comprehensively cover the original source code without
gaps or overlaps. Consequently, the original source code text can be fully
reconstructed by traversing the tree. To simplify tree traversing, parse tree
nodes are owned by their parents.

The source code text is expected to be provided to the parse tree builder, and
the concrete parse tree is then used as input for the formatting tool. During
tree traversal, your program interprets the lexis and tree nesting of the
source code in accordance with your programming language formatting rules,
considering the concrete tree configuration. It then feeds the source code
tokens into the syntax-unaware Pretty Printer. The printer generates the final
output string.

The [Json Formatter](https://github.com/Eliah-Lakhin/lady-deirdre/tree/1f4ecdac2a1d8c73e6d94909fb0c7fcd04d31fc0/work/crates/examples/src/json_formatter)
example demonstrates the basic usage of both tools.
