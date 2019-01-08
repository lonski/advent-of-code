require 'time'

class Record
  attr_accessor :time, :message

  def initialize(str)
    time_str, @message = /\[(.*)\] (.*)/.match(str).captures
    @time = DateTime.parse(time_str)
  end
end

class Guard
  attr_accessor :id

  def initialize(id)
    @id = id
    @naps = []
  end

  def add_nap(nap_start, nap_end)
    @naps << [nap_start, nap_end]
  end

  def count_nap_time
    @naps
      .map{ |b, e| ((b-e) * 24 * 60).to_i.abs }
      .reduce(0, :+)
  end

  def count_days_with_nap_at(time)
    time = time.strftime('%H%M')
    @naps
      .map{ |b, e| [b.strftime('%H%M'), e.strftime('%H%M')] }
      .select{ |b, e| time >= b && time < e }
      .size
  end

  def most_asleep_minute
    most_asleep_minute = 60.times
      .map{ |m| Time.parse "00:#{m}" }
      .map{ |t| [t, count_days_with_nap_at(t)] }
      .sort_by{ |t, count| -count } 
      .map{ |t, count| [t.min, count] }
      .first
  end
end

guards = {}
current_guard = nil
nap_start = nil
File.readlines('input.txt')
  .map{ |l| Record.new l }
  .sort_by{ |r| r.time }
  .each do |r| 
  if r.message.start_with? 'Guard #'
    id = r.message.split[1][1..-1].to_i
    guards[id] ||= Guard.new id
    current_guard = guards[id]
  elsif r.message == 'falls asleep'
    nap_start = r.time 
  elsif r.message == 'wakes up'
    current_guard.add_nap nap_start, r.time
  end
end

puts "Part I: " + guards
  .map{ |id, g| [g, g.count_nap_time] }
  .sort_by{ |g, nap_time| -nap_time }
  .map{ |g, nap_time| g.id * g.most_asleep_minute[0] }
  .first.to_s

puts "Part II: " + guards
  .map{ |id, g| [g, g.most_asleep_minute] }
  .sort_by{ |g, m| -m[1] }
  .map{ |g, m| g.id * m[0] }
  .first.to_s
