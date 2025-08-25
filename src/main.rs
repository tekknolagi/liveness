use ascent::ascent;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct BlockId(i32);

impl std::fmt::Debug for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B{}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct VarId(i32);

impl std::fmt::Debug for VarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{}", self.0)
    }
}

ascent! {
    #![measure_rule_times]
    relation block_use(BlockId, VarId);
    relation block_def(BlockId, VarId);
    relation block_succ(BlockId, BlockId);  // (succ, pred)
    relation live_out(BlockId, VarId);
    relation live_in(BlockId, VarId);
    live_out(b, v) <-- block_succ(s, b), live_in(s, v);
    live_in(b, v) <-- (live_out(b, v) | block_use(b, v)), !block_def(b, v);
    // live_out(n, v) <-- block_pred(n, m),
    //                    block_use(m, v),
    //                    (live_out(m, v) | !block_def(m, v));
}
fn main() {
    let mut prog = AscentProgram::default();
    let b1 = BlockId(1);
    let b2 = BlockId(2);
    let b3 = BlockId(3);
    let b4 = BlockId(4);
    let r10 = VarId(10);
    let r11 = VarId(11);
    let r12 = VarId(12);
    let r13 = VarId(13);
    let r14 = VarId(14);
    let r15 = VarId(15);
    let r16 = VarId(16);
    prog.block_def = vec![
        (b1, r10),
        (b1, r11),
        (b2, r12),
        (b2, r13),
        (b3, r14),
        (b3, r15),
        (b4, r16),
    ];
    prog.block_succ = vec![
        (b2, b1),
        (b3, b2),
        (b2, b3),
        (b4, b2),
    ];
    prog.block_use = vec![
        (b1, r11),
        (b2, r13),
        (b3, r12),
        (b3, r13),
        (b3, r14),
        (b3, r15),
        (b4, r10),
        (b4, r12),
        (b4, r16),
    ];
    prog.run();
    println!("live out: {:?}", prog.live_out);
    println!("live in: {:?}", prog.live_in);
    println!("{}", prog.scc_times_summary());
}
