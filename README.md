## Microbenchmark

Benchmarking copy vs ref for a Vector4.

```
test bref_f32 ... bench:   2,736,055 ns/iter (+/- 364,885)
test bref_f64 ... bench:   4,872,076 ns/iter (+/- 436,928)
test copy_f32 ... bench:   2,708,568 ns/iter (+/- 31,162)
test copy_f64 ... bench:   4,890,014 ns/iter (+/- 553,050)
```
