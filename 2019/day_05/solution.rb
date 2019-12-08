class Computer

  def initialize(program, input)
    @pointer = 0
    @output = 0
    @input = input
    @mem = program.split(",").map(&:to_i)
    @funcs = {
      1 => Proc.new do |pmode| #add
        @mem[@mem[@pointer + 3]] = deref(@pointer + 1, pmode[0]) + deref(@pointer + 2, pmode[1])
        @pointer += 4
      end,
      2 => Proc.new do |pmode| #multiply
        @mem[@mem[@pointer + 3]] = deref(@pointer + 1, pmode[0]) * deref(@pointer + 2, pmode[1])
        @pointer += 4
      end,
      3 => Proc.new do |pmode| #input
        @mem[@mem[@pointer + 1]] = @input
        @pointer += 2
      end,
      4 => Proc.new do |pmode| #output
        @output = @mem[@mem[@pointer + 1]]
        @pointer += 2
      end,
      5 => Proc.new do |pmode| #jump_if_true
        if deref(@pointer+1, pmode[0]) != 0
          @pointer = deref(@pointer+2, pmode[1])
        else
          @pointer += 3
        end
      end,
      6 => Proc.new do |pmode| #jump_if_false
        if deref(@pointer+1, pmode[0]) == 0
          @pointer = deref(@pointer+2, pmode[1])
        else
          @pointer += 3
        end
      end,
      7 => Proc.new do |pmode| #less_than
        p1 = deref(@pointer+1, pmode[0]) 
        p2 = deref(@pointer+2, pmode[1]) 
        @mem[@mem[@pointer + 3]] = p1 < p2 ? 1 : 0
        @pointer += 4
      end,
      8 => Proc.new do |pmode| #equals
        p1 = deref(@pointer+1, pmode[0]) 
        p2 = deref(@pointer+2, pmode[1]) 
        @mem[@mem[@pointer + 3]] = p1 == p2 ? 1 : 0
        @pointer += 4
      end
    }
  end

  def run
    instruction = parse_instruction(@mem[@pointer])
    while instruction[:opcode] != 99 do
      @funcs[instruction[:opcode]].call instruction[:pmode]
      instruction = parse_instruction(@mem[@pointer])
    end
    @output
  end

  def parse_instruction(n)
    return {opcode: n, pmode:[]} if n<100
    digits = n.digits
    {opcode: digits.reverse[-2..-1].join.to_i, pmode:digits[2..-1]}
  end

  def deref(addr, mode)
    (mode || 0) == 0 ? @mem[@mem[addr]] : @mem[addr]
  end
end

program = File.read 'input.1'
puts "Part 1: #{Computer.new(program, 1).run}"
puts "Part 2: #{Computer.new(program, 5).run}"
