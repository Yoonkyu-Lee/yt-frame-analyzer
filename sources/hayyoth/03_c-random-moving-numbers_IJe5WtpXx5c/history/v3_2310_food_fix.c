#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <string.h>

#define W 50
#define H 30

int state[H][W] = {0};
int next[H][W] = {0};


void render(void) {
  for (size_t j = 0; j < H; j++) {
    for (size_t i = 0; i < W; i++) {
      if (!state[j][i])
        printf("%2c", ' ');
      else {
        if (state[j][i] > 0)
          printf("%2d", state[j][i]);
        else
          printf("%2c", 'F');
      }
    }
    printf("\n");
  }
}

void init(void) {
  state[10][10] = 90;

  // < 0 is food
  state[2][3] = -4;
  state[3][3] = -2;
  state[1][3] = -1;
  state[9][3] = -7;

  memcpy(next, state, sizeof(int) * W * H);
}

const int moves[8][2] = {
  {-1, -1}, {-1, 0}, {-1, 1},
  {0, -1},           {0, 1},
  {1, -1}, {1, 0}, {1, 1}
};

void update(void) {
  for (size_t j = 0; j < H; j++) {
    for (size_t i = 0; i < W; i++) {
      if (state[j][i] > 0) {
        const int *move = moves[rand() % 8];
        int nj = j + move[0];
        int ni = i + move[1];

        if (nj >= 0 && nj < H && ni >= 0 && ni < W) {
          if (state[nj][ni] < 0) {
             // if meet food
            next[j][i]++;
          } else {
            next[j][i]--;
            next[nj][ni]++;
          }
        }
      }
    }
  }
  memcpy(state, next, sizeof(int) * W * H);
}

void clear_term(void) {
  usleep(300000);
  system("clear");
}

int main(void) {
  srand(time(NULL));
  init();

  while (1) {
    clear_term();
    render();
    update();
  }
  return 0;
}
