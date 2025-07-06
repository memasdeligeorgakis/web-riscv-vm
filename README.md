# RISC-V VM


> :warning: Incomplete at the current state

## How to run it

```bash
# chmod +x ./scripts/run_tests.sh
./scripts/run_tests.sh
```

## Roadmap

⬜️ = TODO
🟨 = WIP
✅ = DONE

* [🟨] RV32I
* [🟨] basic infra
* [⬜️] RV32M
* [⬜️] RV32F
* [⬜️] RV32D
* [⬜️] RV32V


## project structure

```bash
/src
	/code_examples				# example rust programs to generate RISC-V code
	/emulator
		/emulator.rs			# core of the emulator
		/rv32i.rs				# implementation of RV32I instructions
		/instruction_formats.rs	# instructions formats R, I, S, B, U
```

## Specs

For this implementation is based on [The RISC-V Instruction Set Manual Volume I](https://drive.google.com/file/d/1uviu1nH-tScFfgrovvFCrj7Omv8tFtkp/view)