
compile:
	@echo "Compiling $(filename)..."
	@mkdir -p build
	@nasm -f elf32 -F dwarf -o build/$(filename).o $(filename)
	@ld -m elf_i386 -o build/$(filename).out build/$(filename).o 

run: compile
	@echo "Running executable.."
	@./build/$(filename).out
	
