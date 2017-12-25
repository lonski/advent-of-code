### --- PART 1 --- ###
def do_turn(elves)
  winners = elves.each_slice(2).map{ |v1,v2| v1 }
  elves.length.odd? ? winners[1..winners.length-1] : winners
end

def find_winner(elf_count)
  elves = 1.upto(elf_count).map { |i| i }
  while elves.length > 1 do
    elves = do_turn(elves)
  end
  elves[0]
end

#puts find_winner(3014387)

### --- PART 2 --- ###
def opposite(elves, i)
  idx = (elves.length.to_f / 2.to_f).ceil + i - 1
  puts "XXX idx=#{idx} i=#{i} len=#{elves.length}"
  idx = i - (elves.length - idx)  if idx >= elves.length 
  idx += 1 if elves.length.even?
  idx
end

elf_count = 6
elves = 1.upto(elf_count).map { |i| i }

i = 0
while elves.length > 1 do
  tmp = elves.dup
  elves.delete_at(opposite(elves,i))
  puts "#{tmp} -> Opposite index: #{opposite(tmp, i)} -> i=#{i} -> #{elves.inspect}"
  i += 1
  i = 0 if i >= elves.length 
end

puts elves.inspect
