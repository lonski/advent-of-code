class Computer

  def initialize(program, noun, verb)
    @pointer = 0
    @mem = program.split(",").map(&:to_i)
    @mem[1] = noun
    @mem[2] = verb

    @funcs = {
      1 => Proc.new { @mem[@mem[@pointer + 3]] = @mem[@mem[@pointer + 1]] + @mem[@mem[@pointer + 2]] }, #add
      2 => Proc.new { @mem[@mem[@pointer + 3]] = @mem[@mem[@pointer + 1]] * @mem[@mem[@pointer + 2]] }  #multiply
    }
  end

  def run
    instruction = @mem[@pointer]
    while instruction != 99 do
      @funcs[instruction].call if @funcs.key? instruction
      @pointer += 4 
      instruction = @mem[@pointer]
    end
    @mem[0]
  end

end

program = File.read 'input.1'
puts "Part 1: #{Computer.new(program, 12, 2).run}"

100.times do |verb|
  100.times do |noun|
    if Computer.new(program, noun, verb).run == 19690720
      puts "Part 2: #{100 * noun + verb}" 
      exit 0
    end
  end
end
