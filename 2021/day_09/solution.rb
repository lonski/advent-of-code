def neighbours(x, y, width, height)
  nb = []
  nb << [x-1, y] if x > 0
  nb << [x, y-1] if y > 0
  nb << [x+1, y] if x < (width - 1)
  nb << [x, y+1] if y < (height - 1)
  nb
end

map = File.read(ARGV[0]).split("\n").map{|l| l.chars.map(&:to_i) }

width = map[0].size
height = map.size

low_points = []
height.times do |y|
  width.times do |x|
    val = map[y][x]
    low_points << [[x,y],val] if neighbours(x, y, width, height).map{|x,y| map[y][x]}.all?{|n| n > val}
  end
end

basins = []
low_points.each do |low_point, val|
  basin = []
  visited = {}
  to_visit = [low_point]
  while !to_visit.empty?
    x,y = to_visit.shift
    visited[[x,y]] = true
    val = map[y][x]
    if val != 9
      to_visit += neighbours(x, y, width, height).reject{|x,y| visited[[x,y]] || map[y][x] == 9}
      basin << [x,y]
    end
  end
  basins << basin.uniq
end

puts "Part 1: #{low_points.map{|_,p| p + 1}.sum}"
puts "Part 2: #{basins.map(&:size).sort[-3..].inject(&:*)}"



