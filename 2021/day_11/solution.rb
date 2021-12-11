def neighbours(x, y, width, height)
  nb = []
  nb << [x-1, y] if x > 0
  nb << [x, y-1] if y > 0
  nb << [x+1, y] if x < (width - 1)
  nb << [x, y+1] if y < (height - 1)
  nb << [x+1, y+1] if y < (height - 1) && x < (width - 1)
  nb << [x-1, y+1] if y < (height - 1) && x > 0
  nb << [x-1, y-1] if y > 0 && x > 0
  nb << [x+1, y-1] if y > 0 && x < (width - 1)
  nb
end

def flash(x, y, map, flashed)
  height = map.size
  width = map[0].size
  return unless flashed[[x, y]].nil?

  val = map[y][x]
  if val < 9
    map[y][x] = val + 1
  else 
    map[y][x] = 0
    flashed[[x,y]] = true
    neighbours(x, y, width, height).each do |nb|
      flash(nb[0], nb[1], map, flashed)
    end
  end
end

def simulate(map)
  height = map.size
  width = map[0].size
  flashed = {}

  height.times do |y|
    width.times do |x|
      flash(x, y, map, flashed)
    end
  end

  flashed
end

def solve_p1
  map = File.read(ARGV[0]).split("\n").map{|l| l.chars.map(&:to_i) }
  flashes = 0
  100.times do |i|
    flashes += simulate(map).size
  end
  
  puts "Part 1: #{flashes}"
end

def solve_p2
  map = File.read(ARGV[0]).split("\n").map{|l| l.chars.map(&:to_i) }
  steps = 1
  octo_count = map[0].size * map.size
  while simulate(map).size != octo_count do
    steps += 1
  end
  
  puts "Part 2: #{steps}"
end

solve_p1
solve_p2
