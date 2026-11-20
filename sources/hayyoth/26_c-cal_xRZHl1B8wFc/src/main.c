#include <stdio.h>
#include <time.h>
#include <stdlib.h>

int get_year(const struct tm *date) { return date->tm_year + 1900; }
int get_month(const struct tm *date) { return date->tm_mon + 1; }
int get_day(const struct tm *date) { return date->tm_mday; }

int get_year_tm(int year) { return year - 1900; }
int get_month_tm(int month) { return month - 1; }

int first_weekday(int year, int month) {
  struct tm date = {0};

  date.tm_year = get_year_tm(year);
  date.tm_mon = get_month_tm(month);
  date.tm_mday = 1;

  mktime(&date);

  return date.tm_wday;
}

int days_in_month(int year, int month) {
  struct tm date = {0};

  date.tm_year = get_year_tm(year);
  date.tm_mon = get_month_tm(month) + 1;
  date.tm_mday = 0;

  mktime(&date);

  return date.tm_mday;
}

void print_month(int year, int month, const struct tm *now) {
  int total_days = days_in_month(year, month);
  int first_day = first_weekday(year, month);

  int month_now = get_month(now);
  int year_now = get_year(now);
  int day_now = get_day(now);

  int is_current_month = year == year_now && month == month_now;

  printf("Su Mo Tu We Th Fr Sa\n");

  for (int i = 0; i < first_day; i++)
    printf("   ");

  for (int day = 1; day <= total_days; day++) {
    if (is_current_month && day == day_now)
      printf("%s%2d%s ", "\033[7m", day, "\033[0m");
    else
      printf("%2d ", day);

    if ((first_day + day) % 7 == 0 || day == total_days)
      printf("\n");
  }
  printf("\n");
}

void print_year(int year, const struct tm *now) {
  for (int month = 1; month <= 12; month++) {
    printf("-------- %2d --------\n", month);
    print_month(year, month, now);
  }
}

int main(int argc, char *argv[]) {
  int year = 0, month = 0;

  time_t now_t = time(NULL);
  struct tm *now = localtime(&now_t);

  switch (argc) {
    case 1:
      year = get_year(now);
      month = get_month(now);
      break;
    case 2:
      year = atoi(argv[1]);
      break;
    case 3:
      month = atoi(argv[1]);
      year = atoi(argv[2]);
      break;
  }

  if (month) {
    print_month(year, month, now);
  } else {
    print_year(year, now);
  }


  return 0;
}
