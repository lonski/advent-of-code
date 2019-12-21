require_relative '../intcpu.rb'
require_relative '../board.rb'

DEBUG = false

class Droid

  START_POS = [40,40]

  attr_accessor :oxygen_station

  def initialize(fn)
    program = File.read fn
    @cpu = IntCpu.new(program)

    map_size = 65
    @map = map_size.times.map{ map_size.times.map { ' ' } }
    @pos = START_POS.dup
    set_tile(@pos, '.')
  end

  def set_tile(pos, c)
    @map[pos[1]][pos[0]] = c
  end

  def move_to(pos)
    output = 0

    return nil if @map[pos[1]][pos[0]] == '#'

    find_best_path(@pos, pos).each do |new_pos|
      dir = pos_to_dir(new_pos)
      @cpu.input = dir
      @cpu.run
      output = @cpu.output[0]

      case output
      when 0
        set_tile(new_pos, '#')
        return nil unless new_pos == pos
      when 1
        @pos = new_pos
        set_tile(new_pos, '.')
      when 2
        @pos = new_pos
        @oxygen_station = @pos.dup
        set_tile(new_pos, 'S')
      end

      #print_map

    end

    output
  end

  def explore

    frontier = [@pos.dup]
    visited = []

    while !frontier.empty?
      current = frontier.shift
      get_neighbours(current).each do |new_pos|
        unless visited.include? new_pos

          output = move_to(new_pos)
          
          visited << new_pos unless output.nil?
          frontier << new_pos unless output == 0

        end
      end
    end
  end

  def find_best_path(start, finish)
    frontier = [start.dup]
    came_from = {}
    came_from[start.dup] = nil

    while !frontier.empty?
      current = frontier.shift

      break if current == finish

      get_neighbours(current).each do |n|
        unless came_from.include? n
          frontier << n
          came_from[n] = current
        end
      end
    end

    current = finish.dup
    path = []

    while current != start
      path << current
      current = came_from[current]
      if current.nil?
        puts "FAILED TO FIND PATH"
        return []
      end
    end

    path.reverse
  end

  def get_neighbours(pos)
    [
      [pos[0]+1, pos[1]],
      [pos[0]-1, pos[1]],
      [pos[0], pos[1]+1],
      [pos[0], pos[1]-1],
    ]
    .reject{|p| @map[p[1]].nil? }
    .reject{|p| ['#'].include? @map[p[1]][p[0]]  }
  end

  def pos_to_dir(pos)
    dx = pos[0] - @pos[0]
    dy = pos[1] - @pos[1]
    raise "Diagonal moves not supported dx=#{dx} dy=#{dy} pos=#{pos} current_pos=#{@pos}" if dx !=0 && dy != 0

    return 1 if dy == -1
    return 2 if dy == 1
    return 3 if dx == -1
    return 4 if dx == 1

    raise "Wrong diff dx=#{dx} dy=#{dy}"
  end

  def dir_to_pos(dir)
    case dir
    when 1
      return [@pos[0], @pos[1] - 1]
    when 2
      return [@pos[0], @pos[1] + 1]
    when 3
      return [@pos[0] - 1, @pos[1]]
    when 4
      return [@pos[0] + 1, @pos[1]]
    end
  end

  def print_map
    #(@map.size).times { print "\r\e[A" }
    (0..@map.size-1).each do |y|
      row = @map[y]
      (0..row.size-1).each do |x|
        if [x,y] == @pos 
          putc 'D'
        elsif [x,y] == @oxygen_station
          putc 'S'
        else
          putc @map[y][x]
        end
      end
      puts
    end
  end
end

droid = Droid.new 'input_0'

droid.explore
droid.print_map

puts "Part 1: #{droid.find_best_path(Droid::START_POS, droid.oxygen_station).size}"

