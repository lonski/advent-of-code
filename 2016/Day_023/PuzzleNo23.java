import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.Arrays;

enum Register
{
  A,
  B,
  C,
  D
}

class ProgramContext
{
  ProgramContext(Map<Register, Long> registers, List<Instruction> instructions){
    this.registers = registers;
    this.instructions = instructions;
    this.pointer = 0;
  }

  Map<Register, Long> registers;
  List<Instruction> instructions;
  int pointer;
}

interface Instruction
{
  /**
   * @return Instruction pointer movement
   */
  int execute(ProgramContext context);
  Instruction toggle();

  static Instruction parse(String str)
  {
    String[] tokens = str.split(" ");

    switch (tokens[0]) {
      case "cpy": 
        return PuzzleNo23.isNumeric(tokens[1]) ? new CopyInstruction(Long.parseLong(tokens[1]), Register.valueOf(tokens[2].toUpperCase()))
          : new CopyInstruction(Register.valueOf(tokens[1].toUpperCase()), Register.valueOf(tokens[2].toUpperCase()));
      case "inc":
        return new IncrementInstruction(Register.valueOf(tokens[1].toUpperCase()));
      case "dec":
        return new DecrementInstruction(Register.valueOf(tokens[1].toUpperCase()));
      case "jnz":
        return new JumpInstruction(tokens[1], tokens[2]);
      case "tgl":
        return new ToggleInstruction(Register.valueOf(tokens[1].toUpperCase()));
    }

    return null;
  }
}

class ToggleInstruction implements Instruction
{
  private Register register;

  ToggleInstruction(Register register){
    this.register = register;
  }

  @Override
  public int execute(ProgramContext context){
    long regValue = context.registers.get(register);
    int pointer = context.pointer + (int)regValue;
    if ( pointer < context.instructions.size() ) {
      Instruction i = context.instructions.get(pointer);
      System.out.println(i);
    }
    return 1;
  }
}  

class CopyInstruction implements Instruction
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

  public Instruction toggle() {
    return new JumpInstruction( value == null ? String.valueOf(sourceReg).toLowerCase() : String.valueOf(value), 
        String.valueOf(destinationReg).toLowerCase());
  }

  @Override
  public int execute(ProgramContext context)
  {
    long modifier = value == null ? context.registers.get(sourceReg) : value;
    context.registers.put(destinationReg, modifier);
    return 1;
  }
}

class IncrementInstruction implements Instruction
{
  private Register register;

  IncrementInstruction(Register register)
  {
    this.register = register;
  }

  public Instruction toggle() {
    return new DecrementInstruction(register);
  }

  @Override
  public int execute(ProgramContext context)
  {
    context.registers.put(register, context.registers.get(register) + 1);
    return 1;
  }
}

class DecrementInstruction implements Instruction
{
  private Register register;

  DecrementInstruction(Register register)
  {
    this.register = register;
  }

  public Instruction toggle() {
    return new IncrementInstruction(register);
  }

  @Override
  public int execute(ProgramContext context)
  {
    context.registers.put(register, context.registers.get(register) - 1);
    return 1;
  }
}

class JumpInstruction implements Instruction{
  private Register registerOperandValue;
  private Long operandValue;
  private Long jumpVal;
  private Register jumpValReg;

  JumpInstruction(String arg1, String arg2){
    if ( PuzzleNo23.isNumeric(arg1) && PuzzleNo23.isNumeric(arg2)){
      init(Long.parseLong(arg1), Long.parseLong(arg2));
    }
    else if ( PuzzleNo23.isNumeric(arg2) ) {
      init(Register.valueOf(arg1.toUpperCase()), Long.parseLong(arg2));
    }
    else if ( PuzzleNo23.isNumeric(arg1) ) {
      init(Long.parseLong(arg1), Register.valueOf(arg2.toUpperCase()));
    }
    else{
      init(Register.valueOf(arg1.toUpperCase()), Register.valueOf(arg2.toUpperCase()));
    }
  }

  private void init(Register registerOperandValue, long value){
    this.registerOperandValue = registerOperandValue;
    this.jumpVal = value;
    this.operandValue = null;
    this.jumpValReg = null;
  }

  private void init(long operandValue, Register registerValue){
    this.operandValue = operandValue;
    this.jumpValReg = registerValue;
    this.jumpVal = null;
  }

  private void init(Register registerOperandValue, Register registerValue){
    this.registerOperandValue = registerOperandValue;
    this.jumpValReg = registerValue;
    this.jumpVal = null;
    this.operandValue = null;
  }

  private void init(long operandValue, long value){
    this.operandValue = operandValue;
    this.jumpVal = value;
    this.jumpValReg = null;
  }

  @Override
  public int execute(ProgramContext context)
  {
    long operand = operandValue == null ? context.registers.get(registerOperandValue) : operandValue;
    long valueToJump = jumpVal == null ? context.registers.get(jumpValReg) : jumpVal;
    return operand != 0 ? (int)valueToJump : 1;
  }
}

public class PuzzleNo23
{
  public static void main(String[] args)
    throws IOException
  {
    Map<Register, Long> registers = new HashMap<>();
    registers.put(Register.A, 7L);
    registers.put(Register.B, 0L);
    registers.put(Register.C, 0L);
    registers.put(Register.D, 0L);

    System.out.println(PuzzleNo23.isNumeric("c"));

    List<Instruction> instructions = Files.readAllLines(Paths.get("puzzle_23.input"))
      .stream()
      .map(Instruction::parse)
      .collect(Collectors.toList());

    ProgramContext context = new ProgramContext(registers, instructions);
    while (context.pointer < context.instructions.size()) {
      Instruction i = context.instructions.get(context.pointer);
      context.pointer += i.execute(context);
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
