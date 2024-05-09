#include <stdio.h>

extern void lexer_init(const char *lex_path, const char *src_path);
extern size_t lexer_get_tokens_count();
extern const char *lexer_get_token_name(size_t index);
extern const char *lexer_get_token_value(size_t index);

int main() {
  lexer_init("/home/zys/repo/seu_lex/c99_modified.l",
             "/home/zys/repo/seu_lex/src.txt");
  size_t tokens_count = lexer_get_tokens_count();
  for (size_t i = 0; i < tokens_count; i++) {
    printf("Token: %s, Value: %s\n", lexer_get_token_name(i),
           lexer_get_token_value(i));
  }
  return 0;
}