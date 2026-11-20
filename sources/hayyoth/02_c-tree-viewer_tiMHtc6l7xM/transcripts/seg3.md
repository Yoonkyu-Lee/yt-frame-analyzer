# Segment 3 (06:35 - 07:40)

## Timeline
- [06:35] Mission-Control-style desktop view (green background, "Desktop" label). Terminal window (left, 2 tabs: "~/lab — vim README.md" active, "~/lab — -zsh") shows vim with main.c, 31 lines, status line `"main.c" 31L, 488B written` (normal mode, cursor on the `;` at end of line 14). Firefox window (right, highlighted, tooltip "The Open Group Base Specifications Issue 8") at pubs.opengroup.org/onlinepubs/9799919799/ showing the opendir() EXAMPLES section (Find And Open a File sample, with `dp->d_name` highlighted).
- [06:36-06:37] Browser brought to front (zoomed in). Reading opendir() example: fdopendir/openat/fstat sample; `closedir(d); // note this implicitly closes dfd`, then APPLICATION USAGE, RATIONALE. Webcam overlay visible (ignore). Terminal behind unchanged.
- [06:39] Scrolled up to "Find And Open a File" sample start (includes list: stdio, dirent, fcntl, sys/stat, stdint, stdlib, unistd; `int main(int argc, char *argv[])`).
- [06:40] Scrolled to "EXAMPLES / Open a Directory Stream" fragment: `DIR *dir; struct dirent *dp; if ((dir = opendir(".")) == NULL) { perror ("Cannot open ."); exit (1); } while ((dp = readdir (dir)) != NULL) {`.
- [06:41] Scrolled back down, the fdopendir example again (`if (dp->d_name[0] == '.') continue;` highlighted).
- [06:42-06:43] Scrolls past APPLICATION USAGE, RATIONALE to FUTURE DIRECTIONS / SEE ALSO (closedir, dirfd, fstatat, open, posix_getdents, readdir, rewinddir...), CHANGE HISTORY, Issue 6/7/8.
- [06:44] Scrolled to ERRORS section of opendir (EBADF, ENOTDIR, EACCES, ELOOP, ENAMETOOLONG, ENOENT, EMFILE, ENFILE).
- [06:45-06:47] Scrolled back to top of opendir page: SYNOPSIS `DIR *fdopendir(int fd); DIR *opendir(const char *dirname);`, DESCRIPTION, RETURN VALUE.
- [06:48-06:50] Browser window drifts/moves slightly rightwards (dragging); vim main.c visible at left unchanged.
- [06:52] Typed in the INDEX keyword box: `read` (autocomplete dropdown "struct dirent", "dirent").
- [06:54] Searched `readdir`: result "readdir, readdir_r — read a directory".
- [06:55] readdir page: NAME, SYNOPSIS `#include <dirent.h>` `struct dirent *readdir(DIR *dirp);` and (OB) `int readdir_r(DIR *restrict dirp, struct dirent *restrict entry, struct dirent **restrict result);` DESCRIPTION.
- [06:56-06:58] Scrolls through readdir DESCRIPTION (dirent struct, "shall not return entries containing empty names", errno note: "Applications wishing to check for error situations should set errno to zero before calling readdir()..."), RETURN VALUE.
- [06:59] Browser window being dragged left (covers more of terminal).
- [07:00] ERRORS: EOVERFLOW, ENOMEM; may fail: EBADF, ENOENT. Then EXAMPLES start.
- [07:01-07:04] EXAMPLES: `static void lookup(const char *arg)` sample with opendir("."), `errno = 0; if ((dp = readdir(dirp)) != NULL) { if (strcmp(dp->d_name, arg) != 0) continue; (void) printf("found %s\n", arg); (void) closedir(dirp); return; } } while (dp != NULL); if (errno != 0) perror("error reading directory"); else (void) printf("failed to find %s\n", arg); (void) closedir(dirp); return;`.
- [07:05-07:06] APPLICATION USAGE / RATIONALE of readdir (errno zero before call...), FUTURE DIRECTIONS (readdir_r may be removed), SEE ALSO, CHANGE HISTORY.
- [07:07-07:08] Bottom of page ("End of informative text").
- [07:10-07:11] Typed `dirent` into search box (dropdown shows "struct dirent", "dirent").
- [07:13] Result: "dirent.h — format of directory entries".
- [07:14] dirent.h page: NAME, SYNOPSIS `#include <dirent.h>`, DESCRIPTION: DIR type, struct dirent members `ino_t d_ino; char d_name[]`, struct posix_dent members `ino_t d_ino; reclen_t d_reclen; unsigned char d_type; char d_name[]`.
- [07:17] Scrolled to DT_* constants: DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, ...
- [07:18-07:19] Browser window shifting right/left (dragging); terminal behind shows main.c lines 1-31 (unchanged).
- [07:24] Mouse at `ino_t`/d_type area; browser window moved.
- [07:29] `d_type` highlighted (double-click/selected) in posix_dent table; DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, DT_SOCK, DT_UNKNOWN listed ("Unknown file type.").
- [07:30] Switched back to terminal (browser hidden). vim in INSERT mode; new blank line 15 added after printf, cursor on line 15 (empty, indented, auto-indent). Closing `}` of while now at line 16; file now 32 lines.
- [07:39] Typed `if` on line 16 (a blank line 15 above kept); file now 33 lines.
- [07:40] Line 16: `    if (dp->d_type ` (typing in progress, cursor after trailing space). Mission-Control-style view again (switching to browser, DT_DIR "Directory." highlighted in the browser at left).

## File states
### main.c (shown in tab titled "vim README.md"; status says "main.c")
Version at 06:35 (31 lines; `"main.c" 31L, 488B written`):
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3|
 4| int walk(const char *path) {
 5|   DIR *dir = opendir(path);
 6|   if (!dir) {
 7|     perror("opendir");
 8|     return 1;
 9|   }
10|
11|   struct dirent *dp;
12|   while ((dp = readdir(dir)) != NULL) {
13|     const char *name = dp->d_name;
14|     printf("%s\n", name);
15|   }
16|
17|   closedir(dir);
18|   return 0;
19| }
20|
21| int main(int argc, char **argv) {
22|   const char *path = "."; // default path is current dir
23|
24|   if (argc > 1) {
25|     path = argv[1];
26|   }
27|
28|   int err = walk(path);
29|
30|   return err;
31| }
```
Version at 07:40 (end of segment, 33 lines, INSERT mode, in progress):
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3|
 4| int walk(const char *path) {
 5|   DIR *dir = opendir(path);
 6|   if (!dir) {
 7|     perror("opendir");
 8|     return 1;
 9|   }
10|
11|   struct dirent *dp;
12|   while ((dp = readdir(dir)) != NULL) {
13|     const char *name = dp->d_name;
14|     printf("%s\n", name);
15|
16|     if (dp->d_type ⟨cursor; incomplete, nothing after trailing space⟩
17|   }
18|
19|   closedir(dir);
20|   return 0;
21| }
22|
23| int main(int argc, char **argv) {
24|   const char *path = "."; // default path is current dir
25|
26|   if (argc > 1) {
27|     path = argv[1];
28|   }
29|
30|   int err = walk(path);
31|
32|   return err;
33| }
```
Indent is 2 spaces. Intermediate: 07:30 line 16 = `    if` ; 07:29 (32 lines) line 15 blank with cursor.

## Shell & outputs
No shell commands in this segment (zsh tab never opened). Only vim status line `"main.c" 31L, 488B written` (06:35 - 07:19) and `-- INSERT --` (07:29 onward).

## Observations
- Programmer is consulting the POSIX spec (Issue 8, pubs.opengroup.org) for opendir, readdir, and dirent.h; specifically looks at the `d_type` field and `DT_DIR` constant (selected `d_type` at 07:29, `Directory.` DT_DIR highlighted at 07:40), clearly to add a directory check (`if (dp->d_type == DT_DIR)`) for recursion in the tree walker.
- The walk function currently only lists names (no skipping of "." / "..", no indentation yet); a blank line left after printf for the new code.
- Style: 2-space indent, K&R braces on same line, `if (!dir)` null check with perror and `return 1`, `walk` returns int error code, main passes it as exit code, default path "." with an inline `//` comment, declare variables at point of use (C99), only `<stdio.h>` and `<dirent.h>` included.
- Reads spec examples (readdir errno handling, `dp->d_name[0] == '.'` skip idiom) but hasn't incorporated them yet.
- Tab title says "vim README.md" yet the buffer is main.c (title likely stale from earlier in the video).
