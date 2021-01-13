use ckb_vm::SupportMachine;

mod convention;
mod cost_model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let code_data = std::fs::read(&args[1])?;
    let code = bytes::Bytes::from(code_data);

    let core_machine = ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new(
        convention::ISA,
        convention::VERSION,
        u64::MAX,
    );

    let machine_builder = ckb_vm::DefaultMachineBuilder::new(core_machine)
        .instruction_cycle_func(Box::new(cost_model::instruction_cycles));
    let mut machine = machine_builder.build();
    machine.load_program(&code, &vec!["main".into()]).unwrap();

    let exit = machine.run();
    let cycles = machine.cycles();
    println!("int exit={:?} cycles={:?}", exit, cycles);

    Ok(())
}
