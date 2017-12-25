def dragon_curve(a)
  b = a.reverse.chars.map{ |c| c == '1' ? '0' : '1' }.join
  "#{a}0#{b}"
end

def fill(input, length)
  out = input
  while out.length < length do
    out = dragon_curve(out)
  end
  out[0, length]
end

def checksum(str)
  begin 
    str = str.chars.each_slice(2).map{ |v1, v2| v2.nil? ? v1 : (v1==v2 ? '1' : '0') }.join
  end while (str.length % 2 == 0)
  str
end

puts checksum(fill('10001110011110000',35651584))

### --- TESTS --- ###

def dragon_curve_test
  raise "Err" unless dragon_curve('10000') == '10000011110'
  raise "Err" unless dragon_curve('10000011110') == '10000011110010000111110'
end

def fill_test
  raise "Err" unless fill('10000', 20) == '10000011110010000111'
end

def checksum_test
  raise 'Err' unless checksum('10000011110010000111') == '01100'
end

dragon_curve_test
fill_test
checksum_test
