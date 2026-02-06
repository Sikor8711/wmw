function fizzBuzz(n) {
  // Write your code here
  for (let i; i < n; i++) {
    if (n % 3 == 0 && n % 5 == 0) {
      console.log("FizzBuzz");
    } else if (n % 3 == 0) {
      console.log("Fizz");
    } else if (n % 5 == 0) {
      console.log("Buzz");
    } else {
      console.log(i + 1);
    }
  }
}
