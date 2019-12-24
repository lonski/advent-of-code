class Moon
  attr_accessor :position, :velocity

  def initialize(str)
    @velocity = [0,0,0]
    m = str.match /<x=(.+), y=(.+)*, z=(.+)>/
    @position = [ m[1].to_i, m[2].to_i, m[3].to_i ]
  end

  def energy
    @position.map(&:abs).reduce(&:+) * @velocity.map(&:abs).reduce(&:+)
  end

  def state
    @position + @velocity
  end

  def state_dim(i)
    @position[i] + @velocity[i]
  end
end

class MoonSimulation

  def initialize(input_lines)
    @moons = input_lines.map{|l| Moon.new(l)}
    @states = []
  end

  def step(n)
    n.times do 
      calculate_velocity
      apply_velocity
    end
  end

  def find_repeated_state
    initial = (0..2).map{|d| @moons.map{|m| m.state_dim(d) } }
    phases = [0,0,0]
    counter = 0

    while true
      calculate_velocity
      apply_velocity
      counter += 1

      (0..2).map{|d| @moons.map{|m| m.state_dim(d) } }.each_with_index do |current,i|
        phases[i] = counter if current == initial[i] && phases[i] == 0
      end

      break if phases.none?{|v| v == 0}
    end

    phases.reduce(:lcm)
  end

  def apply_velocity
    @moons.each do |moon|
      (0..2).each do |axis|
        moon.position[axis] += moon.velocity[axis]
      end
    end
  end

  def calculate_velocity
    (0..2).each do |axis|
      @moons.each do |moon|
        @moons.each do |other_moon|
          if moon != other_moon
            if other_moon.position[axis] > moon.position[axis]
              moon.velocity[axis] += 1
            elsif other_moon.position[axis] < moon.position[axis]
              moon.velocity[axis] -= 1
            end
          end
        end
      end
    end
  end

  def total_energy
    @moons.map(&:energy).reduce(&:+)
  end

  def print
    @moons.each do |m|
      puts "pos=<x=#{m.position[0]}, y=#{m.position[1]}, z=#{m.position[2]}> vel=<x=#{m.velocity[0]}, y=#{m.velocity[1]}, z=#{m.velocity[2]}>"
    end
  end
  
end

ms = MoonSimulation.new(File.read('input_0').lines)
ms.step(1000)
puts "Part 1: #{ms.total_energy}"

ms = MoonSimulation.new(File.read('input_0').lines)
puts "Part 2: #{ms.find_repeated_state}"

