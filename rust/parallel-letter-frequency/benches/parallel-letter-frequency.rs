// benches/parallel-letter-frequency.rs

#![feature(test)]  // 如果需要在 nightly 版本上运行基准测试，请加上这个 feature

extern crate test;

use test::Bencher;
// 请将 "your_crate" 替换为你的实际 crate 名称
use parallel_letter_frequency::frequency;

const SMALL: [&str; 8] = [
  "Freude schöner Götterfunken",
  "Tochter aus Elysium,",
  "Wir betreten feuertrunken,",
  "Himmlische, dein Heiligtum!",
  "Deine Zauber binden wieder",
  "Was die Mode streng geteilt;",
  "Alle Menschen werden Brüder,",
  "Wo dein sanfter Flügel weilt.",
];

const BIG: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

#[bench]
fn bench_frequency_1(b: &mut Bencher) {
  // 设置基准测试的输入数据
  // let input = ODE_AN_DIE_FREUDE; // 你的输入数据;
  let input = BIG; // 你的输入数据;

  // 基准测试的主体逻辑
  b.iter(|| {
    // 调用你的基准测试函数
    frequency(&input, 1);
  });
}

#[bench]
fn bench_frequency_2(b: &mut Bencher) {
  // 设置基准测试的输入数据
  // let input = ODE_AN_DIE_FREUDE; // 你的输入数据;
  let input = BIG; // 你的输入数据;

  // 基准测试的主体逻辑
  b.iter(|| {
    // 调用你的基准测试函数
    frequency(&input, 3);
  });
}





