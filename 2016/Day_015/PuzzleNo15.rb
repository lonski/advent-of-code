class Disc
  attr_accessor :position, :position_count, :id
  
  def initialize(id, pos, pos_count)
    @id = id.to_i
    @position = pos.to_i
    @position_count = pos_count.to_i
  end

  def atSlotIn?(time_diff)
    movement = time_diff >= @position_count ? time_diff % @position_count 
                                            : time_diff
    new_pos = @position + movement
    new_pos = 0 if new_pos == @position_count

    new_pos == 0
  end
end

input = "Disc #1 has 17 positions; at time=0, it is at position 5.
Disc #2 has 19 positions; at time=0, it is at position 8.
Disc #3 has 7 positions; at time=0, it is at position 1.
Disc #4 has 13 positions; at time=0, it is at position 7.
Disc #5 has 5 positions; at time=0, it is at position 1.
Disc #6 has 3 positions; at time=0, it is at position 0."
#Disc #7 has 11 positions; at time=0, it is at position 0." #second part

discs = input.split("\n")
             .map{|l| l.match /#(\d+) has (\d+).*time=(\d+).*position\s(\d+)/ }
             .map{|m| Disc.new(m[1], m[4], m[2])}
time = 0

while !discs.select{|d| !d.atSlotIn?(time+d.id)}.empty?
  time += 1
end

puts time
