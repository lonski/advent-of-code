def distance(p1,p2)
  (p2[0] - p1[0]).abs + (p2[1] - p1[1]).abs
end

def same_angle(start, goal, angle)
    angle ==  Math.atan2(goal[1] - start[1], goal[0] - start[0])
end

def find_quarter_vectors(pos, objects)
  vectors = []
  to_skip = []
  objects.each do |x, y|
    next if to_skip.include? [x,y]
    angle = Math.atan2(y - pos[1], x - pos[0])
    vector_objs = objects
      .select{|o| same_angle(pos, o, angle)}
      .reject{|o| to_skip.include? o }
    vector_objs = vector_objs.sort_by{|o| distance(pos, o)}
    unless vector_objs.empty?
      to_skip += vector_objs
      vectors << [angle, vector_objs]
    end
  end

  puts to_skip.inspect
  vectors.sort_by{|angle, v| angle}
end

map = File.read('input_5').split("\n")
width = map[0].size
height = map.size
pos = [19,14]
map[pos[1]][pos[0]] = '@'

asteroids = (0..width-1)
  .map {|x| (0..height-1).map{|y| [x,y] }}
  .flatten(1)
  .select{|x,y| map[y][x] != '.' }
  .reject{|o| o == pos}

counter = 0
vectors = [
  find_quarter_vectors(pos, asteroids.select{|ox, oy| ox >= pos[0] && oy < pos[1]}), #QI
  find_quarter_vectors(pos, asteroids.select{|ox, oy| ox > pos[0] && oy >= pos[1]}), #QII
  find_quarter_vectors(pos, asteroids.select{|ox, oy| ox <= pos[0] && oy >= pos[1]}), #QIII
  find_quarter_vectors(pos, asteroids.select{|ox, oy| ox < pos[0] && oy <= pos[1]}) #QIV
].flatten(1)

while true
  vectors.each do |a,v|
    to_vaporize = v.shift 
    unless to_vaporize.nil?
      counter += 1
      puts "Vaporized #{to_vaporize.inspect} counter=#{counter}" 
      map[to_vaporize[1]][to_vaporize[0]] = '.'
      puts map
      height.times { print "\r\e[A" }
      sleep 0.01
      map[to_vaporize[1]][to_vaporize[0]] = '.'
      map[to_vaporize[1]][to_vaporize[0]] = '.'# alph[counter-1]
      if counter == 200
        puts "Vaporized #{to_vaporize.inspect} counter=#{counter}"
        exit 0
      end
    end
  end
end
