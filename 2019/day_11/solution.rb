class Computer

  def initialize(program)
    @pointer = 0
    @input = 0
    @mem = program.split(",").map(&:to_i)
    @handle_output_proc = Proc.new {|v|}
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
        set_param(1, p, @input)
        @pointer += 2
      end,
      4 => Proc.new do |p| #output
        output = get_param(1,p)
        @handle_output_proc.call(output)
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
      end
    }
  end

  def set_input(input)
    @input = input
  end

  def set_output_handler(handler)
    @handle_output_proc = handler
  end

  def run
    @output = []
    instruction = parse_instruction(@mem[@pointer])
    while instruction[:opcode] != 99 do
      @funcs[instruction[:opcode]].call instruction
      instruction = parse_instruction(@mem[@pointer])
    end
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

class Hull

  attr_accessor :panels

  def initialize
    @panels = {}
    @panels[[0,0]] = 0
  end

  def set_panel_color(pos, color)
    @panels[pos] = color
  end

  def get_panel_color(pos)
    @panels[pos] || 0
  end

  def print
    map = []
    @panels.each do |a, c|
      map[a[1]] ||= []
      map[a[1]][a[0]] = c
    end
    (0..map.size-1).each do |y|
      row = map[y]
      (0..row.size-1).each do |x|
        c = row[x] || 0
        putc c == 0 ? ' ' : '#'
      end
      puts
    end
  end
end

class Robot
  attr_accessor :pos, :dir

  def initialize(pos, dir)
    @pos = pos
    @dir = dir
    @dirs = ["UP", "RIGHT", "DOWN", "LEFT"]
  end

  def move(turn)
    #determine new direction
    dir_idx = @dirs.find_index @dir  
    dir_idx -= 1 if turn == 0
    dir_idx += 1 if turn == 1
    dir_idx = @dirs.size - 1 if dir_idx == -1
    dir_idx = 0 if dir_idx == @dirs.size

    #set new dir and move
    @dir = @dirs[dir_idx]
    case @dir
      when "UP"
        pos[1] -= 1
      when "DOWN"
        pos[1] += 1
      when "LEFT"
        pos[0] -= 1
      when "RIGHT"
        pos[0] += 1
    end
  end

  def paint(program, hull, start_input)
    cpu = Computer.new(program)
    cpu.set_input(start_input)
    output = []
    cpu.set_output_handler(
      Proc.new do |v|
        output << v
        if output.size == 2
          hull.set_panel_color(@pos.dup, output[0])
          move(output[1])
          cpu.set_input(hull.get_panel_color(@pos.dup))
          output = []
        end
      end
    )
    cpu.run
  end
end

program = File.read 'input_0'

hull = Hull.new
Robot.new([0,0], "UP").paint(program, hull, 0)
puts "Part 1: #{hull.panels.size}"

hull = Hull.new
Robot.new([0,0], "UP").paint(program, hull, 1)
puts "Part 2:"
hull.print

