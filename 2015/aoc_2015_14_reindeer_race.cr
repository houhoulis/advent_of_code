class Reindeer
  def initialize(@name : String, @speed : Int32, @go : Int32, @rest : Int32)
  end
  def name
    @name
  end

  def distance_in(duration : Int32)
    repeats = duration / (@go + @rest)
    leftover = duration % (@go + @rest)
    @speed * (repeats * @go + [leftover, @go].min)
  end
end

def reindeer(filename : String = "aoc_2015_14_input_1.txt")
  deer = [] of Reindeer
  File.open(filename, "r") do |file|
    while line = file.gets
      deer << deer_from line
    end
  end
  deer
end

def deer_from(line : String)
  name, speed, go, rest = line.chomp.split(" ").values_at(0,3,6,-2)
  Reindeer.new name, speed.to_i, go.to_i, rest.to_i
end

def winning_distance(duration : Int32)
  winner = reindeer.max_by { |deer| deer.distance_in duration }
  winning_distance = winner.distance_in duration
  puts "#{winner.name} went #{winning_distance} in #{duration} seconds."
  winning_distance
end

puts "#1: Winning distance: sample duration of 1000: #{winning_distance 1000}."
puts "#1: Winning distance: ANSWER for duration of 2503: #{winning_distance 2503}."

def running_count(duration : Int32)
  totals = reindeer.reduce({} of Reindeer => Int32) { |memo, deer| memo.merge!({deer => 0}) }

  1.upto(duration).each { |step|
    lead_distance = totals.keys.map { |deer| deer.distance_in step }.max
    leaders = totals.keys.select { |deer| deer.distance_in(step) == lead_distance }
    leaders.each { |deer| totals[deer] += 1 }
  }
  winning_count = totals.values.max
  winner = totals.keys.find { |deer| totals[deer] == winning_count }
  raise "wow!" if winner.nil?
  puts "#{winner.name} earned #{winning_count} in #{duration} seconds."
  winning_count
end

puts "#2: Largest running total: sample duration of 1000: #{running_count 1000}."
puts "#2: Largest running total: ANSWER for duration of 2503: #{running_count 2503}."
