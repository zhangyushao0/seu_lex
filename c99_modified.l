AUTO->auto
BOOL->_Bool
BREAK->break
CASE->case
CHAR->char
COMPLEX->_Complex
CONST->const
CONTINUE->continue
DEFAULT->default
DO->do
DOUBLE->double
ELSE->else
ENUM->enum
EXTERN->extern
FLOAT->float
FOR->for
GOTO->goto
IF->if
IMAGINARY->_Imaginary
INLINE->inline
INT->int
LONG->long
REGISTER->register
RESTRICT->restrict
RETURN->return
SHORT->short
SIGNED->signed
SIZEOF->sizeof
STATIC->static
STRUCT->struct
SWITCH->switch
TYPEDEF->typedef
UNION->union
UNSIGNED->unsigned
VOID->void
VOLATILE->volatile
WHILE->while

ELLIPSIS->\.\.\.
RIGHT_ASSIGN->>>=
LEFT_ASSIGN-><<=
ADD_ASSIGN->\+=
SUB_ASSIGN->\-=
MUL_ASSIGN->\*=
DIV_ASSIGN->/=
MOD_ASSIGN->%=
AND_ASSIGN->&=
XOR_ASSIGN->^=
OR_ASSIGN->\|=
RIGHT_OP->>>
LEFT_OP-><<
INC_OP->\+\+
DEC_OP->\-\-

AND_OP->&&
OR_OP->\|\|
LE_OP-><=
GE_OP->>=
EQ_OP->==
NE_OP->!=

;->;
{->{
}->}
,->,
:->:
=->=
(->\(
)->\)
[->\[
]->\]
.->\.
&->&
!->!
~->~
-->\-
+->\+
*->\*
/->/
%->%
<-><
>->>
^->^
|->\|
?->\?

IDENTIFIER->([a-z]|[A-Z]|_)([a-z]|[A-Z]|[0-9]|_)*

STRING_LITERAL->"(!|[#-~]| |\t)*"

CONSTANT->0(x|X)([a-f]|[A-F]|[0-9])+((u|U)|(u|U)?(l|L|ll|LL)|(l|L|ll|LL)(u|U))?
CONSTANT->0[0-7]*((u|U)|(u|U)?(l|L|ll|LL)|(l|L|ll|LL)(u|U))?
CONSTANT->[1-9][0-9]*((u|U)|(u|U)?(l|L|ll|LL)|(l|L|ll|LL)(u|U))?

CONSTANT->[0-9]+((E|e)(\+|\-)?[0-9]+)(f|F|l|L)?
CONSTANT->[0-9]*\.[0-9]+((E|e)(\+|\-)?[0-9]+)?(f|F|l|L)?
CONSTANT->[0-9]+\.[0-9]*((E|e)(\+|\-)?[0-9]+)?(f|F|l|L)?
CONSTANT->0(x|X)([a-f]|[A-F]|[0-9])+((P|p)(\+|\-)?[0-9]+)(f|F|l|L)?
CONSTANT->0(x|X)([a-f]|[A-F]|[0-9])*\.([a-f]|[A-F]|[0-9])+((P|p)(\+|\-)?[0-9]+)?(f|F|l|L)?
CONSTANT->0(x|X)([a-f]|[A-F]|[0-9])+\.([a-f]|[A-F]|[0-9])*((P|p)(\+|\-)?[0-9]+)?(f|F|l|L)?

COMMENT->//(!|[#-~]| |\t)*\n

COMMENT->#(!|[#-~]| |\t)*\n


WHITESPACE-> |\t|\n|\r

