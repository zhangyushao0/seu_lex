# 使用方法

## 编译成静态库

```shell
cargo build --release
```

如果你的电脑上没有 cargo 的话，我也使用 github 的 workflow 功能编译了 windows 和 linux 的静态库，可以直接下载使用。

## 使用静态库

使用方法参考文件目录下的 main.c, 编译时添加链接选项-lseu_lex 即可。

lib 文件夹下放的就是你下载的静态库（在 github 的 release 中获取），你可以将其放在你的项目目录下，然后在编译时指定路径，
例如使用 gcc 编译

```shell
gcc main.c -Llib/release -lseu_lex -lm
```

-lm 为链接 c 的数学库

在 windows 下我只使用过 clang，使用方法如下

```shell
 clang main.c -Ltarget/release -lseu_lex -lWs2_32 -lUserenv -lNtDll -lBcrypt -lAdvapi32
```

## lex 文件规则

正则文法上，我实现了 () | \* ? + [ ] 这几个符号，样例 lex 文件如

```lex
DIGIT->[1-9][0-9]*
WHILE->while
IDENTIFIER->[a-z]([a-z]|[0-9])*
WHITESPACE-> |\t|\n|\r
LEFT_BRACE->\[
RIGHT_BRACE->\]
```

所示，每行一个规则，-> 左边是规则名，右边是正则表达式，可以使用\t 和\n 来表达转义

同一个字符串可以被多个规则匹配，优先级是按照规则出现的顺序来的，所以你可以把优先级高的规则放在前面。

程序会将 while 识别为 WHILE 而不是 ID

支持转义字符
