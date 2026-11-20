#include <stdio.h>

#define W 50
#define H 30

int state[H][W] = {0};

void render(void) {
  for (size_t j = 0; j < H; j++) {
    for (size_t i = 0; i < W; i++) {
      printf("%d", state[j][i]);
    }
    printf("\n");
  }
}

void init(void) {
  state[10][10] = 1;
}

int main(void) {
  init();
  render();
  return 0;
}
