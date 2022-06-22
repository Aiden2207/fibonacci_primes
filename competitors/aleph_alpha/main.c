#include <pari/pari.h>

void f(long n)
{
  GEN a = fibo(n);
  if (ispseudoprime(a, 16))
    pari_printf("%d,%Ps\n", n, a);
  return;
}

int main()
{
  pari_init(8000000, 500000);

  f(3);
  f(4);

  long n;
  forprime_t iter;
  u_forprime_init(&iter, 5, ULONG_MAX);

  while (n = u_forprime_next(&iter))
    f(n);

  return 0;
}