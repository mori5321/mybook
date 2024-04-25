# Bit Operation Pattern Useful in Algorithm Competion

## Left Bit Shift (<<) for 2 ^ N

```c++
// This is 2^N
1 << N

1 << 2 <=> 000100 <=> 4  (2^2) 
1 << 3 <=> 001000 <=> 8  (2^3)
1 << 4 <=> 010000 <=> 16 (2^4)
```


## intToBitVector (Right bit shift to identify the value of a particular digit in a bit)

Make `int` converted into `vector<int>`.
This is used in the following `Bit Blute-force Search`.

```c++
vector<int> intToBitVector(int target, int bitDigit) {
  vector<int> S(N);

  for (int i = 0; i < bitDigit; ++i) {
    S[i] = (target >> i) & 1
  };


  return S;
} 

intToBitVector(5); 
// => vector<int>({ 1, 0, 1 })

intToBitVector(36); 
// => vector<int>({ 1, 0, 0, 1, 0, 0 })
```

## Q. How can we identify the value of a particular digit in a bit?
A. Right Bit Shift & 1.


```c++
Ex1) -------------------
target number = 32

(32 >> 5) & 1 <=> 000001 & 000001 <=> 000001 (return 1!)
(32 >> 4) & 1 <=> 000010 & 000001 <=> 000000 (return 0)
(32 >> 3) & 1 <=> 000100 & 000001 <=> 000000 (return 0)
(32 >> 2) & 1 <=> 001000 & 000001 <=> 000000 (return 0)
(32 >> 1) & 1 <=> 010000 & 000001 <=> 000000 (return 0)
(32 >> 0) & 1 <=> 100000 & 000001 <=> 000000 (return 0)

You can identify Digit 6 should be 1 in a bit row of `32`.
The bit row is `100000`. The function returns `vector<int>({ 1, 0, 0, 0, 0, 0})` 

Ex2) -------------------

target number = 36

(36 >> 5) & 1 <=> 000001 & 000001 <=> 000001 (return 1!)
(36 >> 4) & 1 <=> 000010 & 000001 <=> 000000 (return 0)
(36 >> 3) & 1 <=> 000100 & 000001 <=> 000000 (return 0)
(36 >> 2) & 1 <=> 001001 & 000001 <=> 000001 (return 1!)
(36 >> 1) & 1 <=> 010010 & 000001 <=> 000000 (return 0)
(36 >> 0) & 1 <=> 100100 & 000001 <=> 000000 (return 0)

You can identify Digit 6 and Digit 3 should be 1 in a bit row of `36`.
The bit row is `100100`.  The function returns `vector<int>({ 1, 0, 0, 1, 0, 0})`
```


## Bit Blute-force Search

This is a code pattern to do Blute-force Search for N Digit Bit.  
It uses Left Bit Shift & Right Bit Shift.

```c++
int bitDigit = 8;

// Blute-force search for 00000000 ~ 11111111
for (int bit = 0; bit < (1 << bitDigit); ++bit) {
  vector<int> = intToBitVector(bit, bitDigit)

  // Do Something for your search...
}
```
