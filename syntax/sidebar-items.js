initSidebarItems({"enum":[["Direction",""],["NodeOrToken",""],["SyntaxKind","The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."],["TokenAtOffset","There might be zero, one or two leaves at a given offset."],["WalkEvent","`WalkEvent` describes tree walking process."]],"macro":[["T",""],["match_ast","Matches a `SyntaxNode` against an `ast` type."]],"mod":[["algo","Collection of assorted algorithms for syntax trees."],["ast","Abstract Syntax Tree, layered on top of untyped `SyntaxNode`s"],["display","This module contains utilities for rendering syntax nodes into a string representing their signature."],["hacks","Things which exist to solve practial issues, but which shouldn’t exist."],["ted","Primitive tree editor, ed for trees."],["utils","A set of utils methods to reuse on other abstraction levels"]],"struct":[["AstPtr","Like `SyntaxNodePtr`, but remembers the type of node."],["GreenNode","Internal node in the immutable tree. It has other nodes and tokens as children."],["Parse","`Parse` is the result of the parsing: a syntax tree and a collection of errors."],["SmolStr","A `SmolStr` is a string type that has the following properties:"],["SourceFile",""],["SyntaxError","Represents the result of unsuccessful tokenization, parsing or tree validation."],["SyntaxText",""],["SyntaxTreeBuilder",""],["TextRange","A range in text, represented as a pair of [`TextSize`][struct@TextSize]."],["TextSize","A measure of text length. Also, equivalently, an index into text."],["TokenText",""]],"type":[["PreorderWithTokens",""],["SyntaxElement",""],["SyntaxElementChildren",""],["SyntaxNode",""],["SyntaxNodeChildren",""],["SyntaxNodePtr","A “pointer” to a [`SyntaxNode`], via location in the source code."],["SyntaxToken",""]]});