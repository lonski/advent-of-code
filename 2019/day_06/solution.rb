class CelestialBody
  attr_accessor :name, :parents, :children

  def initialize(name)
    @name = name
    @children = []
    @parents = []
  end

  def add_orbiting_object(o)
    @children << o 
    o.parents << self
  end

  def count_orbits(total, direct)
    return total + direct if @children.empty?
    total + direct + @children.map{|o| o.count_orbits(total, direct + 1)}.reduce(&:+)
  end

  def distance(goal)
    calc_distance(goal, 0, self).min - 2
  end

  def calc_distance(goal, n, came_from)
    return [n] if @name == goal
    return [] if @parents.empty?
    (@parents + @children - [came_from]).flat_map{|p| p.calc_distance(goal, n+1, self)}
  end
end

objects = {}

m = File.read('input.2').split("\n").map{|l| l.split ')'}.each do |l, r|
  objects[l] = CelestialBody.new(l) unless objects.key? l
  objects[r] = CelestialBody.new(r) unless objects.key? r
  objects[l].add_orbiting_object(objects[r])
end


puts "Part 1: #{objects['COM'].count_orbits(0,0)}"
puts "Part 2: #{objects['YOU'].distance('SAN')}"
