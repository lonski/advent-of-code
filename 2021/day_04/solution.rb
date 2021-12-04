def is_winner(board, numbers)
  board.any? { |row| (numbers & row).size == row.size } || board.transpose.any? { |column| (numbers & column).size == column.size }
end

def find_winner(boards, used, remaining)
  winner = boards.select{ |b| is_winner(b, used) }
  winner.empty? ? find_winner(boards, used << remaining.shift, remaining) : [winner, used, remaining]
end

def find_last_winner(boards, used, remaining)
  winner, used, remaining = find_winner(boards, used, remaining)
  boards.size == 1 ? [boards[0], used, remaining] : find_last_winner(boards - winner, used, remaining)
end

def calc_score(board, used_numbers)
  board.flatten.reject{ |n| used_numbers.include? n }.inject(&:+) * used_numbers[-1]
end

input = File.read(ARGV[0]).split("\n")
numbers = input.shift.split(',').map(&:to_i)
boards = input.reject(&:empty?)
              .each_slice(5)
              .map{ |board| board.map{|row| row.split(' ').map(&:to_i) } }

winner, used_numbers, * = find_winner(boards, [], numbers.dup)
puts "Part 1: #{calc_score(winner, used_numbers)}"

last_winner, used_numbers, * = find_last_winner(boards, [], numbers.dup)
puts "Part 2: #{calc_score(last_winner, used_numbers)}"

