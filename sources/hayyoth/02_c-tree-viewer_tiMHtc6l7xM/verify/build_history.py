"""Usage: python verify/build_history.py <video-folder>
Rebuild src/main.c at every moment vim printed a save checksum, then compare to the on-screen "NL, NB written"."""
import os, sys
root = sys.argv[1]
final = open(os.path.join(root, "src", "main.c"), newline="").read()

def sub(s, *pairs):
    for a, b in pairs:
        assert a in s, a
        s = s.replace(a, b)
    return s

v0253 = r'''#include <stdio.h>

int main(int argc, char **argv) {
  const char *path = "."; // default path is current dir

  if (argc > 1) {
    path = argv[1];
  }

  printf("path %s\n", path);


  return 0;
}
'''
v0629 = r'''#include <stdio.h>
#include <dirent.h>

int walk(const char *path) {
  DIR *dir = opendir(path);
  if (!dir) {
    perror("opendir");
    return 1;
  }

  struct dirent *dp;
  while ((dp = readdir(dir)) != NULL) {
    const char *name = dp->d_name;
    printf("%s\n", name);
  }

  closedir(dir);
  return 0;
}

int main(int argc, char **argv) {
  const char *path = "."; // default path is current dir

  if (argc > 1) {
    path = argv[1];
  }

  int err = walk(path);

  return err;
}
'''
v1909 = sub(final,
    ("  if (indent)\n    printf(\"%*c\", indent * 2, ' ');\n\n", "  printf(\"%*c\", indent * 2, ' ');\n"))
v1813 = sub(v1909,
    ("int walk(const char *path, int indent)", "int walk(const char *path)"),
    ("print_file(name, 1, indent)", "print_file(name, 1, 3)"),
    ("print_file(name, 0, indent)", "print_file(name, 0, 3)"),
    ("walk(sub_path, indent + 1)", "walk(sub_path)"),
    ("walk(path, 0)", "walk(path)"))
v1803 = sub(v1813, ('"%*c"', '"%.*c"'))
v1640 = sub(v1813,
    ("void print_file(const char *name, int is_dir, int indent) {\n  printf(\"%*c\", indent * 2, ' ');\n",
     "void print_file(const char *name, int is_dir) {\n"),
    ("print_file(name, 1, 3)", "print_file(name, 1)"),
    ("print_file(name, 0, 3)", "print_file(name, 0)"))
REC = ("\n\n      // walk to sub path\n      size_t len = strlen(path) + strlen(name) + 2;\n"
       "      char sub_path[len];\n      snprintf(sub_path, len, \"%s/%s\", path, name);\n\n      walk(sub_path);")
v1415 = sub(v1640, (REC, ""))
v1359 = sub(v1415, ("#include <string.h>\n", ""))

versions = [  # (name, text, on-screen lines, on-screen bytes)
    ("v1_0253_args.c",        v0253, 14, 200),
    ("v2_0629_readdir.c",     v0629, 31, 488),
    ("v3_1359_ansi.c",        v1359, 47, 807),
    ("v4_1415_string_h.c",    v1415, 48, 827),
    ("v5_1640_recursion.c",   v1640, 55, 1013),
    ("v6_1803_indent_bug.c",  v1803, 56, 1066),
    ("v7_1813_indent_fix.c",  v1813, 56, 1065),
    ("v8_1909_indent_param.c",v1909, 56, 1102),
    ("v9_1929_final.c",       final, 58, 1119),
]
print(f"{'version':26} {'lines':>9} {'bytes':>11}  delta")
for name, s, L, B in versions:
    open(os.path.join(root, "history", name), "w", newline="").write(s)
    l, b = s.count("\n"), len(s.encode())
    print(f"{name:26} {l:>3}/{L:<3} {'OK' if l==L else '!!'} {b:>4}/{B:<4}  {b-B:+d}")
