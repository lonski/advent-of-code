class Wire

  attr_accessor :points, :point_steps

  def initialize(s)
    @points = []
    @point_steps = {}
    @pos = [0,0]
    @steps = 0
    s.split(',').map{|i| [i[0], i[1..-1].to_i] }.each do |dir, val|
      case dir
      when "R" 
        (1..val).each{|n| add_point([@pos[0]+n, @pos[1]])}
      when "L" 
        (1..val).each{|n| add_point([@pos[0]-n, @pos[1]])}
      when "U" 
        (1..val).each{|n| add_point([@pos[0], @pos[1]+n])}
      when "D" 
        (1..val).each{|n| add_point([@pos[0], @pos[1]-n])}
      else 
        raise "Unsupported direction #{dir}"
      end
      @pos = @points[-1]
    end
  end

  def add_point(p)
    @steps += 1
    @points << p
    @point_steps[p] = @steps
  end

end

wires = File.read("input.1").lines.map{|l| Wire.new(l)}
puts "Part 1: #{(wires[0].points & wires[1].points).map{|p| p[0].abs + p[1].abs}.min}"
puts "Part 2: #{(wires[0].points & wires[1].points).map{|p| wires[0].point_steps[p] + wires[1].point_steps[p]}.min}"

