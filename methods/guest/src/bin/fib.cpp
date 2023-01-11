int fib(int x) {
  if(x > 1)
    return fib(x-1) + fib(x-2);
  else
    return x;
}
