def input filename = 'aoc_2018_01_input.txt'
  raise "no input?" unless File.file?(filename)
  entries = [] ; File.open(filename, 'r') do |file|
    while line = file.gets
      raise "malformed file?" if line.empty?
      entries << line.to_i
    end
  end
  entries
end

def final_frequency frequencies = input
  frequencies.inject(0, :+)
end

def detect_loop frequencies = input
  cycle = frequencies.cycle
  reached = [0]
  while next_step = reached.last + cycle.next
    return next_step if reached.include?(next_step)
    reached << next_step
  end
end
