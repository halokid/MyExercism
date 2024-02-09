# Poker

Welcome to Poker on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Instructions

Pick the best hand(s) from a list of poker hands.

See [wikipedia][poker-hands] for an overview of poker hands.

[poker-hands]: https://en.wikipedia.org/wiki/List_of_poker_hands

## Source

### Created by

- @coriolinus

### Contributed to by

- @Baelyk
- @CGMossa
- @cwhakes
- @efx
- @elektronaut0815
- @ErikSchierboom
- @lutostag
- @nfiles
- @PaulDance
- @petertseng
- @rofrol
- @stringparser
- @xakon
- @ZapAnton

### Based on

Inspired by the training course from Udacity. - https://www.udacity.com/course/design-of-computer-programs--cs212


--------------------------------

这段Rust代码实现了一个用于比较扑克牌手的函数winning_hands和相关的数据结构和方法。

use std::cmp::Reverse; 和 use std::collections::{BinaryHeap, HashMap}; 导入了需要使用的标准库中的一些模块。

PokerHand 结构体定义了一个扑克牌手的数据结构，其中包含了牌面值的计数和牌面值本身。

parse_card 函数用于解析扑克牌的字符串表示，将其转换成数字表示。

impl PokerHand 块下定义了与 PokerHand 结构体相关的方法，其中包括 parse 方法。该方法用于解析扑克牌手的字符串表示，并创建相应的 PokerHand 实例。

winning_hands 函数接受一个包含多个扑克牌手的数组，并返回一个数组，其中包含获胜的扑克牌手。在该函数中，首先将输入的扑克牌手字符串转换为 PokerHand 实例，并放入二进制堆（BinaryHeap）中进行排序。然后从堆中依次取出最大值，并将其对应的扑克牌手放入结果数组中。

总体来说，这段代码实现了一个简单的扑克牌比较算法，用于确定输入的扑克牌手中获胜的手。


