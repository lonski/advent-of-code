class Claim 
  attr_accessor :id, :x, :y, :width, :height

  def initialize(str)
    @id, @x, @y, @width, @height = /#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/
      .match(str).captures.map(&:to_i)
  end

  def draw(grid, grid_size)
    each_point do |x, y|
      pos = x+y*grid_size
      grid[pos] = (grid[pos] == '.' ? '#' : 'X')
    end
  end

  def overlapped?(grid, grid_size)
    each_point{ |x, y| return true if grid[x+y*grid_size] == 'X' } && false
  end

  def each_point
    (@x..@x+@width-1).each{ |x| (@y..@y+@height-1).each{ |y| yield x, y } }
  end
end

grid_size = 2000
input_file = 'input.txt'

grid = Array.new(grid_size*grid_size, '.')
claims = File.readlines(input_file).map{ |s| Claim.new s }

claims.each{|c| c.draw(grid, grid_size) }
puts "Overlapped squares: #{grid.count('X')} "

not_overlapped = claims.find{ |c| !c.overlapped? grid, grid_size }
puts "Claim that is not overlapped: #{not_overlapped.id}"
