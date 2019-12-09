class Computer

  attr_accessor :state, :feedback, :output

  def initialize(program, phase)
    @pointer = 0
    @output = 0
    @phase = phase
    @phase_set = false
    @input = 0
    @state = :running
    @feedback = nil
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
        if !@phase_set
          @phase_set = true
          @mem[@mem[@pointer + 1]] = @phase
          @pointer += 2
        elsif @input != nil
          @mem[@mem[@pointer + 1]] = @input
          @pointer += 2
          @input = nil
        else
          @state = :waiting_for_input
        end
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
      end,
      99 => Proc.new { |pmode| @state = :halt }
    }
  end

  def run_with_feedback
    run(@feedback.output)
  end

  def run(input)
    @input = input
    @state = :running
    instruction = parse_instruction(@mem[@pointer])
    while @state == :running do
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

def execute_part1(program)
  max = 0
  [0,1,2,3,4].permutation do |perm|
    input = 0;
    perm.each do |phase|
      input = Computer.new(program, phase).run(input)
      max = [input, max].max 
    end
  end
  max
end

def execute_part2(program)
  max = 0
  [5,6,7,8,9].permutation do |perm|
    amps = [
      Computer.new(program, perm[0]),
      Computer.new(program, perm[1]),
      Computer.new(program, perm[2]),
      Computer.new(program, perm[3]),
      Computer.new(program, perm[4])
    ]

    amps[0].feedback = amps[4]
    amps[1].feedback = amps[0]
    amps[2].feedback = amps[1]
    amps[3].feedback = amps[2]
    amps[4].feedback = amps[3]

    amps
      .cycle
      .take_while do |a| 
        a.run_with_feedback
        !amps.all?{|amp| amp.state == :halt }
      end

    max = [amps[4].output, max].max
  end

  max
end

program = File.read 'input_0'
puts "Part 1: #{execute_part1(program)}"
puts "Part 2: #{execute_part2(program)}"
