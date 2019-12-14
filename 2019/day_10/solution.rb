TRACE=false
DEBUG=false

def trace(msg)
  puts msg if TRACE
end

def debug(msg)
  puts msg if DEBUG || TRACE
end

def float_equal?(f1,f2)
  (f2 - f1).abs <= 0.1
end

def distance(p1,p2)
  d = (p2[0] - p1[0]).abs + (p2[1] - p1[1]).abs
  trace("\t\tDISTANCE=#{d} p1=#{p1.inspect} p2=#{p2.inspect}")
  d
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

def translate(base, point)
  [point[0] - base[0], point[1] - base[1]]
end

def find_quarter_vectors(pos, objects, no)
  vectors = []
  to_skip = []
  objects.each do |x, y|
    next if to_skip.include? [x,y]
    # calculate linear function factors
    dy = y - pos[1]
    dx = x - pos[0]
    a = dy.to_f / dx.to_f
    b = (pos[1]*x - y*pos[0]).to_f / (x - pos[0]).to_f
    #find all other objects that are on the same line
    #puts "dy=#{dy} dx=#{dx}"
    vector_objs = []
    if is_linear_fun?(a,b)
      vector_objs = objects.select{|o| r = on_line(a,b,[x,y],o); trace("\t\tONLINE RET = #{r}"); r}
    else
      #handle non linear fun, for Q1 && Q3
      #if (dy <= 0 && dx >= 0) || (dy >= 0 && dx <= 0)
      if no == 1 || no == 3
        vector_objs = objects.select{|ox, oy| ox == pos[0]}
      else # Q2 && Q4
        vector_objs = objects.select{|ox, oy| oy == pos[1]}
      end
    end
    #sort by distance
    vector_objs = vector_objs.sort_by{|o| distance(pos, o)}
    unless vector_objs.empty?
      to_skip += vector_objs
      vectors << [a, vector_objs]
    end
  end

  vectors.sort_by{|a, v| a}
end

def part2

  map = File.read('input_4').split("\n")
  width = map[0].size
  height = map.size
  #pos = [11,13]
  #pos = [19,14]
  pos = [8,3]
  map[pos[1]][pos[0]] = '@'
  
  asteroids = (0..width-1)
    .map {|x| (0..height-1).map{|y| [x,y] }}
    .flatten(1)
    .select{|x,y| map[y][x] != '.' }
    .reject{|o| o == pos}

  counter = 0
  vectors = []

  #get objects in quarter I
  q1_objects = asteroids.select{|ox, oy| ox >= pos[0] && oy < pos[1]}
  vectors += find_quarter_vectors(pos, q1_objects,1)
  #puts vectors.inspect

  #get objects in quarter II
  q2_objects = asteroids.select{|ox, oy| ox > pos[0] && oy >= pos[1]}
  #q2_objects = asteroids.select{|ox, oy| ox >= pos[0] && oy >= pos[1]}
  vectors += find_quarter_vectors(pos, q2_objects,2)
  #puts vectors.inspect

  #get objects in quarter III
  q3_objects = asteroids.select{|ox, oy| ox <= pos[0] && oy > pos[1]}
  #q3_objects = asteroids.select{|ox, oy| ox < pos[0] && oy > pos[1]}
  vectors += find_quarter_vectors(pos, q3_objects,3)
  #puts vectors.inspect

  #get objects in quarter IV
  q4_objects = asteroids.select{|ox, oy| ox < pos[0] && oy <= pos[1]}
  vectors += find_quarter_vectors(pos, q4_objects,4)
  #puts vectors.inspect

  while true
    vectors.each do |a,v|
      to_vaporize = v.shift 
      unless to_vaporize.nil?
        counter += 1
        puts "Vaporized #{to_vaporize.inspect} counter=#{counter}"
        #print "\r\e[A" + "\r\e[A"
        
        map[to_vaporize[1]][to_vaporize[0]] = 'X'
        puts map
        height.times { print "\r\e[A" }
        sleep 0.01
        #raise '1' unless (counter == 1 && to_vaporize == [11,12]) || counter != 1
        #raise '2' unless (counter == 2 && to_vaporize == [12,1]) || counter != 2
        #raise '3' unless (counter == 3 && to_vaporize == [12,2]) || counter != 3
        #raise '10' unless (counter == 10 && to_vaporize == [12,8]) || counter != 10
        #raise '20' unless (counter == 20 && to_vaporize == [16,0]) || counter != 20
        #raise '50' unless (counter == 50 && to_vaporize == [16,9]) || counter != 50
        #raise '100' unless (counter == 100 && to_vaporize == [10,16]) || counter != 100
        #raise '199' unless (counter == 199 && to_vaporize == [9,6]) || counter != 199
        map[to_vaporize[1]][to_vaporize[0]] = '.'
        if counter == 200
          puts "Vaporized #{to_vaporize.inspect} counter=#{counter}"
          exit 0
        end
      end
    end
  end
  
end

def part1

  map = File.read('input_5').split("\n")
  width = map[0].size
  height = map.size
  
  map_points = (0..width-1).map {|x| (0..height-1).map{|y| [x,y] }}.flatten(1)
  asteroids = map_points.select{|x,y| map[y][x] != '.' }

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

part2
