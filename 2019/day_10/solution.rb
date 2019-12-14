def float_equal?(f1,f2)
  (f2 - f1).abs <= 0.1
end

def distance(p1,p2)
  (p2[0] - p1[0]) + (p2[1] - p1[1])
end

def on_line(a, b, start, goal)
  #puts "#{start} #{goal}"
  if a == Float::INFINITY || a == -Float::INFINITY || b == Float::INFINITY || b == -Float::INFINITY || a.nan? || b.nan?
    dy = goal[1] - start[1]
    dx = goal[0] - start[0]
    return goal[1] == start[0] if dy == 0
    return goal[0] == start[0]
  end
  #puts "LINE a=#{a} b=#{b} Y=#{a*goal[0].to_f + b} goal[1]=#{goal[1]} "
  float_equal?(goal[1].to_f,  (a*goal[0].to_f + b))
end

map = File.read('input_0').split("\n")
width = map[0].size
height = map.size

map_points = (0..width-1).map {|x| (0..height-1).map{|y| [x,y] }}.flatten(1)
asteroids = map_points.select{|x,y| map[y][x] != '.' }

def translate(base, point)
  [point[0] - base[0], point[1] - base[1]]
end

def part2

  map = File.read('input_0').split("\n")
  width = map[0].size
  height = map.size
  
  map_points = (0..width-1).map {|x| (0..height-1).map{|y| [x,y] }}.flatten(1)
  asteroids = map_points.select{|x,y| map[y][x] != '.' }

  pos = [3,4]

  (0..360).each do |degrees|
    radians = degrees * Math::PI / 180
    a = Math.tan(radians)

    on_line_list = asteroids
      .reject{|ast_p| ast_p == pos}
      .select{|ast_p| on_line(a,0.0,pos,translate(pos, ast_p))}
      .map{|ast_p| [distance(pos,ast_p),ast_p]}
      .sort_by{|dist, ast_p| dist}
      #group_by{|dist, ast_p| dist < 0}
    
    unless on_line_list.empty?
      puts "deg=#{degrees} p=#{on_line_list.map{|dist, ast_p| ast_p}}"
    end

    o1 = on_line(a, 0.0, pos, translate(pos, [1,0]))
    o2 = on_line(a, 0.0, pos, translate(pos, [2,2]))

    if o1 && o2
      puts "d=#{degrees} o1=#{o1} o2=#{o2}"  
    end
  end
end

part2

def part1
  max_reachable = 0
  max_p = [0,0]

  asteroids.each do |pos|
  
    #pos = [1,2]
    reachable = []
    unreachable = []
    asteroids.each do |x,y|
      #x =2
      #y =1
    
      dy = y - pos[1]
      dx = x - pos[0]
    
      a = dy.to_f / dx.to_f
      b = (pos[1]*x - y*pos[0]).to_f / (x - pos[0]).to_f
  
      #puts "DDD"
      #puts on_line(a,b,pos, [8,6])
    
      on_line_list = asteroids
        .reject{|ast_p| ast_p == pos}
        .select{|ast_p| on_line(a,b,pos,ast_p)}
        .map{|ast_p| [distance(pos,ast_p),ast_p]}
        .sort_by{|dist, ast_p| dist}
        .group_by{|dist, ast_p| dist < 0}
  
  
      list_left = on_line_list[true] || []
      list_right = on_line_list[false] || []
  
      #puts "LL #{list_left.inspect}"
      #puts "LR #{list_right.inspect}"
  
      unless list_left.empty?
        reachable << list_left.pop[1]
        unreachable += list_left.map{|dist, ast_p| ast_p}
      end
  
      unless list_right.empty?
        reachable << list_right.shift[1]
        unreachable += list_right.map{|dist, ast_p| ast_p}
      end
  
      #break 
    end
    
    reachable.uniq!
    unreachable.uniq!
    #puts reachable.size
    if reachable.size > max_reachable
      max_reachable = reachable.size
      max_p = pos.dup
    end
  
  end
  
  puts
  puts max_reachable
  puts max_p.inspect

end

#puts "A: #{asteroids.inspect} -> #{asteroids.size}"
#puts
#puts "R: #{reachable.inspect} -> #{reachable.size}"
#puts
#puts "U: #{unreachable.inspect} -> #{unreachable.size}"
#puts
#puts "D: #{asteroids - reachable - unreachable}"
