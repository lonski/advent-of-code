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
end

class MoonSimulation

  def initialize(input_lines)
    @moons = input_lines.map{|l| Moon.new(l)}
    @states = []
    @states << calc_state
  end

  def step(n)
    n.times do 
      calculate_velocity
      apply_velocity
    end
  end

  def find_repeated_state
    initial_state = calc_state
    counter = 0
    while true
      calculate_velocity
      apply_velocity
      counter += 1
      #current_state = calc_state
      break if calc_state == initial_state
      #@states << current_state
      puts counter if counter % 100000 == 0
    end

    counter
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

  def calc_state
    @moons.map(&:state).reduce(&:+).to_s
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

