use criterion::*;
use diva_db::bone::*;

const F_INPUT: (&str, &[u8]) = ("F", include_bytes!("../assets/f_bonedb.bin"));
const D2_INPUT: (&str, &[u8]) = ("DT 2nd", include_bytes!("../assets/d2nd_bonedb.bin"));
const DTEX_INPUT: (&str, &[u8]) = ("DT Extended", include_bytes!("../assets/dtex_bonedb.bin"));

fn read_database(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse");

    for (desc, input) in vec![F_INPUT, D2_INPUT, DTEX_INPUT] {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("Bone database read", desc), &input,
            |b, i| b.iter(|| {
                BoneDatabase::read(&i)
            }),
        );
    }
    group.finish();
}

criterion_group!(benches, read_database);
criterion_main!(benches);
