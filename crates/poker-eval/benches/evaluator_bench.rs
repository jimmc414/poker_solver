use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use poker_core::{Card, Rank, Suit};
use poker_eval::LookupTableEvaluator;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

fn generate_random_7card_hands(count: usize) -> Vec<[Card; 7]> {
    let mut rng = StdRng::seed_from_u64(42);
    let all_cards: Vec<Card> = (0..52u8)
        .map(|i| Card::from_u8(i).expect("valid"))
        .collect();

    let mut hands = Vec::with_capacity(count);
    for _ in 0..count {
        let mut deck = all_cards.clone();
        deck.shuffle(&mut rng);
        hands.push([deck[0], deck[1], deck[2], deck[3], deck[4], deck[5], deck[6]]);
    }
    hands
}

fn bench_7card_evaluation(c: &mut Criterion) {
    let eval = LookupTableEvaluator::new();
    let hands = generate_random_7card_hands(10_000);

    let mut group = c.benchmark_group("hand_evaluation");
    group.throughput(Throughput::Elements(hands.len() as u64));
    group.sample_size(100);

    group.bench_function("evaluate_7_cards", |b| {
        b.iter(|| {
            for hand in &hands {
                criterion::black_box(eval.evaluate_7(hand));
            }
        });
    });

    group.finish();
}

fn bench_5card_evaluation(c: &mut Criterion) {
    let eval = LookupTableEvaluator::new();

    let mut rng = StdRng::seed_from_u64(123);
    let all_cards: Vec<Card> = (0..52u8)
        .map(|i| Card::from_u8(i).expect("valid"))
        .collect();

    let mut hands = Vec::with_capacity(10_000);
    for _ in 0..10_000 {
        let mut deck = all_cards.clone();
        deck.shuffle(&mut rng);
        hands.push([deck[0], deck[1], deck[2], deck[3], deck[4]]);
    }

    let mut group = c.benchmark_group("hand_evaluation_5card");
    group.throughput(Throughput::Elements(hands.len() as u64));
    group.sample_size(100);

    group.bench_function("evaluate_5_cards", |b| {
        b.iter(|| {
            for hand in &hands {
                criterion::black_box(eval.evaluate_5(hand));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_7card_evaluation, bench_5card_evaluation);
criterion_main!(benches);
