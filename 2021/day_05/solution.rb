def parse(s, include_diagonals)
  p1, p2 = s.split(' -> ').map{ |p| p.split(',').map(&:to_i) }
  x1,y1 = p1
  x2,y2 = p2
  dx = x2 - x1
  dy = y2 - y1
  x_pts = x2 > x1 ? x2.downto(x1) : x2.upto(x1) 
  y_pts = y2 > y1 ? y2.downto(y1) : y2.upto(y1) 
  if dx != 0 && dy != 0
    include_diagonals ? x_pts.zip(y_pts) : []
  elsif (x2 - x1) != 0
    x_pts.map { |x|  [x, y1] }
  else
    y_pts.map { |y|  [x1, y] }
  end
end

def count_intersections(map)
  map.inject({}) do |map, points|
    points.each { |p| map[p] = (map[p] || 0) + 1 }
    map
  end.select{ |k, v| v > 1 }.size
end

input = File.read(ARGV[0]).split("\n")
puts "Part 1: #{count_intersections(input.map{|s| parse(s, false)})}"
puts "Part 2: #{count_intersections(input.map{|s| parse(s, true)})}"

