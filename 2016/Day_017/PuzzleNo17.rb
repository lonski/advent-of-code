require 'digest'

def is_open(c)
  ['b','c','d','e','f'].include?(c)
end

def neighbours(pos, pass)
  md5 = Digest::MD5.hexdigest(pass)
  nb = []
  nb.push([ [pos[0], pos[1] - 1], 'U']) if pos[1] > 0 && is_open(md5[0]) #Up
  nb.push([ [pos[0], pos[1] + 1], 'D']) if pos[1] < 3 && is_open(md5[1]) #Down
  nb.push([ [pos[0] - 1, pos[1]], 'L']) if pos[0] > 0 && is_open(md5[2]) #Left
  nb.push([ [pos[0] + 1, pos[1]], 'R']) if pos[0] < 3 && is_open(md5[3]) #Right
  nb
end

start = [0,0]
goal = [3,3]
passcode = 'ioramepc'

shortest = ''
longest = 0

frontier = [ [start, passcode] ]
while !frontier.empty?
  (current_pos, current_pass) = frontier.pop 

  if current_pos == goal
    shortest = current_pass if shortest.empty? || current_pass.length < shortest.length
    longest = [current_pass.length, longest].max
  else
    neighbours(current_pos, current_pass).each do |nb|
      frontier.push([ nb[0], current_pass + nb[1] ])
    end
  end
end

shortest.slice! passcode
puts "Shortest path: #{shortest}"
puts "Longest path: #{longest-8}"
