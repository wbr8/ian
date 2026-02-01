# Interactive AQA assembly language iNterpreter
This application will allow for programs written in AQA Assembly Language to be 
ran and memory contents inspected. It stays true to the original specification, 
and does not implement any additional instructions (for now).

## Specification
Instructions are identical to [the original spec](https://filestore.aqa.org.uk/resources/computing/AQA-75162-75172-ALI.PDF).
The interpreter provides 16 R registers, and 1 KiB of RAM (256 signed 32 bit words).
