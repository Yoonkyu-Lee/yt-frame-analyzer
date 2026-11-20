#include <stdio.h>

size_t hstrlen(const char *s) {
  const char *p = s;
  while (*p++);
  return (size_t)(p - s) - 1;
}

void *hmemcpy(void *s1, const void *s2, size_t n) {
  char *c1 = s1;
  const char *c2 = s2;
  while (n--) *c1++ = *c2++;

  return s1;
}

char *hstrcpy(char *dest, char *src) {
  char *p = dest;

  while ((*p++ = *src++));

  return dest;
}

char *hstrncpy(char *dest, char *src, size_t n) {
  char *p = dest;

  while (n--) *p++ = *src++;

  return dest;
}

char *hstrcat(char *s1, const char *s2) {
  char *p = s1;
  while (*p) p++;
  while ((*p++ = *s2++));

  return s1;
}

int hstrcmp(const char *s1, const char *s2) {
  while (*s1 && *s2 && (*s1 == *s2)) {
    s1++;
    s2++;
  }

  //return *s1 - *s2; // it can have negative
  return (unsigned char)*s1 - (unsigned char)*s2;
}

int main(void) {
  printf("%d\n", hstrcmp("hellO", "hello"));

  return 0;
}
