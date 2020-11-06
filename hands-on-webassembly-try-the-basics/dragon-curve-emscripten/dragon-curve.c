#ifndef DRAGON_CURVE
#define DRAGON_CURVE

// Helper function for generating x,y coordinates from "turns"
int sign(int x) { return (x % 2) * (2 - (x % 4)); }

// Helper function to generate "turns"
// Adapted from https://en.wikipedia.org/wiki/Dragon_curve#[Un]folding_the_dragon
int getTurn(int n)
{
  int turnFlag = (((n + 1) & -(n + 1)) << 1) & (n + 1);
  return turnFlag != 0 ? -1 : 1; // -1 for left turn, 1 for right
}

// fills source with x and y points [x0, y0, x1, y1,...]
// first argument is a pointer to the first element of the array
// that will be provided at runtime.
void dragonCurve(double source[], int size, int len, double x0, double y0)
{
  int angle = 0;
  double x = x0, y = y0;
  for (int i = 0; i < size; i++)
  {
    int turn = getTurn(i);
    angle = angle + turn;
    x = x - len * sign(angle);
    y = y - len * sign(angle + 1);
    source[2 * i] = x;
    source[2 * i + 1] = y;
  }
}
#endif
