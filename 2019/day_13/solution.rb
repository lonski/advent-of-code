require_relative '../intcpu.rb'

class Board

  attr_accessor :tiles

  def initialize
    @tiles = {}
  end

  def set_tile(pos, id)
    @tiles[pos] = id
  end

  def get_tile(pos)
    @tiles[pos] || 0
  end

  def print
    map = []
    @tiles.each do |a, c|
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

class Game

  def initialize
    @pod = [0,0]
    @ball = [0,0]
    @board = Board.new
  end

  def count_blocks(program)
    cpu = IntCpu.new(program)
    cpu.run
    cpu.output.each_slice(3).select{|x,y,id| id == 2}.count
  end

  def play(program)
    cpu = IntCpu.new(program)
    cpu.set_mem(0,2)

    while true 
      cpu.run

      #collect output
      cpu.output.each_slice(3) do |x,y,val|
        point = [x, y]

        if point == [-1,0] 
          @score = val
        else
          @board.set_tile(point, val)
          case val
          when 3
            @pod = point
          when 4
            @ball_prev = @ball
            @ball = point
          end
        end
      end

      #set input
      input = 0 
      if @ball[0] > @pod[0]
        input  = 1
      elsif @ball[0] < @pod[0]
        input = -1
      end

      cpu.output = []
      cpu.input = input

      #Print game
      #26.times { print "\r\e[A" }
      #@board.print
      #puts "Score: #{@score}"
      #sleep 0.01

      break if @board.tiles.values.select{|v| v == 2}.empty?
    end

    @score
  end

end

program = File.read 'input_0'

puts "Part 1: #{Game.new.count_blocks(program)}"
puts "Part 2: #{Game.new.play(program)}"
