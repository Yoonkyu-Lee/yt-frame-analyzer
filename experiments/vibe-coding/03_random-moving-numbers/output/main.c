#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <math.h>
#include <signal.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <time.h>
#include <unistd.h>

#define MAX_WIDTH 240
#define MAX_HEIGHT 80
#define MAX_MOTES 360

typedef struct { float glow; unsigned char digit, hue; } Cell;
typedef struct { float x, y, speed, phase; unsigned char hue; } Mote;

static Cell cells[MAX_WIDTH * MAX_HEIGHT];
static Mote motes[MAX_MOTES];
static char output[MAX_WIDTH * MAX_HEIGHT * 32 + 8192];
static struct termios saved_terminal;
static bool raw_mode, screen_mode;
static volatile sig_atomic_t running = 1;
static uint32_t rng_state;

static uint32_t random_u32(void) {
    rng_state ^= rng_state << 13;
    rng_state ^= rng_state >> 17;
    rng_state ^= rng_state << 5;
    return rng_state;
}

static float random_unit(void) {
    return (float)(random_u32() >> 8) / 16777216.0f;
}

static double now(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (double)t.tv_sec + (double)t.tv_nsec / 1e9;
}

static void stop(int signal_number) {
    (void)signal_number;
    running = 0;
}

static void restore_terminal(void) {
    if (raw_mode) tcsetattr(STDIN_FILENO, TCSANOW, &saved_terminal);
    if (screen_mode) {
        fputs("\033[0m\033[?25h\033[?1049l", stdout);
        fflush(stdout);
    }
}

static void start_terminal(void) {
    screen_mode = isatty(STDOUT_FILENO);
    if (isatty(STDIN_FILENO) && tcgetattr(STDIN_FILENO, &saved_terminal) == 0) {
        struct termios t = saved_terminal;
        t.c_lflag &= (tcflag_t)~(ICANON | ECHO);
        t.c_cc[VMIN] = 0;
        t.c_cc[VTIME] = 0;
        raw_mode = tcsetattr(STDIN_FILENO, TCSANOW, &t) == 0;
    }
    if (screen_mode) fputs("\033[?1049h\033[?25l\033[2J", stdout);
}

static void dimensions(int *width, int *height) {
    struct winsize size;
    *width = 80;
    *height = 24;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &size) == 0) {
        if (size.ws_col) *width = size.ws_col;
        if (size.ws_row) *height = size.ws_row;
    }
    if (*width > MAX_WIDTH) *width = MAX_WIDTH;
    if (*height > MAX_HEIGHT) *height = MAX_HEIGHT;
}

static void reset_mote(Mote *m, int width, int height, bool scatter) {
    m->x = random_unit() * (float)width;
    m->y = scatter ? random_unit() * (float)height : -random_unit() * 8.0f;
    m->speed = 3.0f + random_unit() * 12.0f;
    m->phase = random_unit() * 6.2831853f;
    m->hue = (unsigned char)(random_u32() % 4);
}

static void reset_scene(int width, int height, int count) {
    memset(cells, 0, sizeof(cells));
    for (int i = 0; i < count; ++i) reset_mote(&motes[i], width, height, true);
}

static void advance(int width, int height, int count, float dt, double time) {
    float fade = expf(-dt * 2.0f);
    for (int i = 0; i < width * height; ++i) cells[i].glow *= fade;
    for (int i = 0; i < count; ++i) {
        Mote *m = &motes[i];
        float previous_y = m->y;
        m->x += sinf((float)time * 0.7f + m->phase) * dt * 2.5f;
        m->y += m->speed * dt;
        if (m->x < 0) m->x += (float)width;
        if (m->x >= (float)width) m->x -= (float)width;
        if (m->y >= (float)height) {
            reset_mote(m, width, height, false);
            continue;
        }
        /* Fill crossed rows so fast particles leave continuous trails. */
        int x = (int)m->x;
        int first_y = (int)floorf(previous_y);
        int last_y = (int)floorf(m->y);
        for (int y = first_y; y <= last_y; ++y) {
            if (y < 0 || y >= height) continue;
            Cell *c = &cells[y * width + x];
            c->glow = y == last_y ? 1.0f : 0.8f;
            c->digit = (unsigned char)('0' + random_u32() % 10);
            c->hue = m->hue;
        }
    }
    /* Occasional free-floating sparks shimmer between the falling streams. */
    if (random_unit() < dt * 18.0f) {
        Cell *c = &cells[random_u32() % (uint32_t)(width * height)];
        c->glow = 0.6f;
        c->digit = (unsigned char)('0' + random_u32() % 10);
        c->hue = (unsigned char)(random_u32() % 4);
    }
}

static char *label(char *p, const char *text, int width) {
    size_t length = strlen(text);
    if (length > (size_t)width) length = (size_t)width;
    memcpy(p, text, length);
    return p + length;
}

static bool render(int width, int rows, int height, bool color,
                   bool paused, float speed) {
    static const int palette[4][3] = {
        {23, 37, 86}, {22, 35, 120}, {54, 99, 183}, {58, 136, 222}
    };
    char *p = output;
    if (screen_mode) p += sprintf(p, "\033[H");
    if (rows >= 5) {
        if (color) p += sprintf(p, "\033[1;38;5;159m");
        p = label(p, "  N U M B E R S   /   a little terminal magic", width);
        if (screen_mode) p += sprintf(p, "\033[0m\033[K");
        else if (color) p += sprintf(p, "\033[0m");
        *p++ = '\n';
        if (screen_mode) p += sprintf(p, "\033[K");
        *p++ = '\n';
    }
    for (int y = 0; y < height; ++y) {
        int last_color = -1;
        for (int x = 0; x < width; ++x) {
            Cell *c = &cells[y * width + x];
            if (c->glow < 0.08f) { *p++ = ' '; continue; }
            int shade = c->glow > 0.83f ? 231 :
                palette[c->hue][c->glow > 0.55f ? 2 : c->glow > 0.25f ? 1 : 0];
            if (color && shade != last_color) {
                p += sprintf(p, "\033[38;5;%dm", shade);
                last_color = shade;
            }
            *p++ = (char)c->digit;
        }
        if (color) p += sprintf(p, "\033[0m");
        if (screen_mode) p += sprintf(p, "\033[K");
        if (y + 1 < height || rows >= 5) *p++ = '\n';
    }
    if (rows >= 5) {
        char status[160];
        snprintf(status, sizeof(status),
                 "  %s  %.2fx   [space] pause  [+/-] speed  [r] remix  [q] quit",
                 paused ? "PAUSED" : "FLOWING", (double)speed);
        if (color) p += sprintf(p, "\033[38;5;109m");
        p = label(p, status, width);
        if (screen_mode) p += sprintf(p, "\033[0m\033[K");
        else if (color) p += sprintf(p, "\033[0m");
    }
    if (!screen_mode) *p++ = '\n';
    size_t length = (size_t)(p - output);
    return fwrite(output, 1, length, stdout) == length && fflush(stdout) == 0;
}

static bool number(const char *text, unsigned long *value) {
    char *end;
    errno = 0;
    if (!*text || *text == '-') return false;
    *value = strtoul(text, &end, 10);
    return !errno && !*end;
}

int main(int argc, char **argv) {
    unsigned long frames = 0, seed = (unsigned long)time(NULL) ^ (unsigned long)getpid();
    bool color = isatty(STDOUT_FILENO) && getenv("NO_COLOR") == NULL;
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "--help") || !strcmp(argv[i], "-h")) {
            puts("Usage: ./main [--no-color] [--frames N] [--seed N]\n"
                 "A glowing shower of drifting random digits.\n\n"
                 "Space: pause/resume   +/-: speed   r: remix   q/Esc: quit\n"
                 "--frames N   Stop after N frames (N must be positive).\n"
                 "--seed N     Repeat a random sequence with this 32-bit seed.\n"
                 "--no-color   Disable colors (also honors NO_COLOR).\n\n"
                 "Redirected output defaults to one plain-text snapshot.");
            return EXIT_SUCCESS;
        }
        if (!strcmp(argv[i], "--no-color")) { color = false; continue; }
        bool is_frames = !strcmp(argv[i], "--frames");
        bool is_seed = !strcmp(argv[i], "--seed");
        unsigned long value;
        if ((!is_frames && !is_seed) || i + 1 >= argc ||
            !number(argv[++i], &value) || (is_frames && !value) ||
            (is_seed && value > UINT32_MAX)) {
            fprintf(stderr, "Invalid option or value. Try ./main --help\n");
            return EXIT_FAILURE;
        }
        if (is_frames) frames = value;
        else seed = value;
    }
    rng_state = (uint32_t)seed;
    if (!rng_state) rng_state = 0x9e3779b9u;
    if (!isatty(STDOUT_FILENO) && !frames) frames = 1;

    struct sigaction action;
    memset(&action, 0, sizeof(action));
    action.sa_handler = stop;
    sigemptyset(&action.sa_mask);
    sigaction(SIGINT, &action, NULL);
    sigaction(SIGTERM, &action, NULL);
    sigaction(SIGHUP, &action, NULL);
    atexit(restore_terminal);
    start_terminal();

    int width = 0, rows = 0, height = 0, count = 0;
    bool paused = false, success = true;
    float speed = 1.0f;
    double previous = now(), simulation_time = 0;
    unsigned long frame = 0;
    while (running && (!frames || frame < frames)) {
        double frame_start = now();
        int new_width, new_rows;
        dimensions(&new_width, &new_rows);
        if (width != new_width || rows != new_rows) {
            width = new_width;
            rows = new_rows;
            height = rows >= 5 ? rows - 3 : rows;
            count = width * 3 / 4 + 8;
            if (count > MAX_MOTES) count = MAX_MOTES;
            reset_scene(width, height, count);
            if (screen_mode) fputs("\033[2J", stdout);
        }
        if (raw_mode) {
            unsigned char key;
            while (read(STDIN_FILENO, &key, 1) == 1) {
                if (key == 'q' || key == 'Q' || key == 27) running = 0;
                else if (key == ' ') paused = !paused;
                else if (key == '+' || key == '=') speed = fminf(4.0f, speed + 0.25f);
                else if (key == '-') speed = fmaxf(0.25f, speed - 0.25f);
                else if (key == 'r' || key == 'R') reset_scene(width, height, count);
            }
        }
        if (!running) break;
        float dt = screen_mode ? (float)fmin(frame_start - previous, 0.1) : 1.0f / 30.0f;
        if (frame == 0) dt = 1.0f / 30.0f;
        previous = frame_start;
        if (!paused) {
            simulation_time += (double)(dt * speed);
            advance(width, height, count, dt * speed, simulation_time);
        }
        if (!render(width, rows, height, color, paused, speed)) {
            success = false;
            break;
        }
        ++frame;
        if (screen_mode && (!frames || frame < frames)) {
            double remaining = 1.0 / 30.0 - (now() - frame_start);
            if (remaining > 0) {
                struct timespec delay = {0, (long)(remaining * 1e9)};
                nanosleep(&delay, NULL);
            }
        }
    }
    return success ? EXIT_SUCCESS : EXIT_FAILURE;
}
