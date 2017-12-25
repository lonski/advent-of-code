def is_trap(triplet)
  return true if triplet[0] == '^' && triplet[1] == '^' && triplet[2] != '^'
  return true if triplet[0] != '^' && triplet[1] == '^' && triplet[2] == '^'
  return true if triplet[0] == '^' && triplet[1] != '^' && triplet[2] != '^'
  return true if triplet[0] != '^' && triplet[1] != '^' && triplet[2] == '^'
  return false
end

def extract_triplet(row, i)
  [
    i > 0 ? row[i-1] : '.',
    row[i],
    i < row.length - 1 ? row[i+1] : '.'
  ]
end

def generate_row(prev_row)
  new_row = ''

  prev_row.length.times do |i|
    new_row += is_trap(extract_triplet(prev_row, i)) ? '^' : '.'
  end

  new_row
end

def generate(initial_row, row_count)
  rows = [initial_row]
  (row_count-1).times { |i| rows.push( generate_row(rows[i]) ) }
  rows
end

def count_safe(rows)
  rows.join.count('.')
end

puts count_safe(
    generate(
      '.^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^',
      400000
    )
  )

### --- TESTS --- ###

def extract_triplet_test()
  raise 'Err' unless extract_triplet('..^^.', 0).join == '...'
  raise 'Err' unless extract_triplet('..^^.', 1).join == '..^'
  raise 'Err' unless extract_triplet('..^^.', 2).join == '.^^'
  raise 'Err' unless extract_triplet('..^^.', 4).join == '^..'
end

def count_safe_test()
  raise 'Err' unless count_safe(generate('..^^.',3)) == 6
  raise 'Err' unless count_safe(generate('.^^.^.^^^^',10)) == 38
end

extract_triplet_test
count_safe_test
