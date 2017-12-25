class Node
  attr_accessor :x, :y, :size, :used, :avail, :used_percent, :id

  def initialize(match)
    if !match.nil?
      @x = match[1].to_i 
      @y = match[2].to_i
      @size = match[3].to_i
      @used = match[4].to_i
      @avail = match[5].to_i
      @used_percent = match[6].to_i
      @id = "#{@x}-#{@y}"
    end
  end
end

pattern = /node\-x(\d+)\-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)/

nodes = File.readlines("input.txt")
  .map{ |line| line.match pattern }
  .select{ |m| !m.nil? }
  .map{ |m| Node.new(m) }

viable = {}

nodes.each{ |n|  }

nodes.each do |a|
  if a.used > 0
    nodes.each do |b|
      if a.id != b.id
        if b.avail >= a.used 
          viable[a.id] ||= []
          viable[a.id] << b.id
        end
      end 
    end
  end
end

puts viable.reduce(0) { |sum, n| sum += n[1].length }
