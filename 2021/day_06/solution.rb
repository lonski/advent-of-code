def count_offspring(init_val, times)
  return 1 if init_val >= times
  first_offspring = times - init_val - 1
  return 1 + ((first_offspring / 7)+1).times.map { |n| count_offspring(8, first_offspring - n*7) }.sum
end

def brute_force_solution(input, count)
  threads = 6
  puts input.map{ |n| count_offspring(n, count) }.sum
  #input.each_slice(input.size / threads) do |slice|
  #  fork do
  #    slice.map { |n| puts count_offspring(n, times) }
  #  end
  #end
  #
  #Process.waitall
end

def map_solution(input, count)
  fishes = {}
  input.each do |n|
    fishes[n] ||= 0
    fishes[n] = fishes[n] + 1
  end

  count.times.inject(fishes) do |fishes, n| 
    new_fishes = {}
    fishes.keys.each do |fish|
      if fish == 0
        new_fishes[6] = (new_fishes[6] || 0) + fishes[0]
        new_fishes[8] = (new_fishes[8] || 0) + fishes[0]
      else
        new_fishes[fish - 1] = (new_fishes[fish - 1] || 0) + fishes[fish]
      end
    end
    new_fishes
  end.values.sum
end

input = File.read(ARGV[0]).split(",").map(&:to_i)
puts brute_force_solution(input, 80)
puts map_solution(input, 80)
puts map_solution(input, 256)
