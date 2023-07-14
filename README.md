# m6502-emulator-rust
A rust based m6502 emulator for following along with Ben Eater's "Buil a 65c02-based computer from scratch"

## How to use
You can make your own programs by updating the programs/default.txt file or making a new file in programs.
Later, you can run the program by typing `run <program_name>` (`run -df` for running the default program) in the CLI.

### Important addresses
    100-1FF -   Stack
    6000-6001 - Ports A and B
    6002 -      Print Port: Prints whatever is stored to this port when changed
    8000-FFFF - EEPROM: where the program will be saved.

### Important notes for assembly programs
When creating a new program note that it is very important to set the `.org` to `$8000`. This will set the location of the program to the start of the EEPROM. When trying to store to a memory module which cannot be written to (EEPROM for example) the programm will crash, informing the user of the error.
You can store whatever you want in address `$6002` to print it to the console when running.

### Advanced CLI features
You can configure your CPU's instruction time by typing `configure -It <speed_in_ms>`. This can be useful for running infinite programs like fibonacci.
More configurations to come!