use ckb_vm::SupportMachine;

mod cost_model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let code_data = std::fs::read(&args[1])?;
    let code = bytes::Bytes::from(code_data);

    let asm_core = ckb_vm::machine::asm::AsmCoreMachine::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION1,
        u64::MAX,
    );
    let core =
        ckb_vm::DefaultMachineBuilder::<Box<ckb_vm::machine::asm::AsmCoreMachine>>::new(asm_core)
            .instruction_cycle_func(Box::new(cost_model::instruction_cycles))
            .build();
    let mut machine = ckb_vm::machine::asm::AsmMachine::new(core, None);
    machine.load_program(&code, &vec!["main".into()]).unwrap();

    let exit = machine.run();
    let cycles = machine.machine.cycles();
    println!("asm exit={:?} cycles={:?}", exit, cycles);

    Ok(())
}
