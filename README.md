`cargo download`

then
`RUSTFLAGS='-C target-cpu=native' cargo time`

Benchmarks
-----

On a laptop with a Intel Core i7-11850H processor.

*Year 2024*

| Day   | Solution | Benchmark | threads | simd | unsafe |
|:--------:|:--------:|:------:|:--------:|:--------:|:------:|
| [1](https://adventofcode.com/2024/day/1) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day01.rs) | 10 μs | | | ✓|
| [2](https://adventofcode.com/2024/day/2) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day02.rs) | 25 μs | | | |
| [3](https://adventofcode.com/2024/day/3) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day03.rs) | 11 μs | | | |
| [4](https://adventofcode.com/2024/day/4) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day04.rs) | 2 μs | | ✓| |
| [5](https://adventofcode.com/2024/day/5) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day05.rs) | 15 μs | | | |
| [6](https://adventofcode.com/2024/day/6) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day06.rs) | 374 μs |✓ | | |
| [7](https://adventofcode.com/2024/day/7) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day07.rs) | 168 μs | | | |
| [8](https://adventofcode.com/2024/day/8) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day08.rs) | 7 μs | | | |  
| [9](https://adventofcode.com/2024/day/9) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day09.rs) | 128 μs | | | |
| [10](https://adventofcode.com/2024/day/10) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day10.rs) | 15 μs | | | |
| [11](https://adventofcode.com/2024/day/11) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day11.rs) | 193 μs | | | |
| [12](https://adventofcode.com/2024/day/12) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day12.rs) | 212 μs | | | |
| [13](https://adventofcode.com/2024/day/13) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day13.rs) | 11 μs | | | |
| [14](https://adventofcode.com/2024/day/14) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day14.rs) | 40 μs | | | |
| [15](https://adventofcode.com/2024/day/15) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day15.rs) | 376 μs | | | |
| [16](https://adventofcode.com/2024/day/16) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day16.rs) | 101 μs | | | |
| [17](https://adventofcode.com/2024/day/17) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day17.rs) | 1.7 μs | | | |
| [18](https://adventofcode.com/2024/day/18) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day18.rs) | 30 μs | | | | 
| [19](https://adventofcode.com/2024/day/19) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day19.rs) | 137 μs | | | |
| [20](https://adventofcode.com/2024/day/20) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day20.rs) | 624 μs |✓| | |
| [21](https://adventofcode.com/2024/day/21) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day21.rs) | 0.08 μs | | | |
| [22](https://adventofcode.com/2024/day/22) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day22.rs) | 1245 μs |✓ | | |
| [23](https://adventofcode.com/2024/day/23) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day23.rs) | 49 μs | | | |
| [24](https://adventofcode.com/2024/day/24) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day24.rs) | 9 μs | | | |
| [25](https://adventofcode.com/2024/day/25) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2024/day25.rs) | 2 μs | |✓ | |
 | Total |     | 3.7 ms | | |  |

*Year 2023*

| Day   | Solution | Benchmark | threads | simd | unsafe |
|:--------:|:--------:|:------:|:--------:|:--------:|:------:|
| [1](https://adventofcode.com/2023/day/1) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day01.rs) | 56 μs | | | |
| [2](https://adventofcode.com/2023/day/2) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day02.rs) | 2 μs | | | |
| [3](https://adventofcode.com/2023/day/3) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day03.rs) | 69 μs | | | |
| [4](https://adventofcode.com/2023/day/4) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day04.rs) | 22 μs | | | |
| [5](https://adventofcode.com/2023/day/5) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day05.rs) | 15 μs | | | |
| [6](https://adventofcode.com/2023/day/6) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day06.rs) | 80 ns | | | |
| [7](https://adventofcode.com/2023/day/7) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day07.rs) | 51 μs | | | |
| [8](https://adventofcode.com/2023/day/8) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day08.rs) | 9 μs | | | |
| [9](https://adventofcode.com/2023/day/9) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day09.rs) | 16 μs | | | |
| [10](https://adventofcode.com/2023/day/10) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day10.rs) | 31 μs | | | |
| [11](https://adventofcode.com/2023/day/11) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day11.rs) | 13 μs | | | |
| [12](https://adventofcode.com/2023/day/12) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day12.rs) | 923 μs |✓ | | |
| [13](https://adventofcode.com/2023/day/13) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day13.rs) | 60 μs | | | |
| [14](https://adventofcode.com/2023/day/14) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day14.rs) | 1684 μs | | | |
| [15](https://adventofcode.com/2023/day/15) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day15.rs) | 143 μs | | | |
| [16](https://adventofcode.com/2023/day/16) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day16.rs) | 1127 μs |✓ | | |
| [17](https://adventofcode.com/2023/day/17) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day17.rs) | 2193 μs | | | |
| [18](https://adventofcode.com/2023/day/18) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day18.rs) | 29 μs | | | |
| [19](https://adventofcode.com/2023/day/19) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day19.rs) | 113 μs | | |  |
| [20](https://adventofcode.com/2023/day/20) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day20.rs) | 10 μs | | | |
| [21](https://adventofcode.com/2023/day/21) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day21.rs) | 218 μs | | | |
| [22](https://adventofcode.com/2023/day/22) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day22.rs) | 171 μs | | | |
| [23](https://adventofcode.com/2023/day/23) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day23.rs) | 345 μs | | | |
| [24](https://adventofcode.com/2023/day/24) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day24.rs) | 310 μs | | | |
| [25](https://adventofcode.com/2023/day/25) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2023/day25.rs) | 112 μs | | | |
 | Total |     | 8 ms | | | |

*Year 2021*

| Day   | Source | Benchmark | Thread | SIMD | unsafe |
|:--------:|:--------:|:------:|:--------:|:-----:|:------:|
| [1](https://adventofcode.com/2021/day/1) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day01.rs) | 5.4 μs | | | |
| [2](https://adventofcode.com/2021/day/2) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day02.rs) | 0.8 μs | | | |
| [3](https://adventofcode.com/2021/day/3) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day03.rs) | 2 μs | |✓ | |
| [4](https://adventofcode.com/2021/day/4) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day04.rs) | 10 μs | | | |
| [5](https://adventofcode.com/2021/day/5) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day05.rs) | 114 μs | | | |
| [6](https://adventofcode.com/2021/day/6) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day06.rs) | 433 ns | | | |
| [7](https://adventofcode.com/2021/day/7) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day07.rs) | 4285 ns | | | |
| [8](https://adventofcode.com/2021/day/8) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day08.rs) | 12 μs | | | |
| [9](https://adventofcode.com/2021/day/9) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day09.rs) | 49 μs | | | |
| [10](https://adventofcode.com/2021/day/10) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day10.rs) | 7 μs | | | |
| [11](https://adventofcode.com/2021/day/11) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day11.rs) | 68 μs | | | |
| [12](https://adventofcode.com/2021/day/12) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day12.rs) | 8 μs | | | |
| [13](https://adventofcode.com/2021/day/13) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day13.rs) | 16 μs | | | |
| [14](https://adventofcode.com/2021/day/14) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day14.rs) | 7 μs | | | |
| [15](https://adventofcode.com/2021/day/15) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day15.rs) | 2837 μs | | | |
| [16](https://adventofcode.com/2021/day/16) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day16.rs) | 2 μs | | | |
| [17](https://adventofcode.com/2021/day/17) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day17.rs) | 6 μs | | | |
| [18](https://adventofcode.com/2021/day/18) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day18.rs) | 441 μs |✓ | | |
| [19](https://adventofcode.com/2021/day/19) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day19.rs) | 514 μs |✓ | | |
| [20](https://adventofcode.com/2021/day/20) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day20.rs) | 1446 μs | | | |
| [21](https://adventofcode.com/2021/day/21) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day21.rs) | 164 μs | | | |
| [22](https://adventofcode.com/2021/day/22) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day22.rs) | 890 μs | | | |
| [23](https://adventofcode.com/2021/day/23) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day23.rs) | 745 μs | | | |
| [24](https://adventofcode.com/2021/day/24) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day24.rs) | 2.8 μs | | | |
| [25](https://adventofcode.com/2021/day/25) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2021/day25.rs) | 319 μs | | | |
 | Total |     | 7.6ms | | | |

*Year 2017*
| Day   | Source | Benchmark | Thread | SIMD | unsafe |
|:--------:|:--------:|:------:|:--------:|:-----:|:------:|
| [1](https://adventofcode.com/2017/day/1) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day01.rs) | 3.6 μs | | | |
| [2](https://adventofcode.com/2017/day/2) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day02.rs) | 2 μs | | | |
| [3](https://adventofcode.com/2017/day/3) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day03.rs) | 3.2 μs | | | |
| [4](https://adventofcode.com/2017/day/4) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day04.rs) | 125 μs | | | |
| [5](https://adventofcode.com/2017/day/5) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day05.rs) | 38333 μs | | | |
| [6](https://adventofcode.com/2017/day/6) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day06.rs) | 107 μs | | | |
| [7](https://adventofcode.com/2017/day/7) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day07.rs) | 163 μs | | | |
| [8](https://adventofcode.com/2017/day/8) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day08.rs) | 101 μs | | | |
| [9](https://adventofcode.com/2017/day/9) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day09.rs) | 16 μs | | | |
| [10](https://adventofcode.com/2017/day/10) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day10.rs) | 51 μs | | | |
| [11](https://adventofcode.com/2017/day/11) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day11.rs) | 97 μs | | | |
| [12](https://adventofcode.com/2017/day/12) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day12.rs) | 76 μs | | | |
| [13](https://adventofcode.com/2017/day/13) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day13.rs) | 2328 ns | | | |
| [14](https://adventofcode.com/2017/day/14) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day14.rs) | 788 μs |✓ | | |
| [15](https://adventofcode.com/2017/day/15) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day15.rs) | 23073 μs |✓ | | |
| [16](https://adventofcode.com/2017/day/16) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day16.rs) | 75 μs | | | |
| [17](https://adventofcode.com/2017/day/17) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day17.rs) | 122 μs | | | |
| [18](https://adventofcode.com/2017/day/18) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day18.rs) | 4139 ns | | | |
| [19](https://adventofcode.com/2017/day/19) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day19.rs) | 20 μs | | | |
| [20](https://adventofcode.com/2017/day/20) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day20.rs) | 334 μs | | | |
| [21](https://adventofcode.com/2017/day/21) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day21.rs) | 9 μs | | | |
| [22](https://adventofcode.com/2017/day/22) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day22.rs) | 30337 μs | | | |
| [23](https://adventofcode.com/2017/day/23) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day23.rs) | 25 μs | | | |
| [24](https://adventofcode.com/2017/day/24) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day24.rs) | 762 μs | | | |
| [25](https://adventofcode.com/2017/day/25) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2017/day25.rs) | 6068 μs | | | |
 | Total |     | 100ms | | | |

*Year 2015*
| Day   | Source | Benchmark | Thread | SIMD | unsafe |
|:--------:|:--------:|:------:|:--------:|:-----:|:------:|
| [1](https://adventofcode.com/2015/day/1) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day01.rs) | 1 μs | | | |
| [2](https://adventofcode.com/2015/day/2) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day02.rs) | 5 μs | | | |
| [3](https://adventofcode.com/2015/day/3) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day03.rs) | 217 μs | | | |
| [4](https://adventofcode.com/2015/day/4) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day04.rs) | 5768 μs |✓ |✓ | |
| [5](https://adventofcode.com/2015/day/5) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day05.rs) | 31 μs | | | |
| [6](https://adventofcode.com/2015/day/6) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day06.rs) | 5552 μs | | | |
| [7](https://adventofcode.com/2015/day/7) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day07.rs) | 50 μs | | | |
| [8](https://adventofcode.com/2015/day/8) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day08.rs) | 6 μs | | | |
| [9](https://adventofcode.com/2015/day/9) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day09.rs) | 28 μs | | | |
| [10](https://adventofcode.com/2015/day/10) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day10.rs) | 6 μs | | | |
| [11](https://adventofcode.com/2015/day/11) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day11.rs) | 43 ns | | | |
| [12](https://adventofcode.com/2015/day/12) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day12.rs) | 54 μs | | | |
| [13](https://adventofcode.com/2015/day/13) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day13.rs) | 39 μs | | | |
| [14](https://adventofcode.com/2015/day/14) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day14.rs) | 37 μs | | | |
| [15](https://adventofcode.com/2015/day/15) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day15.rs) | 87 μs | | | |
| [16](https://adventofcode.com/2015/day/16) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day16.rs) | 69 μs | | | |
| [17](https://adventofcode.com/2015/day/17) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day17.rs) | 12 μs | | | |
| [18](https://adventofcode.com/2015/day/18) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day18.rs) | 72 μs | |✓ | |
| [19](https://adventofcode.com/2015/day/19) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day19.rs) | 66 μs | | | |
| [20](https://adventofcode.com/2015/day/20) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day20.rs) | 332 μs | | | |
| [21](https://adventofcode.com/2015/day/21) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day21.rs) | 3095 ns | | | |
| [22](https://adventofcode.com/2015/day/22) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day22.rs) | 890 μs | | | |
| [23](https://adventofcode.com/2015/day/23) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day23.rs) | 4 μs | | | |
| [24](https://adventofcode.com/2015/day/24) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day24.rs) | 27 μs | | | |
| [25](https://adventofcode.com/2015/day/25) | [source](https://github.com/gbagan/advent-of-code-rust/blob/master/src/year2015/day25.rs) | 0.1 μs | | | |
 | Total |     | 13ms | | | |
