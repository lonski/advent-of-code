import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

enum Register
{
    A,
    B,
    C,
    D
}

interface Instruction
{
    /**
     * @return Instruction pointer movement
     */
    int execute(Map<Register, Long> registers);

    static Instruction parse(String str)
    {
        String[] tokens = str.split(" ");

        switch (tokens[0]) {
            case "cpy": {
                return PuzzleNo12.isNumeric(tokens[1]) ? new CopyInstruction(Long.parseLong(tokens[1]), Register.valueOf(tokens[2].toUpperCase()))
                        : new CopyInstruction(Register.valueOf(tokens[1].toUpperCase()), Register.valueOf(tokens[2].toUpperCase()));
            }
            case "inc":
                return new IncrementInstruction(Register.valueOf(tokens[1].toUpperCase()));
            case "dec":
                return new DecrementInstruction(Register.valueOf(tokens[1].toUpperCase()));
            case "jnz":
                return PuzzleNo12.isNumeric(tokens[1]) ? new JumpInstruction(Long.parseLong(tokens[1]), Long.parseLong(tokens[2]))
                        : new JumpInstruction(Register.valueOf(tokens[1].toUpperCase()), Long.parseLong(tokens[2]));
        }

        return null;
    }
}

class CopyInstruction
        implements Instruction
{
    private Long value;
    private Register sourceReg;
    private Register destinationReg;

    CopyInstruction(Register source, Register register)
    {
        this.value = null;
        this.sourceReg = source;
        this.destinationReg = register;
    }

    CopyInstruction(Long value, Register register)
    {
        this.value = value;
        this.destinationReg = register;
    }

    @Override
    public int execute(Map<Register, Long> registers)
    {
        long modifier = value == null ? registers.get(sourceReg) : value;
        registers.put(destinationReg, modifier);
        return 1;
    }
}

class IncrementInstruction
        implements Instruction
{
    private Register register;

    IncrementInstruction(Register register)
    {
        this.register = register;
    }

    @Override
    public int execute(Map<Register, Long> registers)
    {
        registers.put(register, registers.get(register) + 1);
        return 1;
    }
}

class DecrementInstruction
        implements Instruction
{
    private Register register;

    DecrementInstruction(Register register)
    {
        this.register = register;
    }

    @Override
    public int execute(Map<Register, Long> registers)
    {
        registers.put(register, registers.get(register) - 1);
        return 1;
    }
}

class JumpInstruction implements Instruction{
    private Register registerOperandValue;
    private Long operandValue;
    private long jumpVal;

    JumpInstruction(Register registerOperandValue, long value){
        this.registerOperandValue = registerOperandValue;
        this.jumpVal = value;
        this.operandValue = null;
    }

    JumpInstruction(long operandValue, long value){
        this.operandValue = operandValue;
        this.jumpVal = value;
    }

    @Override
    public int execute(Map<Register, Long> registers)
    {
        long operand = operandValue == null ? registers.get(registerOperandValue) : operandValue;
        return operand != 0 ? (int)jumpVal : 1;
    }
}

public class PuzzleNo12
{
    public static void main(String[] args)
            throws IOException
    {
        Map<Register, Long> registers = new HashMap<>();
        registers.put(Register.A, 0L);
        registers.put(Register.B, 0L);
        registers.put(Register.C, 1L);
        registers.put(Register.D, 0L);

        int pointer = 0;

        List<Instruction> instructions = Files.readAllLines(Paths.get("puzzle_12.input"))
                .stream()
                .map(Instruction::parse)
                .collect(Collectors.toList());

        while (pointer < instructions.size()) {
            Instruction i = instructions.get(pointer);
            pointer += i.execute(registers);
            //System.out.println(String.format("%d %d", pointer, instructions.size()));
        }

        registers.entrySet()
                .stream()
                .map( e -> String.format("%s -> %d", e.getKey(), e.getValue()))
                .forEach(System.out::println);
    }

    public static boolean isNumeric(String str)
    {
        try {
            double d = Double.parseDouble(str);
        }
        catch (NumberFormatException nfe) {
            return false;
        }
        return true;
    }
}
