TRACE=false
DEBUG=false

def trace(msg)
  puts msg if TRACE
end

def debug(msg)
  puts msg if DEBUG
end

def float_equal?(f1,f2)
  (f2 - f1).abs <= 0.1
end

def distance(p1,p2)
  (p2[0] - p1[0]) + (p2[1] - p1[1])
end

def is_linear_fun?(a,b)
  a != Float::INFINITY && 
  a != -Float::INFINITY && 
  b != Float::INFINITY &&
  b != -Float::INFINITY &&
  !a.nan? &&
  !b.nan?
end

def on_line(a, b, start, goal)
  trace("\t\tONLINE a=#{a} b=#{b} s=#{start} e=#{goal}")
  if a == Float::INFINITY || a == -Float::INFINITY || b == Float::INFINITY || b == -Float::INFINITY || a.nan? || b.nan?
    dy = goal[1] - start[1]
    dx = goal[0] - start[0]
    trace("\t\tINF dy=#{dy} dx=#{dx}")
    return dx == 0 || dy == 0
  end
  trace("\t\tLINE a=#{a} b=#{b} Y=#{a*goal[0].to_f + b} goal[1]=#{goal[1]} ")
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

  map = File.read('input_3').split("\n")
  width = map[0].size
  height = map.size
  
  map_points = (0..width-1).map {|x| (0..height-1).map{|y| [x,y] }}.flatten(1)
  asteroids = map_points.select{|x,y| map[y][x] != '.' }
  counter = 0

  pos = [8,3]

  #get objects in quarter I
  q1_objects = asteroids
    .select{|ox, oy| ox >= pos[0] && oy < pos[1]}
    .reject{|o| o == pos}

  #for each dx in (width - pos.x)
  (0..(width-pos[0]-1)).each do |dx|

    #find objects on given dx, sorted by y desc
    obj_on_dx = q1_objects
      .select{|ox, oy| ox - pos[0] == dx}
      .sort_by{|ox, oy| oy}

    debug("dx=#{dx} objs=#{obj_on_dx.inspect}")

    #list of objects to skip due to be blocked
    to_skip = []

    #for each object of above find all objects on the line
    obj_on_dx.each do |x, y|
      debug("  checking [#{x},#{y}]")
      debug("  q1_objects=#{q1_objects.inspect}")
      debug("  to_skip=#{to_skip.inspect}")
      next if to_skip.include?([x,y])
      # calculate linear function factors
      dy = y - pos[1]
      dx = x - pos[0]
      a = dy.to_f / dx.to_f
      b = (pos[1]*x - y*pos[0]).to_f / (x - pos[0]).to_f

      #find all other objects that are on the same line
      on_line_objs = []
      if is_linear_fun?(a,b)
        on_line_objs = q1_objects.select{|o| r = on_line(a,b,[x,y],o); trace("\t\tONLINE RET = #{r}"); r}
      else
        #handle non linear fun, only for Q1
        on_line_objs = q1_objects.select{|ox, oy| ox == pos[0]}
      end

      #sort on_line objects by distance
      on_line_objs = on_line_objs.sort_by{|o| distance([x,y], o)}
      debug("   #{[x,y].inspect} line: #{on_line_objs.inspect}")

      #get closest object and vaporize
      unless on_line_objs.empty?
        to_vaporize = on_line_objs.first
        if asteroids.include? to_vaporize
          counter += 1
          asteroids = asteroids - [to_vaporize]
          to_skip += (on_line_objs - [to_vaporize])
          puts "Vaporized #{to_vaporize.inspect} counter=#{counter}"
        end
      end
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
