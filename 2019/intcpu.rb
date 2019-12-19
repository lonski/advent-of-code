class IntCpu

  attr_accessor :output, :input, :state, :handle_output_proc

  def initialize(program)
    @pointer = 0
    @state = :running
    @input = 0
    @output = []
    @mem = program.split(",").map(&:to_i)
    @handle_output_proc = nil
    @relative_base = 0
    @funcs = {
      1 => Proc.new do |p| #add
        set_param(3,p,get_param(1,p) + get_param(2,p))
        @pointer += 4
      end,
      2 => Proc.new do |p| #multiply
        set_param(3,p,get_param(1,p) * get_param(2,p))
        @pointer += 4
      end,
      3 => Proc.new do |p| #input
        if @input.nil?
          @state = :waiting_for_input
        else
          set_param(1, p, @input)
          @input = nil
          @pointer += 2
        end
      end,
      4 => Proc.new do |p| #output
        output = get_param(1,p)
        @output << output
        @handle_output_proc.call(output) unless @handle_output_proc.nil?
        @pointer += 2
      end,
      5 => Proc.new do |p| #jump_if_true
        if get_param(1,p) != 0
          @pointer = get_param(2,p)
        else
          @pointer += 3
        end
      end,
      6 => Proc.new do |p| #jump_if_false
        if get_param(1,p) == 0
          @pointer = get_param(2,p)
        else
          @pointer += 3
        end
      end,
      7 => Proc.new do |p| #less_than
        val = get_param(1,p) < get_param(2,p) ? 1 : 0
        set_param(3, p, val)
        @pointer += 4
      end,
      8 => Proc.new do |p| #equals
        val  = get_param(1,p) == get_param(2,p) ? 1 : 0
        set_param(3, p, val)
        @pointer += 4
      end,
      9 => Proc.new do |p| # relative base offset
        @relative_base += get_param(1,p)
        @pointer += 2
      end,
      99 => Proc.new { |p| @state = :halt }
    }
  end

  def run
    @output = []
    @state = :running
    instruction = parse_instruction(@mem[@pointer])
    while @state == :running do
      @funcs[instruction[:opcode]].call instruction
      instruction = parse_instruction(@mem[@pointer])
    end
  end

  def set_mem(addr, val)
    @mem[addr] = val
  end

  def parse_instruction(n)
    return {opcode: n, pmode:[]} if n<100
    digits = n.digits
    {opcode: digits.reverse[-2..-1].join.to_i, pmode:digits[2..-1]}
  end

  def set_param(param_no, inst, val)
    @mem[addr_for(@pointer + param_no,inst[:pmode][param_no-1])] = val
  end

  def get_param(param_no, instr)
    addr = addr_for(@pointer+param_no, instr[:pmode][param_no-1])
    @mem[addr] || 0
  end

  def addr_for(pointer, mode)
    case (mode || 0) 
    when 0 
      @mem[pointer] || 0
    when 1
      pointer || 0
    when 2
      @relative_base + @mem[pointer] || 0
    end
  end
end
