# 使用方法

## 编译成静态库

```shell
cargo build --release
```

如果你的电脑上没有 cargo 的话，我也使用github的workflow功能编译了windows和linux的静态库，可以直接下载使用。

## 使用静态库

使用方法参考文件目录下的 main.c, 编译时添加链接选项-lseu_lex 即可。

lib文件夹下放的就是你下载的静态库（在github的release中获取），你可以将其放在你的项目目录下，然后在编译时指定路径，
例如使用 gcc 编译

```shell
gcc main.c -Llib/release -lseu_lex -lm
```
-lm 为链接 c 的数学库

在windows下我只使用过clang，使用方法如下

```shell
 clang main.c -Ltarget/release -lseu_lex -lWs2_32 -lUserenv -lNtDll -lBcrypt -lAdvapi32
```



## lex 文件规则

正则文法上，我只实现了 () | \* 这几个符号，样例 lex 文件如

```lex
DIGIT->(1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*
WHILE->while
ID->(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z|0|1|2|3|4|5|6|7|8|9)*
WHITESPACE-> |\t|\n
```

所示，每行一个规则，-> 左边是规则名，右边是正则表达式，可以使用\t 和\n 来表达转义

同一个字符串可以被多个规则匹配，优先级是按照规则出现的顺序来的，所以你可以把优先级高的规则放在前面。

程序会将 while 识别为 WHILE 而不是 ID
