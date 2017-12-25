def swap(str, idx1, idx2)
  chars = str.chars
  chars[idx1], chars[idx2] = chars[idx2], chars[idx1]    
  chars.join
end

lines = File.readlines("input_test.txt").reverse
password = 'decab'
puts password
lines.each do |line|
  match = line.match /swap position (\d) with position (\d)/
  if !match.nil?
    password = swap(password, match[1].to_i, match[2].to_i)
  else 

    match = line.match /swap letter (\w) with letter (\w)/
    if !match.nil?
      password = swap(password, password.index(match[1]), password.index(match[2]))
    else

      match = line.match /reverse positions (\d) through (\d)/
      if !match.nil?
        password[ match[1].to_i .. match[2].to_i ] = password[ match[1].to_i .. match[2].to_i ].reverse
      else

        match = line.match /rotate (\w+) (\d) step/
        if !match.nil?
          password = password.chars.rotate( (match[1] == 'right' ? -1 : 1) * match[2].to_i ).join
        else

          match = line.match /move position (\d) to position (\d)/
          if !match.nil?
            chars = password.chars
            chars.insert( match[2].to_i, chars.delete_at(match[1].to_i)) 
            password = chars.join
          else

            match = line.match /rotate based on position of letter (\w)/
            if !match.nil?
              idx = password.index(match[1])
              rotate_cnt = idx + 1 + (idx >= 4 ? 1 : 0)
              password = password.chars.rotate( -1 * rotate_cnt ).join
            else
              puts line
              raise "Err! Incorrent input or missing rule!"
            end

          end
        end
      end
    end
  end

end
puts password
